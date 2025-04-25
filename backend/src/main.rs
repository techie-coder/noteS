use axum::{
    extract::State, response::IntoResponse, routing::delete, routing::get, routing::post,
    routing::put, Router,
};
use sqlx::SqlitePool;

mod todo;
use todo::*;

#[tokio::main]
async fn main() {
    let db = SqlitePool::connect("sqlite:todos.db").await.unwrap();
    let app = Router::new()
        .route("/", get(root))
        .route("/todos/all", get(Todo::get_all_todos))
        .route("/todos/create", post(Todo::create_todo))
        .route("/todos/update/status/:id", put(Todo::update_status))
        .route("/todos/update/title/:id", put(Todo::update_title))
        .route("/todos/update/content/:id", put(Todo::update_content))
        .route("/todos/delete/:id", delete(Todo::delete_todo))
        .with_state(db);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root(State(_db): State<SqlitePool>) -> impl IntoResponse {
    "Hello from rust"
}
