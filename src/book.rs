#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub isbn: String,
}

impl Book {
    pub fn new_book(title: String, author: String, isbn: String) -> Self {
        Book {
            title,
            author,
            isbn,
        }
    }

    /*pub fn to_string(&self) -> String {
        format!("Title: {}\nAuthor: {}\nISBN: {}", self.title, self.author, self.isbn)
    }*/
}
