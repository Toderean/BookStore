use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fs::OpenOptions,
    io::{Seek, Write},
};

use crate::{book::Book, consts::PATH};

#[derive(Serialize, Deserialize, Debug)]
pub struct Inventory {
    pub books: Vec<Book>,
}

impl Inventory {
    pub fn add(&mut self, b: Book) -> Result<(), Box<dyn Error>> {
        let max_capacity = 30;
        if self.books.len() < max_capacity {
            self.books.push(b);
            Ok(())
        } else {
            Err("Cannot add any more books".into())
        }
    }

    pub fn remove(&mut self, b: usize) {
        self.books.remove(b);
    }

    pub fn update(
        &mut self,
        index: usize,
        field: String,
        update: String,
    ) -> Result<(), Box<dyn Error>> {
        let book = match self.books.get_mut(index) {
            None => return Err("None existing book".into()),
            Some(b) => b,
        };
        match field.as_str() {
            "title" => book.title = update,
            "author" => book.author = update,
            "genre" => book.genre = update,
            "price" => {
                if let Ok(price) = update.parse::<u32>() {
                    book.price = price;
                } else {
                    return Err("Invalid price format".into());
                }
            }
            "quantity" => {
                if let Ok(quantity) = update.parse::<u32>() {
                    book.quantity = quantity;
                } else {
                    return Err("Invalid quantity format".into());
                }
            }
            _ => return Err("Invalid field".into()),
        }

        Ok(())
    }

    pub fn display(
        &mut self,
        criteria: &str,
        book_position: Option<usize>,
    ) -> Result<(), Box<dyn Error>> {
        match criteria {
            "author" => {
                println!("Books are sorted by author:\n");
                self.books.sort_by_key(|x| x.author.clone());
                for book in &self.books {
                    println!("{book}");
                }
                println!();
                Ok(())
            }
            "title" => {
                println!("Books are sorted by title:\n");
                self.books.sort_by_key(|x| x.title.clone());
                for book in &self.books {
                    println!("{book}");
                }
                println!();
                Ok(())
            }
            "genre" => {
                println!("Books are sorted by genre:\n");
                self.books.sort_by_key(|x| x.genre.clone());
                for book in &self.books {
                    println!("{book}");
                }
                println!();
                Ok(())
            }
            "quantity" => {
                println!("Books are sorted by quantity:\n");
                self.books.sort_by_key(|x| x.quantity.clone());
                for book in &self.books {
                    println!("{book}");
                }
                println!();
                Ok(())
            }
            "price" => {
                println!("Books are sorted by price:\n");
                self.books.sort_by_key(|x| x.price.clone());
                for book in &self.books {
                    println!("{book}");
                }
                println!();
                Ok(())
            }
            "all" => {
                println!("Books in the inventory:\n");
                for book in &self.books {
                    println!("{book}");
                }
                println!();
                Ok(())
            }
            "book" => {
                let book_position = book_position.unwrap_or_default();
                println!("Specific book:\n");
                println!("{}", &self.books[book_position]);
                Ok(())
            }
            _ => Err("Invalid criteria".into()),
        }
    }

    pub fn sell_book(
        &mut self,
        book_index: usize,
        sell_quantity: u32,
    ) -> Result<(), Box<dyn Error>> {
        if &sell_quantity == &self.books[book_index].quantity {
            self.remove(book_index)
        } else {
            self.books[book_index].quantity -= sell_quantity;
        }
        Ok(())
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let mut path = OpenOptions::new().write(true).open(PATH)?;
        let serialized = serde_json::to_string(&self)?;

        path.seek(std::io::SeekFrom::Start(0))?;
        path.set_len(0)?;
        path.write_all(&serialized.as_bytes())?;

        Ok(())
    }

    pub fn get_index(&self, book: &str) -> Option<usize> {
        self.books
            .iter()
            .position(|x| x.title.to_lowercase() == book.to_lowercase())
    }
}
