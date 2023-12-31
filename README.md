# Library Management (Rust)

This project aims to develop a simple library management system using Rust. The application allows users to add, delete, and search for books in a virtual library. It also includes additional features such as loan tracking, user management, and report generation.

## Development Steps

1. **Define Data Structures:** Begin by defining the necessary data structures, including structures for books, users, and loans.

2. **Implement Basic Features:** Implement basic functionality such as adding books, deleting books, and searching for books by title or author.

3. **Loan Tracking:** Add the capability to track loans, recording who borrowed a book and when it is due for return.

4. **Command-Line Interface (CLI):** Set up a command-line interface (CLI) to provide an interactive way for users to interact with the library management system.

5. **Additional Features:** Customize the project by adding features such as user management, generating reports on current loans, or any other functionalities.

## Rust Concepts to Explore

While working on this project, gives the opportunity to explore various Rust concepts, including:

- Secure memory management with Rust's ownership system.
- Usage of data structures, enumerations, and traits.
- Handling strings and vectors.
- Error handling with Rust's error management system.
- Utilizing Rust's standard collections, such as `Vec` and `HashMap`.
- Organizing code using modules and packages.

## Rocket GUI Implementation

In addition to the command-line interface (CLI) version of the library management system, this project also includes an implementation of a graphical user interface (GUI) using the Rocket framework.

To launch the GUI version of the library management system, follow these steps:

1. Ensure you have the necessary dependencies by running `cargo update` to update your dependencies based on the `Cargo.toml` file.

2. Open a terminal or command prompt and navigate to the project directory.

3. Run the command `cargo run` to compile and launch the application.

4. Access the library management system GUI by opening a web browser and entering `http://localhost:8000` in the address bar.

5. Interact with the graphical interface to add, delete, search for books, and explore other features of the library management system.

Please note that the Rocket GUI implementation requires an internet browser to access the interface.

## HTML File Integration

To keep the HTML files separate from the Rust code, i create a directory called `html` in the `src` directory. This directory will contain all the HTML files needed for the GUI. In the `routes.rs` file, you can define routes that correspond to these HTML files.

For example, if you have an HTML file named `hello.html` located in the `html` directory, you can define a route in the `routes.rs` file as follows:

```rust
#[get("/")]
async fn hello() -> Result<RawHtml<String>, io::Error> {
    let file_path = "html/hello.html";
    let contents = fs::read_to_string(file_path)?;
    Ok(RawHtml(contents))
}
