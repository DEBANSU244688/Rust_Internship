use std::sync::{Arc, RwLock};
use axum::{extract::State, routing::{get, post}, Router};

use crate::book::*;
use handler::*;

mod handler;
mod book;

type AppState = State<Arc<RwLock<Vec<Book>>>>;

#[tokio::main]
async fn main() {
    // Load books from CSV at server startup
    let books = load_books_from_csv().unwrap_or_else(|_| {
        eprintln!("⚠️ Failed to load books. Starting with empty list.");
        Vec::new()
    });

    // Shared state across routes using Arc + RwLock
    let shared_books = Arc::new(RwLock::new(books));

    // Route Setup
    let app = Router::new()
        .route("/books", get(list_books))
        .route("/books/new", post(add_book))
        .route("/books/search", get(search_book))
        .route("/books/{id}", get(get_book)
                            .put(update_book)
                            .delete(delete_book))
        .route("/ping", get(|| async {"📡 API is alive"}))
        .with_state(shared_books); // Sharing state with handlers

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
    .await
    .expect("❌ Failed to bind to port 3000");

    println!("🚀 Server running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}