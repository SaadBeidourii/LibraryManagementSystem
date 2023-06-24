use rocket::{Build, Rocket, routes};
use rocket::get;
use rocket::post;
use rocket::Route;



#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}


#[get("/books")]
fn get_books() -> &'static str {
    "List of books"
    // Ajoutez ici la logique pour récupérer la liste des livres et la renvoyer en réponse
}

#[post("/books")]
fn add_book() -> &'static str {
    "Book added"
    // Ajoutez ici la logique pour ajouter un livre à la liste
}


pub fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![hello, get_books, add_book])
}

