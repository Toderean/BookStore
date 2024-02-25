use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub genre: String,
    pub quantity: u32,
    pub price: u32,
}

impl Book {
    pub fn new(title: String, author: String, genre: String, quantity: u32, price: u32) -> Book {
        Book {
            title,
            author,
            genre,
            quantity,
            price,
        }
    }
}

impl std::fmt::Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Title: {}, Author: {}, Genre: {}, Quantity: {}, Price: {},",
            &self.title, &self.author, &self.genre, &self.quantity, &self.price
        )
    }
}
