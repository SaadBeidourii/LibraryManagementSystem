#[macro_use]

mod library_test;
mod library;
mod book;
mod routes;
mod mocks;


#[rocket::main]
async fn main() {
    routes::rocket().launch().await;
}