use rocket::{Build, Rocket, routes};
use rocket::get;
use rocket::post;
use rocket::Route;
use rocket::response::content;
use rocket::response::content::{RawCss, RawHtml};
use std::io;
use std::fs;
use rocket::fs::FileServer;
use crate::book::Book;
use crate::library::Library;
use crate::mocks::get_mock_books;

#[get("/")]
async fn hello() -> Result<RawHtml<String>, io::Error> {
    let file_path = "html/hello.html";
    let contents = fs::read_to_string(file_path)?;
    Ok(RawHtml(contents))
}

#[get("/books")]
fn get_books() -> Result<RawHtml<String>, io::Error> {
    let file_path = "html/listOfBooks.html";
    let mut contents = fs::read_to_string(file_path)?;

    let book_list = get_mock_books();  // Retrieve the mock books
    let book_list_html = generate_book_list_html(&book_list);

    // Replace a placeholder in the HTML template with the book list HTML
    contents = contents.replace("{{book_list}}", &book_list_html);

    // Load and include your CSS file using RawCss
    let css_content = fs::read_to_string("css/card.css")?;
    let css = RawCss(css_content.clone());

    // Create the Html response with included CSS
    let response = RawHtml(format!(
        r#"<style type="text/css">{}</style>{}"#,
        css_content, contents
    ));

    Ok(response)
}


fn generate_book_list_html(book_list: &[Book]) -> String {
    let mut html_content = String::new();

    for book in book_list {
        html_content.push_str(&format!(
            r#"<div class="card">
                <p>Title: {}</p>
                <p>Author: {}</p>
                <p>ISBN: {}</p>
            </div>"#,
            book.title, book.author,book.isbn
        ));
    }

    html_content
}


#[post("/books")]
fn add_book() -> &'static str {
    "Book added"
}


pub fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![hello, get_books, add_book])
}

