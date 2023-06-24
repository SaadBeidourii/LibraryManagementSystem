use crate::book::Book;

pub struct Library {
    pub books : Vec<Book>,
}

impl Library{
    pub fn new_library() -> Self {
        Library{
            books: Vec::new(),
        }
    }

    pub fn add_book(&mut self, book: Book){
        self.books.push(book);
    }

    pub fn remove_book(&mut self, isbn: &str) -> Option<Book> {
        if let Some(index) = self.books.iter().position(|b| b.isbn == isbn) {
            Some(self.books.remove(index))
        } else {
            None
        }
    }
}