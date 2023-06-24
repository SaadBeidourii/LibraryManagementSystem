#[cfg(test)]
mod tests {
    use crate::library::Library;
    use crate::book::Book;

    #[test]
    fn test_add_book() {
        let mut library = Library::new_library();
        let book = Book::new_book("Title".to_string(), "Author".to_string(), "ISBN".to_string());
        library.add_book(book.clone());

        assert_eq!(library.books.len(), 1);
    }

    #[test]
    fn test_remove_book() {
        let mut library = Library::new_library();
        let book = Book::new_book("Title".to_string(), "Author".to_string(), "ISBN".to_string());
        library.add_book(book.clone());

        let removed_book = library.remove_book("ISBN");
        assert_eq!(removed_book, Some(book));
        assert_eq!(library.books.len(), 0);
    }
}
