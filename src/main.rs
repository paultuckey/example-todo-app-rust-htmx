mod db;

#[macro_use]
extern crate rocket;

use crate::db::{
    add_todo, clear_completed, get_todo, get_todos, maybe_create_database, toggle_todo_completed,
    update_todo,
};
use rocket::form::Form;
use rocket_dyn_templates::{context, Template};

const DB_URL: &str = "sqlite://sqlite.db";

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    maybe_create_database().await.expect("bbb");

    let _rocket = rocket::build()
        .attach(Template::fairing())
        // .attach(Template::try_custom(|engines| {
        //     let hbs = &mut engines.handlebars;
        //     Ok(())
        // }))
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
async fn get_index() -> Template {
    let todos = get_todos().await.expect("ccc");
    Template::render(
        "index",
        context! {
            todos
        },
    )
}

#[derive(FromForm)]
struct TodoForm {
    title: String,
}

#[post("/todos", data = "<form>")]
async fn post_todos(form: Form<TodoForm>) -> Template {
    let id = add_todo(&form.title).await.expect("aaa");
    let todo = get_todo(id).await.expect("ahhhh");
    Template::render(
        "todo-read",
        context! {
            todo
        },
    )
}

#[post("/todo-edit/<id>", data = "<form>")]
async fn post_todo_edit(id: i64, form: Form<TodoForm>) -> Template {
    update_todo(id, &form.title).await.expect("afgh");
    let todo = get_todo(id).await.expect("ahhhh");
    Template::render(
        "todo-read",
        context! {
            todo
        },
    )
}

#[get("/todo-edit/<id>")]
async fn get_todo_edit(id: i64) -> Template {
    let todo = get_todo(id).await.expect("ahhhh");
    Template::render(
        "todo-edit",
        context! {
            todo
        },
    )
}

#[get("/todo-read/<id>")]
async fn get_todo_read(id: i64) -> Template {
    let todo = get_todo(id).await.expect("ahhhh");
    Template::render(
        "todo-read",
        context! {
            todo
        },
    )
}

#[post("/todo-complete/<id>")]
async fn post_todo_complete(id: i64) -> Template {
    toggle_todo_completed(id).await.expect("ahhhh");
    let todo = get_todo(id).await.expect("ahhhh");
    Template::render(
        "todo-read",
        context! {
            todo
        },
    )
}

#[post("/todos-clear-completed")]
async fn post_todo_clear_completed() -> Template {
    clear_completed().await.expect("ahhhh");
    let todos = get_todos().await.expect("aaa");
    Template::render(
        "todo-cards",
        context! {
            todos
        },
    )
}
