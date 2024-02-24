use serde::{Deserialize, Serialize};
use std::fs::{self, File, OpenOptions};
use std::io::{self, Seek, Write};
use std::path::Path;
use std::{error::Error, io::Read};

const PATH: &str = "./src/inventory.json";

#[derive(Serialize, Deserialize, Debug, Default)]
struct Book {
    title: String,
    author: String,
    genre: String,
    quantity: u32,
    price: u32,
}

impl Book {
    fn new(title: String, author: String, genre: String, quantity: u32, price: u32) -> Book {
        Book {
            title,
            author,
            genre,
            quantity,
            price,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Inventory {
    books: Vec<Book>,
}

impl Inventory {
    fn add(&mut self, b: Book) -> Result<(), Box<dyn Error>> {
        let max_capacity = 30;
        if self.books.len() < max_capacity {
            self.books.push(b);
            autosave(self)?;
            Ok(())
        } else {
            autosave(self)?;
            Err("Cannot add any more books".into())
        }
    }

    fn remove(&mut self, b: usize) -> Result<(), Box<dyn Error>> {
        if b >= 0 {
            self.books.remove(b);
            Ok(())
        } else {
            Err("cannot have negative numbers".into())
        }
    }

    fn update(
        &mut self,
        index: usize,
        field: String,
        update: String,
    ) -> Result<(), Box<dyn Error>> {
        let mut book = match self.books.get_mut(index) {
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

    fn display(&mut self, criteria: &str) -> Result<(), Box<dyn Error>> {
        match criteria {
            "author" => {
                println!("Books are sorted by author:\n");
                self.books.sort_by_key(|x| x.author.clone());
                for books in &self.books {
                    println!(
                        "Title: {}, Author: {}, Genre: {}, Quantity: {}, Price: {},",
                        books.title, books.author, books.genre, books.quantity, books.price
                    );
                }
                println!();
                Ok(())
            }
            "title" => {
                println!("Books are sorted by title:\n");
                self.books.sort_by_key(|x| x.title.clone());
                for books in &self.books {
                    println!(
                        "Title: {}, Author: {}, Genre: {}, Quantity: {}, Price: {},",
                        books.title, books.author, books.genre, books.quantity, books.price
                    );
                }
                println!();
                Ok(())
            }
            "genre" => {
                println!("Books are sorted by genre:\n");
                self.books.sort_by_key(|x| x.genre.clone());
                for books in &self.books {
                    println!(
                        "Title: {}, Author: {}, Genre: {}, Quantity: {}, Price: {},",
                        books.title, books.author, books.genre, books.quantity, books.price
                    );
                }
                println!();
                Ok(())
            }
            "quantity" => {
                println!("Books are sorted by quantity:\n");
                self.books.sort_by_key(|x| x.quantity.clone());
                for books in &self.books {
                    println!(
                        "Title: {}, Author: {}, Genre: {}, Quantity: {}, Price: {},",
                        books.title, books.author, books.genre, books.quantity, books.price
                    );
                }
                println!();
                Ok(())
            }
            "price" => {
                println!("Books are sorted by price:\n");
                self.books.sort_by_key(|x| x.price.clone());
                for books in &self.books {
                    println!(
                        "Title: {}, Author: {}, Genre: {}, Quantity: {}, Price: {},",
                        books.title, books.author, books.genre, books.quantity, books.price
                    );
                }
                println!();
                Ok(())
            }
            "all" => {
                println!("Books in the inventory:\n");
                for books in &self.books {
                    println!(
                        "Title: {}, Author: {}, Genre: {}, Quantity: {}, Price: {},",
                        books.title, books.author, books.genre, books.quantity, books.price
                    );
                }
                println!();
                Ok(())
            }
            _ => Err("Invalid criteria".into()),
        }
    }

    fn sell_book(&self, mut b: Book, sell_quantity: u32) -> Result<(), Box<dyn Error>> {  
        b.quantity -= sell_quantity;
        Ok(())
    }
}

fn print_display() {
    println!("  If you want to display the books ordered by some criteria just enter the following keys:");
    println!("  -price :sort by price");
    println!("  -quantity :sort by the quantities");
    println!("  -genre: sort by genre");
    println!("  -author: sort by authors");
    println!("  -title: sort by titles");
    println!("  -all: display all books");
}

fn print_help() {
    println!("Instructions: ");
    println!("1.To display all the books just enter: (display) \n");
    println!("2.If you want to add a new book in inventory just enter keyword (add) followed by the necessary fields (title,author,genre,quantity,price).\n");
    println!("3.If you want to delete a book from the inventory just enter the keyword (remove) followed by the name of the book you want to delete.\n");
    println!("4.If you want to update a book you'll need to enter the keyword (update) followd by the title in the and the new fields updated (title,author,genre,quantity,price).\n");
    println!("5.To sell a book you'll need to enter keyword (sell) followed by fields of a book (title,author,genre,quantity,price) and finally insert the quantity you want to sell.\n")
}

fn get_index(inventory: &Inventory, book: &str) -> Option<usize> {
    inventory
        .books
        .iter()
        .position(|x| x.title.to_lowercase() == book.to_lowercase())
}

fn autosave(inventory: &Inventory) -> Result<(), Box<dyn Error>> {
    let mut path = OpenOptions::new().write(true).open(PATH)?;
    let serialized = serde_json::to_string(inventory)?;

    path.seek(std::io::SeekFrom::Start(0))?;
    path.set_len(0)?;
    path.write_all(&serialized.as_bytes())?;

    Ok(())
}

fn read_json_file(file_path: &Path) -> Result<Inventory, Box<dyn Error>> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)?;

    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    let data = match serde_json::from_str::<Inventory>(&contents) {
        Ok(inventory) => inventory,
        Err(_) => Inventory { books: vec![] },
    };

    Ok(data)
}

fn get_command(command: String) -> String {
    print!("Enter a command {}: ", command);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let input = input.trim().to_string();
    input
}
fn main() {
    loop {
        let criteria = "all";
        let path: &Path = Path::new(PATH);

        let mut inventory = read_json_file(path).unwrap();

        let commands = get_command(" ".to_string());
        match commands.as_str() {
            "help" => print_help(),
            "exit" => {
                println!("Exiting...");
                break;
            }
            "display" => loop {
                print_display();
                let show = get_command("display".to_string()).to_lowercase();
                if show.as_str() == "help" {
                    print_display();
                }
                if show.as_str() == "exit" {
                    break;
                }
                match show.as_str() {
                    "price" => {
                        inventory.display("price").unwrap();
                        break;
                    }
                    "author" => {
                        inventory.display("author").unwrap();
                        break;
                    }
                    "title" => {
                        inventory.display("title").unwrap();
                        break;
                    }
                    "genre" => {
                        inventory.display("genre").unwrap();
                        break;
                    }
                    "quantity" => {
                        inventory.display("quantity").unwrap();
                        break;
                    }
                    "all" => {
                        inventory.display("all").unwrap();
                        break;
                    }
                    _ => println!("Not a command!\n"),
                }
            },
            "remove" => loop {
                inventory.display("all").unwrap();
                let show = get_command("which book you want to remove".to_string()).to_lowercase();
                if show.as_str() == "exit" {
                    break;
                }
                let book_index = match get_index(&inventory, &show) {
                    Some(index) => index,
                    None => {
                        println!("This book doesn't exist!\n");
                        continue;
                    }
                };
                inventory.remove(book_index).unwrap();
            },
            "update" => loop {
                inventory.display("all").unwrap();
                let show = get_command("which book you want to update".to_string()).to_lowercase();
                if show.as_str() == "exit" {
                    break;
                }
                let book_index = match get_index(&inventory, &show) {
                    Some(index) => index,
                    None => {
                        println!("This book doesn't exist!\n");
                        continue;
                    }
                };
                loop {
                    println!("\n You can update Title, Author, Genre, Price, Quantity or to exit entering Exit \n");
                    let field =
                        get_command("wich field you want to update".to_string()).to_lowercase();
                    match field.as_str() {
                        "title" => loop {
                            let update = get_command("title".to_string());
                            inventory
                                .update(book_index, "title".to_string(), update.to_string())
                                .unwrap();
                            inventory.display("all").unwrap();
                            break;
                        },
                        "author" => loop {
                            let update = get_command("author".to_string());
                            inventory
                                .update(book_index, "author".to_string(), update.to_string())
                                .unwrap();
                            inventory.display("all").unwrap();
                            break;
                        },
                        "genre" => loop {
                            let update = get_command("genre".to_string());
                            inventory
                                .update(book_index, "genre".to_string(), update.to_string())
                                .unwrap();
                            inventory.display("all").unwrap();
                            break;
                        },
                        "price" => loop {
                            let update = get_command("title".to_string());
                            while !update.parse::<u32>().is_ok() {
                                println!("\n Not a number \n");
                            }
                            inventory
                                .update(book_index, "title".to_string(), update.to_string())
                                .unwrap();
                            inventory.display("all").unwrap();
                            break;
                        },
                        "quantity" => loop {
                            let update = get_command("title".to_string());
                            while !update.parse::<u32>().is_ok() {
                                println!("\n Not a number \n");
                            }
                            inventory
                                .update(book_index, "title".to_string(), update.to_string())
                                .unwrap();
                            inventory.display("all").unwrap();
                            break;
                        },
                        "exit" => break,
                        _ => println!("This field doens't exist \n"),
                    }
                }
            },
            "add" => {
                let mut book = Book::default();

                loop {
                    match get_command("title".to_string()).as_str() {
                        "exit" => {
                            println!("Exiting add process...");
                            break;
                        }
                        title if !title.is_empty() => {
                            book.title = title.to_string();
                        }
                        _ => println!("Enter a non-empty title!"),
                    }

                    match get_command("author".to_string()).as_str() {
                        "exit" => {
                            println!("Exiting add process...");
                            break;
                        }
                        author if !author.is_empty() => {
                            book.author = author.to_string();
                        }
                        _ => println!("Enter a non-empty author!"),
                    }

                    match get_command("genre".to_string()).as_str() {
                        "exit" => {
                            println!("Exiting add process...");
                            break;
                        }
                        genre if !genre.is_empty() => {
                            book.genre = genre.to_string();
                        }
                        _ => println!("Enter a non-empty genre!"),
                    }

                    let price_input = get_command("price".to_string());
                    if price_input == "exit" {
                        println!("Exiting add process...");
                        break;
                    }
                    let price = match price_input.parse::<u32>() {
                        Ok(value) => value,
                        Err(_) => {
                            println!("Enter a valid price or type 'exit' to cancel!");
                            continue;
                        }
                    };
                    book.price = price;

                    let quantity_input = get_command("quantity".to_string());
                    if quantity_input == "exit" {
                        println!("Exiting add process...");
                        break;
                    }
                    let quantity = match quantity_input.parse::<u32>() {
                        Ok(value) => value,
                        Err(_) => {
                            println!("Enter a valid quantity or type 'exit' to cancel!");
                            continue;
                        }
                    };
                    book.quantity = quantity;

                    inventory.add(book).unwrap();
                    println!("\nBook added successfully!\n");
                    inventory.display("all").unwrap();
                    break;
                }
            }
            "sell" => {
                    loop{
                    inventory.display("all").unwrap();
                    let book_to_sell = get_command("wich book you want to sell".to_string());
                    if book_to_sell.as_str() == "exit"{
                        println!("Exiting sell process...");
                        break;
                    }
                    let book_index = match get_index(&inventory, &book_to_sell) {
                        Some(index) => index,
                        None => {
                            println!("This book doesn't exist!\n");
                            continue;
                        }
                    };

                    let mut book = match inventory.books.get(book_index) {
                        Some(b) => b.clone(),
                        None => {
                            println!("Error finding the book\n");
                            continue;
                        }
                    };
                    
                    {
                        let sell_quantity = loop {
                            let quantity_input = get_command("how much you want to sell".to_string());
                            if quantity_input == "exit" {
                                println!("Exiting sell process...");
                                break 0;
                            }
                            match quantity_input.parse::<u32>() {
                                Ok(quantity) => {
                                    if book.quantity < quantity {
                                        println!("Not enough books!");
                                        continue;
                                    }
                                    break quantity;
                                }
                                Err(_) => println!("Enter a valid quantity!"),
                            }
                        };
                    
                        // inventory.sell_book(book, sell_quantity).unwrap();
                    }
                }
            }
            _ => println!("Wrong input. Try 'help' to see the list of commands."),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_add_book() {
//         let mut inventory = Inventory { books: vec![] };
//         let book = Book {
//             title: String::from("Book 1"),
//             author: String::from("Author 1"),
//             genre: String::from("Genre 1"),
//             quantity: 10,
//             price: 5,
//         };

//         //add book
//         let result = inventory.add(book);
//         assert!(result.is_ok());

//         //add the same book again, should fail
//         // let result = inventory.add(book);
//         // assert!(result.is_err());
//     }

//     #[test]
//     fn test_remove_book() {
//         let mut inventory = Inventory {
//             books: vec![
//                 Book {
//                     title: String::from("Book 1"),
//                     author: String::from("Author 1"),
//                     genre: String::from("Genre 1"),
//                     quantity: 10,
//                     price: 5,
//                 },
//                 Book {
//                     title: String::from("Book 2"),
//                     author: String::from("Author 2"),
//                     genre: String::from("Genre 2"),
//                     quantity: 5,
//                     price: 10,
//                 },
//             ],
//         };

//         //remove a book
//         let book_to_remove = Book {
//             title: String::from("Book 1"),
//             author: String::from("Author 1"),
//             genre: String::from("Genre 1"),
//             quantity: 10,
//             price: 5,
//         };
//         let result = inventory.remove(book_to_remove);
//         assert!(result.is_ok());

//         // remove a non-existent book, should fail
//         let book_not_in_inventory = Book {
//             title: String::from("Book 3"),
//             author: String::from("Author 3"),
//             genre: String::from("Genre 3"),
//             quantity: 8,
//             price: 10,
//         };
//         let result = inventory.remove(book_not_in_inventory);
//         assert!(result.is_err());
//     }

//     #[test]
//     fn test_update_book() {
//         let mut inventory = Inventory {
//             books: vec![
//                 Book {
//                     title: String::from("Book 1"),
//                     author: String::from("Author 1"),
//                     genre: String::from("Genre 1"),
//                     quantity: 10,
//                     price: 5,
//                 },
//                 Book {
//                     title: String::from("Book 2"),
//                     author: String::from("Author 2"),
//                     genre: String::from("Genre 2"),
//                     quantity: 5,
//                     price: 10,
//                 },
//             ],
//         };

//         // update a book
//         let updated_book = Book {
//             title: String::from("Book 1"),
//             author: String::from("Author 1"),
//             genre: String::from("Genre 1"),
//             quantity: 15,
//             price: 5,
//         };
//         let result = inventory.update(updated_book);
//         assert!(result.is_ok());

//         // update a non-existent book, should fail
//         let non_existent_book = Book {
//             title: String::from("Book 3"),
//             author: String::from("Author 3"),
//             genre: String::from("Genre 3"),
//             quantity: 8,
//             price: 10,
//         };
//         let result = inventory.update(non_existent_book);
//         assert!(result.is_err());
//     }
// }
