#[derive(Debug)]
struct Library {
    name: String,
    books: BookCollection,
}

#[derive(Debug, Clone)]
struct BookCollection(Vec<String>);

impl Library {
    fn add_book(&mut self, book: &str) {
        self.books.0.push(book.to_string());
    }

    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            books: BookCollection(Vec::new()),
        }
    }

    fn get_books(&self) -> BookCollection {
        self.books.clone()
    }
}

impl Iterator for BookCollection {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        match self.0.pop() {
            Some(book) => {
                println!("Accessing book: {book}");
                Some(book)
            }

            None => {
                println!("Out of books at the library!");
                None
            }
        }
    }
}

fn main() {
    let mut my_library = Library::new("Calgary");
    my_library.add_book("The Doom of the Darksword");
    my_library.add_book("Demian - die Geschichte einer Jugend");
    my_library.add_book("구운몽");
    my_library.add_book("吾輩は猫である");

    for item in my_library.get_books() {
        println!("{item}");
    }
}
