// Returning a string slice lets the result borrow from the input instead of
// allocating a new `String`.

fn first_word(sentence: &str) -> &str {
    match sentence.find(char::is_whitespace) {
        Some(index) => &sentence[..index],
        None => sentence,
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_borrowed_first_word() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("rust"), "rust");
    }
}
