use axum::{
    extract::State,
    http,
    http::Method,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Router,
};
use sqlx::SqlitePool;
use tower_http::cors::CorsLayer;

mod todo;
use todo::*;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(vec!["http://localhost:5173"
            .parse::<http::HeaderValue>()
            .unwrap()])
        .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(vec![http::header::CONTENT_TYPE]);
    let db = SqlitePool::connect("sqlite:todos.db").await.unwrap();
    let app = Router::new()
        .route("/", get(root))
        .route("/todos/all", post(Todo::get_all_todos))
        .route("/todos/create", post(Todo::create_todo))
        .route("/todos/update/status/:id", put(Todo::update_status))
        .route("/todos/update/title/:id", put(Todo::update_title))
        .route("/todos/update/content/:id", put(Todo::update_content))
        .route("/todos/delete/:id", delete(Todo::delete_todo))
        .with_state(db)
        .layer(cors);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root(State(_db): State<SqlitePool>) -> impl IntoResponse {
    "Hello from rust"
}
