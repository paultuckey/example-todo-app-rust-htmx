use crate::db::{
    DbError, add_todo, clear_completed, get_todo, get_todos, maybe_create_database,
    toggle_todo_completed, update_todo,
};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::Html;
use axum::routing::post;
use axum::{Form, Router, routing::get};
use serde::Deserialize;
use std::error::Error;
use std::fmt::Display;
use tracing::info;

mod db;

mod templates {
    typed_handlebars::directory!("templates/");
}

const DB_URL: &str = "sqlite://sqlite.db";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    maybe_create_database().await?;

    let app = Router::new()
        .route("/", get(get_index))
        .route("/todos", post(post_todos))
        .route("/todo-edit/{id}", get(get_todo_edit).post(post_todo_edit))
        .route("/todo-read/{id}", get(get_todo_read))
        .route("/todo-complete/{id}", post(post_todo_complete))
        .route("/todos-clear-completed", post(post_todo_clear_completed));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    info!("listening on http://{}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}

async fn get_index() -> Result<Html<String>, StatusCode> {
    let todos = get_todos().await?;
    ok(templates::index(
        todos
            .iter()
            .map(|todo| templates::index::TodosItem::new(todo.id, todo.completed, &todo.title))
            .collect::<Vec<_>>(),
    ))
}

#[derive(Deserialize, Debug)]
struct TodoForm {
    title: String,
}

async fn post_todos(Form(form): Form<TodoForm>) -> Result<Html<String>, StatusCode> {
    let id = add_todo(&form.title).await?;
    let todo = get_todo(id).await?;
    ok(templates::todo_read(templates::todo_read::Todo::new(
        todo.id,
        todo.completed,
        &todo.title,
    )))
}

async fn post_todo_edit(
    Path(id): Path<i64>,
    Form(form): Form<TodoForm>,
) -> Result<Html<String>, StatusCode> {
    update_todo(id, &form.title).await?;
    let todo = get_todo(id).await?;
    ok(templates::todo_read(templates::todo_read::Todo::new(
        todo.id,
        todo.completed,
        &todo.title,
    )))
}

async fn get_todo_edit(Path(id): Path<i64>) -> Result<Html<String>, StatusCode> {
    let todo = get_todo(id).await?;
    ok(templates::todo_edit(templates::todo_edit::Todo::new(
        todo.id,
        &todo.title,
    )))
}

async fn get_todo_read(Path(id): Path<i64>) -> Result<Html<String>, StatusCode> {
    let todo = get_todo(id).await?;
    ok(templates::todo_read(templates::todo_read::Todo::new(
        todo.id,
        todo.completed,
        &todo.title,
    )))
}

async fn post_todo_complete(Path(id): Path<i64>) -> Result<Html<String>, StatusCode> {
    toggle_todo_completed(id).await?;
    let todo = get_todo(id).await?;
    ok(templates::todo_read(templates::todo_read::Todo::new(
        todo.id,
        todo.completed,
        &todo.title,
    )))
}

async fn post_todo_clear_completed() -> Result<Html<String>, StatusCode> {
    clear_completed().await?;
    let todos = get_todos().await?;
    ok(templates::todo_cards(
        todos
            .iter()
            .map(|todo| templates::todo_cards::TodosItem::new(todo.id, todo.completed, &todo.title))
            .collect::<Vec<_>>(),
    ))
}

// Helper function to map DB errors into HTTP errors
impl From<DbError> for StatusCode {
    fn from(_: DbError) -> Self {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

// Helper function to render templates
fn ok(template: impl Display) -> Result<Html<String>, StatusCode> {
    Ok(Html(template.to_string()))
}
