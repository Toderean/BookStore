use std::fs::{self, OpenOptions,File};
use std::{error::Error, io::Read};
use serde::{Serialize, Deserialize};
use std::io::{self, Seek, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct Book{
    title: String,
    author: String,
    genre: String,
    quantity: u32,
    price: u32,
}

impl Book{
    fn new(title: String, author: String, genre: String, quantity: u32,price:u32)-> Book{
        Book{
            title,
            author,
            genre,
            quantity,
            price,
            }
    }
}


#[derive(Serialize, Deserialize, Debug)]
struct Inventory{
    books: Vec<Book>,
}

impl Inventory{
    fn add(&mut self, b: Book) -> Result<(), Box<dyn Error>> {
        let max_capacity = 30; 
        if self.books.len() < max_capacity {
            self.books.push(b);
            autosave( self)?;
            Ok(())
        } else {
            autosave( self)?;
            Err("Cannot add any more books".into())
        }
    }
    
    fn remove(&mut self, b: Book) -> Result<(), Box<dyn Error>> {
        if let Some(index) = self.books.iter().position(|x| x.title.to_lowercase() == b.title && x.author.to_lowercase() == b.author && x.genre.to_lowercase() == b.genre && x.quantity == b.quantity) {
            self.books.remove(index);
            autosave( self)?;
            Ok(())
        } else {
            autosave( self)?;
            Err(format!("Cannot find book {}", b.title).into())
        }
    }

    fn update(&mut self, b: Book) -> Result<(), Box<dyn Error>>{
        if let Some(index) = self.books.iter().position(|x| x.title.to_lowercase() == b.title) {
            self.books[index] = b;
            autosave( self)?;
            Ok(())
        }else{
            autosave( self)?;
            Err(format!("Cannot update book {}", b.title).into())
        }
    }

    fn display(&mut self, criteria: &str) -> Result<(), Box<dyn Error>>{
        match criteria {
            "author" =>{ 
                println!("Books are sorted by author:");
                self.books.sort_by_key(|x| x.author.clone());
                for books in &self.books{
                    println!("Title: {}, Author: {}, Genre: {}, Quantity: {}, Price: {},",books.title,books.author,books.genre,books.quantity,books.price);
                }
                Ok(())
                    },
            "title" => { 
                println!("Books are sorted by title:");
                self.books.sort_by_key(|x| x.title.clone());
                for books in &self.books{
                    println!("Title: {}, Author: {}, Genre: {}, Quantity: {}, Price: {},",books.title,books.author,books.genre,books.quantity,books.price);
                }
                Ok(())
                    },
            "gendre" =>{ 
                println!("Books are sorted by genre:");
                self.books.sort_by_key(|x| x.genre.clone());
                for books in &self.books{
                    println!("Title: {}, Author: {}, Genre: {}, Quantity: {}, Price: {},",books.title,books.author,books.genre,books.quantity,books.price);
                }
                Ok(())
                    },
            "quantity" => { 
                println!("Books are sorted by quantity:");
                self.books.sort_by_key(|x| x.quantity.clone());
                for books in &self.books{
                    println!("Title: {}, Author: {}, Genre: {}, Quantity: {}, Price: {},",books.title,books.author,books.genre,books.quantity,books.price);
                }
                Ok(())
                    },
            "price" => { 
                println!("Books are sorted by price:");
                self.books.sort_by_key(|x| x.price.clone());
                for books in &self.books{
                    println!("Title: {}, Author: {}, Genre: {}, Quantity: {}, Price: {},",books.title,books.author,books.genre,books.quantity,books.price);
                }
                Ok(())
                    },
            "default" => {
                println!("Books in the inventory:");
                for books in &self.books {
                    println!("Title: {}, Author: {}, Genre: {}, Quantity: {}, Price: {},",books.title,books.author,books.genre,books.quantity,books.price);
                }
                Ok(())
            },
            _ => Err("Invalid criteria".into())
        }
    }

    fn sell_book(&mut self, mut b: Book, sell_quantity: u32)-> Result<(),Box<dyn Error>>{
        if let Some(index) = self.books.iter().position(|x| x.title.to_lowercase() == b.title) {
            if b.quantity >= sell_quantity {
                b.quantity = b.quantity - sell_quantity;
                println!("Selled at {}",sell_quantity * b.price);
                self.update(b)?;
                autosave( self)?;
                Ok(())
            }else{
                Err(format!("Not enough quantity to sell! We have only {} pieces",b.quantity).into())
            }
        }else{
            autosave( self)?;
            Err(format!("Cannot update book {}", b.title).into())
        }
    }
}

fn print_help(){
    println!("Instructions: ");
    println!("1.To display all the books just enter: (display)");
    println!("  If you want to display the books ordered by some criteria just enter the following keys:");
    println!("  -price :sort by price");
    println!("  -quantity :sort by the quantities");
    println!("  -genre: sort by genre");
    println!("  -author: sort by authors");
    println!("  -title: sort by titles");
    println!("2.If you want to add a new book in inventory just enter keyword (add) followed by the necessary fields (title,author,genre,quantity,price).");
    println!("3.If you want to delete a book from the inventory just enter the keyword (remove) followed by the name of the book you want to delete.");
    println!("4.If you want to update a book you'll need to enter the keyword (update) followd by the title in the and the new fields updated (title,author,genre,quantity,price).");
    println!("5.To sell a book you'll need to enter keyword (sell) followed by fields of a book (title,author,genre,quantity,price) and finally insert the quantity you want to sell.")
}


fn write_to_file(file_path: &Path) -> Result<(),Box<dyn Error>>{
    
    let mut path = OpenOptions::new().write(true).open(file_path)?;
    
    let book1 =Book::new("Book1".to_string(),"Author1".to_string(), "Genre1".to_string(),11, 11);
    let book2 =Book::new("Book3".to_string(),"Author2".to_string(), "Genre2".to_string(),10, 10);
    
    let mut invent = Inventory {books: Vec::new()};
    invent.add(book1)?;
    invent.add(book2)?;

    let serialized = serde_json::to_string(&invent).unwrap();

    path.seek(std::io::SeekFrom::Start(0))?;
    path.set_len(0)?;
    path.write_all(&serialized.as_bytes())?;

    Ok(())
}

fn autosave(inventory: &Inventory)-> Result<(), Box<dyn Error>>{
    let mut path = OpenOptions::new().write(true).open("./src/inventory.json")?;
    let serialized = serde_json::to_string(inventory)?;
        
    path.seek(std::io::SeekFrom::Start(0))?;
    path.set_len(0)?;
    path.write_all(&serialized.as_bytes())?;

    Ok(())
}

fn read_json_file(file_path: &Path) -> Result<Inventory, Box<dyn Error>> {
    let mut file = File::open(file_path)?;

    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    let data= serde_json::from_str(&contents).expect("Error converting to Inventory struct!");
    // println!("{:?}",data);

    Ok(data)
}




fn main() {
    loop {
        let criteria = "default";
        let path = Path::new("./src/inventory.json");

        // write_to_file(path).unwrap();
        let mut inventory = read_json_file(path).unwrap();

        print!("Enter a command: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        let input = input.trim().to_lowercase();
        let commands:Vec<&str> = input.split_whitespace().collect();
        
        println!("{:?}",commands);

        match commands.get(0) {
            Some(&"help") => print_help(),
            Some(&"quit") => {
                println!("Exiting...");
                break;
            }
            Some(&"display") => {
                if let Some(criteria) = commands.get(1) {
                    inventory.display(criteria).unwrap();
                } else {
                    inventory.display(criteria).unwrap();
                }
            }
            Some(&"remove") => {
                if let (Some(title), Some(author), Some(genre), Some(quantity), Some(price)) = (
                    commands.get(1),
                    commands.get(2),
                    commands.get(3),
                    commands.get(4),
                    commands.get(5),
                ) {
                    let book = Book {
                        title: title.to_string(),
                        author: author.to_string(),
                        genre: genre.to_string(),
                        quantity: quantity.parse().unwrap_or_default(), 
                        price: price.parse().unwrap_or_default(),     
                    };
                inventory.remove(book).unwrap();
                }
            }
            Some(&"update") => {
                if let (Some(title), Some(author), Some(genre), Some(quantity), Some(price)) = (
                    commands.get(1),
                    commands.get(2),
                    commands.get(3),
                    commands.get(4),
                    commands.get(5),
                ) {
                    let book = Book {
                        title: title.to_string(),
                        author: author.to_string(),
                        genre: genre.to_string(),
                        quantity: quantity.parse().unwrap_or_default(), 
                        price: price.parse().unwrap_or_default(),     
                    };
                inventory.update(book).unwrap();
                }
            }
            Some(&"add") => {
                if let (Some(title), Some(author), Some(genre), Some(quantity), Some(price)) = (
                    commands.get(1),
                    commands.get(2),
                    commands.get(3),
                    commands.get(4),
                    commands.get(5),
                ) {
                    let book = Book {
                        title: title.to_string(),
                        author: author.to_string(),
                        genre: genre.to_string(),
                        quantity: quantity.parse().unwrap_or_default(), 
                        price: price.parse().unwrap_or_default(),     
                    };
                inventory.add(book).unwrap();
                }
            }
            Some(&"sell") =>{
                if let (Some(title), Some(author), Some(genre), Some(quantity), Some(price),Some(sell_quantity)) = (
                    commands.get(1),
                    commands.get(2),
                    commands.get(3),
                    commands.get(4),
                    commands.get(5),
                    commands.get(6),
                ){
                    let book = Book {
                        title: title.to_string(),
                        author: author.to_string(),
                        genre: genre.to_string(),
                        quantity: quantity.parse().unwrap_or_default(), 
                        price: price.parse().unwrap_or_default(),     
                    };
                    println!("book quantity = {} quantity = {}",book.quantity, sell_quantity.parse::<u32>().unwrap());
                    inventory.sell_book(book, sell_quantity.parse::<u32>().unwrap()).unwrap();
                }
            }
            _ => println!("Wrong input. Try 'help' to see the list of commands."),
        }
    }
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_book() {
        let mut inventory = Inventory { books: vec![] };
        let book = Book {
            title: String::from("Book 1"),
            author: String::from("Author 1"),
            genre: String::from("Genre 1"),
            quantity: 10,
            price: 5,
        };

        //add book
        let result = inventory.add(book);
        assert!(result.is_ok());

        //add the same book again, should fail
        // let result = inventory.add(book);
        // assert!(result.is_err());
    }

    #[test]
    fn test_remove_book() {
        let mut inventory = Inventory {
            books: vec![
                Book {
                    title: String::from("Book 1"),
                    author: String::from("Author 1"),
                    genre: String::from("Genre 1"),
                    quantity: 10,
                    price: 5,

                },
                Book {
                    title: String::from("Book 2"),
                    author: String::from("Author 2"),
                    genre: String::from("Genre 2"),
                    quantity: 5,
                    price: 10,
                },
            ],
        };

        //remove a book
        let book_to_remove = Book {
            title: String::from("Book 1"),
            author: String::from("Author 1"),
            genre: String::from("Genre 1"),
            quantity: 10,
            price: 5,
        };
        let result = inventory.remove(book_to_remove);
        assert!(result.is_ok());

        // remove a non-existent book, should fail
        let book_not_in_inventory = Book {
            title: String::from("Book 3"),
            author: String::from("Author 3"),
            genre: String::from("Genre 3"),
            quantity: 8,
            price: 10,
        };
        let result = inventory.remove(book_not_in_inventory);
        assert!(result.is_err());
    }

    #[test]
    fn test_update_book() {
        let mut inventory = Inventory {
            books: vec![
                Book {
                    title: String::from("Book 1"),
                    author: String::from("Author 1"),
                    genre: String::from("Genre 1"),
                    quantity: 10,
                    price: 5,
                },
                Book {
                    title: String::from("Book 2"),
                    author: String::from("Author 2"),
                    genre: String::from("Genre 2"),
                    quantity: 5,
                    price: 10,
                },
            ],
        };

        // update a book
        let updated_book = Book {
            title: String::from("Book 1"),
            author: String::from("Author 1"),
            genre: String::from("Genre 1"),
            quantity: 15,
            price: 5,
        };
        let result = inventory.update(updated_book);
        assert!(result.is_ok());

        // update a non-existent book, should fail
        let non_existent_book = Book {
            title: String::from("Book 3"),
            author: String::from("Author 3"),
            genre: String::from("Genre 3"),
            quantity: 8,
            price: 10,
        };
        let result = inventory.update(non_existent_book);
        assert!(result.is_err());
    }
}
