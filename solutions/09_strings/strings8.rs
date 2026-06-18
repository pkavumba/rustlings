// Common string cleanup often chains methods that return either `&str` or
// `String`. Convert at the end when the function needs to return ownership.

fn clean_slug(input: &str) -> String {
    input.trim().to_lowercase().replace(' ', "-")
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_for_urls() {
        assert_eq!(clean_slug("  Rust Strings Are Fun  "), "rust-strings-are-fun");
    }
}
