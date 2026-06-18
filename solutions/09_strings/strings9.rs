// A Rust `String` stores UTF-8 bytes. Bytes and characters are not always the
// same count.

fn counts(input: &str) -> (usize, usize) {
    (input.len(), input.chars().count())
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distinguishes_bytes_from_chars() {
        assert_eq!(counts("rust"), (4, 4));
        assert_eq!(counts("🦀é"), (6, 2));
    }
}
