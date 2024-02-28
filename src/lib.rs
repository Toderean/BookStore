use std::{
    error::Error,
    fs::OpenOptions,
    io::{Read, Write},
    path::Path,
};

use inventory::Inventory;

pub mod book;
pub mod consts;
pub mod inventory;
mod tests;

pub fn print_display() {
    println!("  If you want to display the books ordered by some criteria just enter the following keys:");
    println!("  -price :sort by price");
    println!("  -quantity :sort by the quantities");
    println!("  -genre: sort by genre");
    println!("  -author: sort by authors");
    println!("  -title: sort by titles");
    println!("  -all: display all books");
}

pub fn print_help() {
    println!("Instructions: ");
    println!("1.To display all the books just enter: (display) \n");
    println!("2.If you want to add a new book in inventory just enter keyword (add) followed by the necessary fields (title,author,genre,quantity,price).\n");
    println!("3.If you want to delete a book from the inventory just enter the keyword (remove) followed by the name of the book you want to delete.\n");
    println!("4.If you want to update a book you'll need to enter the keyword (update) followd by the title in the and the new fields updated (title,author,genre,quantity,price).\n");
    println!("5.To sell a book you'll need to enter keyword (sell) followed by fields of a book (title,author,genre,quantity,price) and finally insert the quantity you want to sell.\n")
}

pub fn read_json_file(file_path: &Path) -> Result<Inventory, Box<dyn Error>> {
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

pub fn get_command(command: &str) -> String {
    print!("Enter a command {}: ", command);
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let input = input.trim().to_string();
    input
}
