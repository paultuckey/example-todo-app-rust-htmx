use crate::DB_URL;
use serde::Serialize;
use sqlx::migrate::MigrateDatabase;
use sqlx::{Error, Pool, Sqlite, SqlitePool};

#[derive(Serialize)]
pub struct Todo {
    id: i64,
    title: String,
    completed: bool,
}

async fn conn() -> Result<Pool<Sqlite>, sqlx::Error> {
    SqlitePool::connect(DB_URL).await
}

pub async fn maybe_create_database() -> Result<(), Error> {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        info!("Creating database {}", DB_URL);
        Sqlite::create_database(DB_URL).await?
    } else {
        info!("Database already exists");
    }
    sqlx::query(
        "
    CREATE TABLE IF NOT EXISTS todos (
       id INTEGER PRIMARY KEY,
       title TEXT NOT NULL,
       completed INTEGER
    )
",
    )
        .execute(&conn().await?)
        .await?;
    Ok(())
}

pub async fn add_todo(title: &String) -> Result<i64, DbError> {
    let res = sqlx::query("INSERT INTO todos (title, completed) VALUES (?, 0)")
        .bind(title)
        .execute(&conn().await?)
        .await?;
    info!("Todo added with id {:?}", res.last_insert_rowid());
    Ok(res.last_insert_rowid())
}

pub async fn get_todo(id: i64) -> Result<Todo, DbError> {
    let row: (i64, String, i8) =
        sqlx::query_as("SELECT id, title, completed FROM todos WHERE id=?")
            .bind(id)
            .fetch_one(&conn().await?)
            .await?;
    Ok(Todo {
        id: row.0,
        title: row.1,
        completed: row.2 == 1,
    })
}

pub async fn update_todo(id: i64, title: &String) -> Result<(), DbError> {
    sqlx::query(
        "UPDATE todos SET title = ? WHERE id=?",
    )
        .bind(title)
        .bind(id)
        .execute(&conn().await?)
        .await?;
    Ok(())
}

pub async fn toggle_todo_completed(id: i64) -> Result<(), DbError> {
    sqlx::query(
        "UPDATE todos SET completed = \
        CASE WHEN completed = 1 THEN 0 \
        ELSE 1
    END
    WHERE id=?",
    )
        .bind(id)
        .execute(&conn().await?)
        .await?;
    Ok(())
}

pub async fn clear_completed() -> Result<(), DbError> {
    sqlx::query("DELETE FROM todos where completed = 1")
        .execute(&conn().await?)
        .await?;
    Ok(())
}

pub async fn get_todos() -> Result<Vec<Todo>, DbError> {
    let rows: Vec<(i64, String, i8)> = sqlx::query_as("SELECT id, title, completed FROM todos ORDER BY id DESC")
        .fetch_all(&conn().await?)
        .await?;
    Ok(rows
        .iter()
        .map(|row| Todo {
            id: row.0,
            title: row.1.clone(),
            completed: row.2 == 1,
        })
        .collect::<Vec<Todo>>())
}


pub struct DbError;

impl From<Error> for DbError {
    fn from(_: Error) -> Self {
        DbError
    }
}
