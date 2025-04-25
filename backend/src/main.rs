use axum::{extract, routing::get, Router};
use std::time::SystemTime;

struct Note {
    user: String,
    title: String,
    content: String,
    time: SystemTime,
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello from rust!" }));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn create_note(extract::Json(payload): extract::Json<Note>) {}
