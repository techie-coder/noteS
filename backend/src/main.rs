use axum::{
    extract::{Path, State},
    routing::get,
    server::Server,
    Json, Router,
};

use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::net::SocketAddr;

#[derive(Clone)]
struct AppState {
    db: SqlitePool,
}

#[derive(Serialize, Deserialize)]
struct Note {
    id: Option<i64>,
    title: String,
    content: String,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let db = SqlitePool::connect(&env::var("DATABASE_URL")?).await;
    let app = Router::new()
        .route("/notes", get(list_notes).post(add_note))
        .route("/notes/:id", get(get_note))
        .with_state(AppState { db });

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("Server running on http://{}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn list_notes(State(state): State<AppState>) -> Json<Vec<Note>> {
    let notes = sqlx::query_as!(Note, "SELECT id, title, content FROM notes")
        .fetch_all(&state.db)
        .await
        .unwrap();
    Json(notes)
}

async fn get_note(Path(id): Path<i64>, State(state): State<AppState>) -> Json<Note> {
    let note = sqlx::query_as!(
        Note,
        "SELECT id, title, content from notes WHERE id = ?",
        id
    )
    .fetch_one(&state.db)
    .await
    .unwrap();
    Json(note)
}

async fn add_note(State(state): State<AppState>, Json(note): Json<Note>) -> Json<Note> {
    let result = sqlx::query!(
        "INSERT INTO notes (title, content) VALUES (?, ?)",
        note.title,
        note.content
    )
    .execute(&state.db)
    .await
    .unwrap();

    let id = result.last_insert_rowid();
    Json(Note {
        id: Some(id),
        ..note
    })
}
