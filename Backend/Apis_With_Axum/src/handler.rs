use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response:: {IntoResponse, Json}
};
use serde::{Deserialize, Serialize};

use crate::{book::*, AppState};

#[derive(Debug, Deserialize)]
pub struct SearchParams {
    pub title: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateBook {
    pub title: Option<String>,
    pub author: Option<String>,
}

// List all books
pub async fn list_books(State(books): AppState) -> Json<Vec<Book>> {
    let books_reader = books.read().unwrap();
    Json(books_reader.clone())
}

// Get a specific book by ID
pub async fn get_book(Path(id): Path<u32>, State(books): AppState) -> impl IntoResponse {
    let books_reader = books.read().unwrap();

    match books_reader.iter().find(|book| book.id == id) {
        Some(book) => Json(book).into_response(),
        None => (StatusCode::NOT_FOUND, "❌ Book Not Found").into_response(),
    }
}

// Add a new book
pub async fn add_book(State(books): AppState, Json(new_book): Json<CreateBook>) -> impl IntoResponse {
    if new_book.title.is_none() || new_book.author.is_none() {
        return (StatusCode::BAD_REQUEST, "🚫 Title & Author Required").into_response()
    }

    let mut books_writer = books.write().unwrap();
    let new_book_id = books_writer.iter().map(|book| book.id).max().unwrap_or(0) + 1;

    let book = Book {
        id: new_book_id,
        title: new_book.title.unwrap(),
        author: new_book.author.unwrap(),
    };
    books_writer.push(book.clone());

    save_books_to_csv(&books_writer).unwrap();
    (StatusCode::CREATED, Json(book)).into_response()
}

// Update an existing book
pub async fn update_book(
    Path(id): Path<u32>,
    State(books): AppState,
    Json(updated): Json<CreateBook>,
) -> impl IntoResponse {
    if updated.title.is_none() || updated.author.is_none() {
        return (StatusCode::BAD_REQUEST, "🚫 Title & Author Required").into_response()
    }

    let mut books_writer = books.write().unwrap();
    let mut found = false;

    for book in books_writer.iter_mut() {
        if book.id == id {
            book.title = updated.title.unwrap();
            book.author = updated.author.unwrap();
            found = true;
            break;
        }
    }

    if found {
        save_books_to_csv(&books_writer).unwrap();
        (StatusCode::OK, "✅ Book Updated").into_response()
    } else {
        (StatusCode::NOT_FOUND, "❌ Book Not Found").into_response()
    }
}

// Delete a book
pub async fn delete_book(Path(id): Path<u32>, State(books): AppState) -> impl IntoResponse {
    let mut books_writer = books.write().unwrap();
    let len_before = books_writer.len();
    books_writer.retain(|book| book.id != id);

    if books_writer.len() < len_before {
        save_books_to_csv(&books_writer).unwrap();
        (StatusCode::OK, "🗑️ Book Deleted").into_response()
    } else {
        (StatusCode::NOT_FOUND, "❌ Book Not Found").into_response()
    }
}

// Search books by title
pub async fn search_book(Query(params): Query<SearchParams>, State(books): AppState) -> Json<Vec<Book>> {
    let books_reader = books.read().unwrap();
    
    let filtered_books = books_reader.iter().filter(|book| {
        if let Some(title) = &params.title {
            book.title.to_lowercase().contains(&title.to_lowercase())
        } else {
            true
        }
    })
    .cloned().collect();

    Json(filtered_books)
}