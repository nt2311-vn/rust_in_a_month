#[derive(Debug)]
struct Library {
    name: String,
    books: Vec<String>,
}

impl Library {
    fn add_book(&mut self, book: &str) {
        self.books.push(book.to_string());
    }

    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            books: Vec::new(),
        }
    }
}
fn main() {
    let my_library = Library::new("Calgary");
    println!("{my_library:?}");
}
