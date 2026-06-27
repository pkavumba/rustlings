// Struct fields can own data. This exercise practices choosing owned field
// types, field init shorthand, and moving data into a struct.

#[derive(Debug)]
struct Book {
    // TODO: Add the fields that the tests expect.
}

fn build_book(title: String, author: String, pages: u16) -> Book {
    // TODO: Use field init shorthand for fields whose variable names match.
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_owned_book_data() {
        let book = build_book(
            String::from("The Rust Book"),
            String::from("Rust Team"),
            550,
        );

        assert_eq!(book.title, "The Rust Book");
        assert_eq!(book.author, "Rust Team");
        assert_eq!(book.pages, 550);
    }
}
