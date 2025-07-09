use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Book {
    pub id: u32,
    pub title: String,
    pub author: String,
}

pub fn load_books_from_csv() -> Result<Vec<Book>, csv::Error> {
    let mut reader = csv::Reader::from_path("assets/books.csv")?;
    let mut books = Vec::new();

    for record in reader.deserialize() {
        let book: Book = record?;
        books.push(book);
    }

    Ok(books)
}

pub fn save_books_to_csv(books: &[Book]) -> Result<(), csv::Error> {
    let mut writer = csv::Writer::from_path("assets/books.csv")?;

    for book in books {
        writer.serialize(book)?;
    }

    writer.flush()?;
    Ok(())
}