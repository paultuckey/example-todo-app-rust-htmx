#[macro_use]
extern crate rocket;

use rocket::form::Form;
use rocket::http::Status;
use rocket_dyn_templates::{context, Metadata, Template};
use rocket_dyn_templates::handlebars::Handlebars;

use crate::db::{add_todo, clear_completed, DbError, get_todo, get_todos, maybe_create_database, toggle_todo_completed, update_todo};

mod db;

const DB_URL: &str = "sqlite://sqlite.db";

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    maybe_create_database().await.expect("Failed to create DB");

    let _rocket = rocket::build()
        .attach(Template::fairing())
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
        .await?;
    Ok(())
}

#[get("/")]
async fn get_index() -> Result<Template, Status> {
    let todos = get_todos().await?;
    Ok(Template::render(
        "index",
        context! {
            todos
        },
    ))
}

#[derive(FromForm)]
struct TodoForm {
    title: String,
}

#[post("/todos", data = "<form>")]
async fn post_todos(form: Form<TodoForm>) -> Result<Template, Status> {
    let id = add_todo(&form.title).await?;
    let todo = get_todo(id).await?;
    Ok(Template::render(
        "todo-read",
        context! {
            todo
        },
    ))
}

#[post("/todo-edit/<id>", data = "<form>")]
async fn post_todo_edit(id: i64, form: Form<TodoForm>) -> Result<Template, Status> {
    update_todo(id, &form.title).await?;
    let todo = get_todo(id).await?;
    Ok(Template::render(
        "todo-read",
        context! {
            todo
        },
    ))
}

#[get("/todo-edit/<id>")]
async fn get_todo_edit(id: i64) -> Result<Template, Status> {
    let todo = get_todo(id).await?;
    Ok(Template::render(
        "todo-edit",
        context! {
            todo
        },
    ))
}

#[get("/todo-read/<id>")]
async fn get_todo_read(id: i64) -> Result<Template, Status> {
    let todo = get_todo(id).await?;
    Ok(Template::render(
        "todo-read",
        context! {
            todo
        },
    ))
}

#[post("/todo-complete/<id>")]
async fn post_todo_complete(id: i64) -> Result<Template, Status> {
    toggle_todo_completed(id).await?;
    let todo = get_todo(id).await?;
    Ok(Template::render(
        "todo-read",
        context! {
            todo
        },
    ))
}

#[post("/todos-clear-completed")]
async fn post_todo_clear_completed() -> Result<Template, Status> {
    clear_completed().await?;
    let todos = get_todos().await?;
    Ok(Template::render(
        "todo-cards",
        context! {
            todos
        },
    ))
}

impl From<DbError> for Status {
    fn from(_: DbError) -> Self {
        Status::InternalServerError
    }
}

