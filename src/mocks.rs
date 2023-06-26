
use crate::book::Book;

pub fn get_mock_books() -> Vec<Book> {
    // Create and return a vector of mock books
    let book1 = Book {
        title: "Book 1".parse().unwrap(),
        author: "Author 1".to_string(),
        isbn: "22".to_string(),
    };

    let book2 = Book {
        title: "Book 2".to_string(),
        author: "Author 2".to_string(),
        isbn: "23".to_string(),
    };

    let book3 = Book {
        title: "Book 3".to_string(),
        author: "Author 3".to_string(),
        isbn: "22".to_string(),
    };


    vec![book1, book2, book3]
}
