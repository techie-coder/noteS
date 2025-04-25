use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, Row, SqlitePool};

#[derive(Serialize, Deserialize, FromRow)]
pub struct Todo {
    id: i32,
    title: String,
    content: Option<String>,
    status: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct CreateTodo {
    title: String,
    content: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Title {
    title: String,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Content {
    content: String,
}

async fn get_status(db: &SqlitePool, id: i32) -> Result<String, sqlx::Error> {
    let res = sqlx::query(
        "SELECT status from todos
        WHERE id = ?",
    )
    .bind(id)
    .fetch_one(db)
    .await?;

    Ok(res.get("status"))
}

impl Todo {
    pub async fn get_all_todos(State(db): State<SqlitePool>) -> impl IntoResponse {
        let res: Result<Vec<Self>, Error> =
            sqlx::query_as("SELECT * FROM todos").fetch_all(&db).await;

        match res {
            Ok(todos) => (StatusCode::OK, Json(todos)).into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }
    }
    pub async fn create_todo(
        State(db): State<SqlitePool>,
        Json(body): Json<CreateTodo>,
    ) -> impl IntoResponse {
        let res = sqlx::query("INSERT INTO todos (title, content, status) VALUES (?, ?, ?)")
            .bind(&body.title)
            .bind(&body.content)
            .bind("incomplete")
            .execute(&db)
            .await;

        match res {
            Ok(todo) => (
                StatusCode::CREATED,
                Json(Self {
                    id: todo.last_insert_rowid() as i32,
                    title: body.title.clone(),
                    content: Some(body.content.clone()),
                    status: "incomplete".to_string(),
                }),
            )
                .into_response(),
            Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
        }
    }
    pub async fn update_status(
        State(db): State<SqlitePool>,
        Path(id): Path<i32>,
    ) -> impl IntoResponse {
        let status = match get_status(&db, id.clone()).await {
            Ok(status) => status,
            Err(_) => return (StatusCode::NOT_FOUND, "Todo not found").into_response(),
        };

        let new_status = if status == "complete" {
            "incomplete"
        } else {
            "complete"
        };

        let res = sqlx::query(
            "UPDATE todos
            SET status = ?
            WHERE id = ?",
        )
        .bind(new_status)
        .bind(&id)
        .execute(&db)
        .await;
        match res {
            Ok(_) => (StatusCode::OK, "Todo updated".to_string()).into_response(),
            Err(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "error updating todo".to_string(),
            )
                .into_response(),
        }
    }
    pub async fn update_title(
        State(db): State<SqlitePool>,
        Path(id): Path<i32>,
        Json(body): Json<Title>,
    ) -> impl IntoResponse {
        let res = sqlx::query(
            "UPDATE todos
            SET title = ?
            WHERE id = ?",
        )
        .bind(&body.title)
        .bind(&id)
        .execute(&db)
        .await;
        match res {
            Ok(_) => (StatusCode::OK, "Todo updated".to_string()).into_response(),
            Err(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "error updating todo".to_string(),
            )
                .into_response(),
        }
    }
    pub async fn update_content(
        State(db): State<SqlitePool>,
        Path(id): Path<i32>,
        Json(body): Json<Content>,
    ) -> impl IntoResponse {
        let res = sqlx::query(
            "UPDATE todos
            SET content = ?
            WHERE id = ?",
        )
        .bind(&body.content)
        .bind(&id)
        .execute(&db)
        .await;
        match res {
            Ok(_) => (StatusCode::OK, "Todo updated".to_string()).into_response(),
            Err(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "error updating todo".to_string(),
            )
                .into_response(),
        }
    }
    pub async fn delete_todo(
        State(db): State<SqlitePool>,
        Path(id): Path<i32>,
    ) -> impl IntoResponse {
        let res = sqlx::query(
            "DELETE FROM todos
            WHERE id = ?
            ",
        )
        .bind(&id)
        .execute(&db)
        .await;
        match res {
            Ok(_) => (StatusCode::OK, "Todo deleted".to_string()).into_response(),
            Err(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "error deleting todo".to_string(),
            )
                .into_response(),
        }
    }
}
