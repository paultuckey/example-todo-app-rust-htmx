#[macro_use]
extern crate rocket;

use std::fmt::Display;

use rocket::form::Form;
use rocket::http::Status;
use rocket::response::content::RawHtml;

use crate::db::{
    DbError, add_todo, clear_completed, get_todo, get_todos, maybe_create_database,
    toggle_todo_completed, update_todo,
};

mod db;

mod templates {
    typed_handlebars::directory!("templates/");
}

const DB_URL: &str = "sqlite://sqlite.db";

#[rocket::main]
async fn main() -> Result<(), DbError> {
    maybe_create_database().await?;

    let _ = rocket::build()
        .mount(
            "/",
            routes![
                get_index,
                post_todos,
                get_todo_read,
                get_todo_edit,
                post_todo_edit,
                post_todo_complete,
                post_todo_clear_completed
            ],
        )
        .launch()
        .await;
    Ok(())
}

#[get("/")]
async fn get_index() -> Result<RawHtml<String>, Status> {
    let todos = get_todos().await?;
    Ok(html(templates::index(
        todos
            .iter()
            .map(|todo| templates::index::TodosItem::new(todo.id, todo.completed, &todo.title))
            .collect::<Vec<_>>(),
    )))
}

#[derive(FromForm)]
struct TodoForm {
    title: String,
}

#[post("/todos", data = "<form>")]
async fn post_todos(form: Form<TodoForm>) -> Result<RawHtml<String>, Status> {
    let id = add_todo(&form.title).await?;
    let todo = get_todo(id).await?;
    Ok(html(templates::todo_read(templates::todo_read::Todo::new(
        todo.id,
        todo.completed,
        &todo.title,
    ))))
}

#[post("/todo-edit/<id>", data = "<form>")]
async fn post_todo_edit(id: i64, form: Form<TodoForm>) -> Result<RawHtml<String>, Status> {
    update_todo(id, &form.title).await?;
    let todo = get_todo(id).await?;
    Ok(html(templates::todo_read(templates::todo_read::Todo::new(
        todo.id,
        todo.completed,
        &todo.title,
    ))))
}

#[get("/todo-edit/<id>")]
async fn get_todo_edit(id: i64) -> Result<RawHtml<String>, Status> {
    let todo = get_todo(id).await?;
    Ok(html(templates::todo_edit(templates::todo_edit::Todo::new(
        todo.id,
        &todo.title,
    ))))
}

#[get("/todo-read/<id>")]
async fn get_todo_read(id: i64) -> Result<RawHtml<String>, Status> {
    let todo = get_todo(id).await?;
    Ok(html(templates::todo_read(templates::todo_read::Todo::new(
        todo.id,
        todo.completed,
        &todo.title,
    ))))
}

#[post("/todo-complete/<id>")]
async fn post_todo_complete(id: i64) -> Result<RawHtml<String>, Status> {
    toggle_todo_completed(id).await?;
    let todo = get_todo(id).await?;
    Ok(html(templates::todo_read(templates::todo_read::Todo::new(
        todo.id,
        todo.completed,
        &todo.title,
    ))))
}

#[post("/todos-clear-completed")]
async fn post_todo_clear_completed() -> Result<RawHtml<String>, Status> {
    clear_completed().await?;
    let todos = get_todos().await?;
    let todos = todos
        .iter()
        .map(|todo| templates::todo_cards::TodosItem::new(todo.id, todo.completed, &todo.title))
        .collect::<Vec<_>>();
    Ok(html(templates::todo_cards(todos)))
}

// Helper function to map DB errors into HTTP errors
impl From<DbError> for Status {
    fn from(_: DbError) -> Self {
        Status::InternalServerError
    }
}

// Helper function to render templates
fn html(template: impl Display) -> RawHtml<String> {
    RawHtml(template.to_string())
}