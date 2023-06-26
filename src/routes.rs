use rocket::{Build, Rocket, routes};
use rocket::get;
use rocket::post;
use rocket::Route;
use rocket::response::content;
use rocket::response::content::RawHtml;
use std::io;
use std::fs;

#[get("/")]
async fn hello() -> Result<RawHtml<String>, io::Error> {
    let file_path = "html/hello.html";
    let contents = fs::read_to_string(file_path)?;
    Ok(RawHtml(contents))
}

#[get("/books")]
fn get_books() -> RawHtml<String> {
    let list_of_books="<h1>List of books</h1>";
    let html_content_books= format!("{}",list_of_books);
    RawHtml(html_content_books)
}

#[post("/books")]
fn add_book() -> &'static str {
    "Book added"
}


pub fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![hello, get_books, add_book])
}

