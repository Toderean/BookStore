use std::path::Path;

use book_store::{
    book::Book, consts::PATH, get_command, print_display, print_help, read_json_file,
};


fn main() {
    let mut inventory = read_json_file(Path::new(PATH)).unwrap();

    loop {
        let commands = get_command(" ");
        match commands.as_str() {
            "help" => print_help(),
            "exit" => {
                println!("Exiting...");
                break;
            }
            "display" => loop {
                print_display();
                let show = get_command("display").to_lowercase();
                if show.as_str() == "help" {
                    print_display();
                }
                if show.as_str() == "exit" {
                    break;
                }
                match show.as_str() {
                    "price" | "author" | "title" | "genre" | "quantity" | "all" => {
                        inventory.display(show.as_str(), None).unwrap();
                        break;
                    }
                    _ => println!("Not a command!\n"),
                }
            },
            "remove" => loop {
                inventory.display("all", None).unwrap();
                let show = get_command("which book you want to remove").to_lowercase();
                if show.as_str() == "exit" {
                    break;
                }
                let book_index = match inventory.get_index(&show) {
                    Some(index) => index,
                    None => {
                        println!("This book doesn't exist!\n");
                        continue;
                    }
                };
                inventory.remove(book_index);
            },
            "update" => loop {
                inventory.display("all", None).unwrap();
                let show = get_command("which book you want to update").to_lowercase();
                if show.as_str() == "exit" {
                    break;
                }
                let book_index = match inventory.get_index(&show) {
                    Some(index) => index,
                    None => {
                        println!("This book doesn't exist!\n");
                        continue;
                    }
                };
                loop {
                    println!("\n You can update Title, Author, Genre, Price, Quantity or to exit entering Exit \n");
                    let field = get_command("wich field you want to update").to_lowercase();
                    match field.as_str() {
                        "title" | "author" | "genre" => loop {
                            let update = get_command(&field);
                            inventory
                                .update(book_index, field, update.to_string())
                                .unwrap();
                            inventory.display("book", Some(book_index)).unwrap();
                            break;
                        },
                        "price" | "quantity" => loop {
                            let update = get_command(&field);
                            if update.parse::<u32>().is_err() {
                                println!("\n Not a number \n");
                                continue;
                            }
                            inventory
                                .update(book_index, field, update.to_string())
                                .unwrap();
                            inventory.display("book", Some(book_index)).unwrap();
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
                    match get_command("title").as_str() {
                        "exit" => {
                            println!("Exiting add process...");
                            break;
                        }
                        title if !title.is_empty() => {
                            book.title = title.to_string();
                        }
                        _ => println!("Enter a non-empty title!"),
                    }

                    match get_command("author").as_str() {
                        "exit" => {
                            println!("Exiting add process...");
                            break;
                        }
                        author if !author.is_empty() => {
                            book.author = author.to_string();
                        }
                        _ => println!("Enter a non-empty author!"),
                    }

                    match get_command("genre").as_str() {
                        "exit" => {
                            println!("Exiting add process...");
                            break;
                        }
                        genre if !genre.is_empty() => {
                            book.genre = genre.to_string();
                        }
                        _ => println!("Enter a non-empty genre!"),
                    }

                    let price_input = get_command("price");
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

                    let quantity_input = get_command("quantity");
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
                    inventory.display("all", None).unwrap();
                    break;
                }
            }
            "sell" => loop {
                inventory.display("all", None).unwrap();
                let book_to_sell = get_command("which book you want to sell");
                if book_to_sell.as_str() == "exit" {
                    println!("Exiting sell process...");
                    break;
                }
                let book_index = match inventory.get_index(&book_to_sell) {
                    Some(index) => index,
                    None => {
                        println!("This book doesn't exist!\n");
                        continue;
                    }
                };

                let book = inventory.books.get(book_index).unwrap();

                let sell_quantity = loop {
                    let quantity_input = get_command("how much you want to sell");
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

                inventory.sell_book(book_index, sell_quantity).unwrap();
            },
            "save" => {
                println!("Saving the inventory state...");
                inventory.save().unwrap();
            }
            _ => println!("Wrong input. Try 'help' to see the list of commands."),
        }
    }

    inventory.save().unwrap();
}
