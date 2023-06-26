use rocket::{Build, Rocket, routes};
use rocket::get;
use rocket::post;
use rocket::Route;
use rocket::response::content;
use rocket::response::content::RawHtml;



#[get("/")]
fn hello() -> RawHtml<String> {
    let hello_world = "<h1>Hello, world!</h1>";
    let button_html =  r#"<a href="/books"><button>List of books</button></a>"#;
    let html_content = format!("{} {}", hello_world, button_html);
    RawHtml(html_content)
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

