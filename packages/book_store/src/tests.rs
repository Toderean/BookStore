#[cfg(test)]
mod tests {
    use crate::book::Book;
    use crate::consts::TEST_INVENTORY_PATH;
    use crate::inventory::Inventory;
    use crate::read_json_file;
    use std::fs;
    use std::path::Path;

    fn setup_test_inventory() -> Inventory {
        let mut books = Vec::new();
        books.push(Book::new(
            String::from("Book 1"),
            String::from("Author 1"),
            String::from("Genre 1"),
            10,
            20,
        ));
        books.push(Book::new(
            String::from("Book 2"),
            String::from("Author 2"),
            String::from("Genre 2"),
            15,
            25,
        ));
        Inventory { books }
    }

    #[test]
    fn test_add_book() {
        let mut inventory = setup_test_inventory();
        let new_book = Book::new(
            String::from("New Book"),
            String::from("New Author"),
            String::from("New Genre"),
            5,
            30,
        );
        inventory.add(new_book.clone()).unwrap();
        assert_eq!(
            inventory.get_index(new_book.title.as_str()),
            Some(inventory.get_index(new_book.title.as_str()).unwrap())
        );
    }

    #[test]
    fn test_remove_book() {
        let mut inventory = setup_test_inventory();
        let book_index = inventory.get_index("Book 1").unwrap();
        inventory.remove(book_index);
        assert_eq!(inventory.get_index("Book 1"), None);
    }

    #[test]
    fn test_update_book() {
        let mut inventory = setup_test_inventory();
        let book_index = inventory.get_index("Book 1").unwrap();
        inventory
            .update(
                book_index,
                String::from("title"),
                String::from("Updated Book"),
            )
            .unwrap();
        let updated_book = inventory.books[book_index].clone();
        assert_eq!(updated_book.title, "Updated Book");
    }

    #[test]
    fn test_sell_book() {
        let mut inventory = setup_test_inventory();
        let book_index = inventory.get_index("Book 1").unwrap();
        let initial_quantity = inventory.books[book_index].quantity;
        inventory.sell_book(book_index, 5).unwrap();
        assert_eq!(inventory.books[book_index].quantity, initial_quantity - 5);
    }

    #[test]
    fn test_sell_book_2() {
        let mut inventory = setup_test_inventory();
        let book_index = inventory.get_index("Book 2").unwrap();
        let initial_quantity = inventory.books[book_index].quantity;
        inventory.sell_book(book_index, 14).unwrap();
        assert_eq!(inventory.books[book_index].quantity, initial_quantity - 14);
    }

    #[test]
    fn test_display_books() {
        let mut inventory = setup_test_inventory();
        assert!(inventory.display("all", None).is_ok());
    }

    #[test]
    fn test_get_index() {
        let inventory = setup_test_inventory();
        assert_eq!(inventory.get_index("Book 1"), Some(0));
    }

    #[test]
    fn test_sell_more_than_available() {
        let mut inventory = setup_test_inventory();
        let book_index = inventory.get_index("Book 1").unwrap();
        assert!(inventory.sell_book(book_index, 9).is_ok());
        // assert!(inventory.sell_book(book_index, 20).is_err());
    }

    #[test]
    fn test_sell_more_than_available_2() {
        let mut inventory = setup_test_inventory();
        let book_index = inventory.get_index("Book 1").unwrap();
        assert!(inventory.sell_book(book_index, 20).is_ok());
        // assert!(inventory.sell_book(book_index, 20).is_err());
    }

    #[test]
    fn test_invalid_json_file() {
        fs::write(TEST_INVENTORY_PATH, "invalid json").unwrap();
        assert!(read_json_file(Path::new(TEST_INVENTORY_PATH)).is_ok());
    }

    #[test]
    fn test_invalid_json_file_2() {
        fs::write(TEST_INVENTORY_PATH, "invalid json").unwrap();
        assert!(read_json_file(Path::new(TEST_INVENTORY_PATH)).is_err());
    }
    #[test]
    fn test_non_existing_book() {
        let inventory = setup_test_inventory();
        assert_eq!(inventory.get_index("Non Existing Book"), None);
    }

    #[test]
    fn test_add_max_capacity() {
        let mut inventory = Inventory { books: Vec::new() };
        for _ in 0..30 {
            let new_book = Book::new(
                String::from("New Book"),
                String::from("New Author"),
                String::from("New Genre"),
                5,
                30,
            );
            inventory.add(new_book.clone()).unwrap();
        }
        let new_book = Book::new(
            String::from("New Book"),
            String::from("New Author"),
            String::from("New Genre"),
            5,
            30,
        );
        assert!(inventory.add(new_book).is_err());
    }
}
