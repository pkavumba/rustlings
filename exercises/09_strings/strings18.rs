// You can rearrange borrowed string slices and collect them into a new owned
// `String`.

fn reverse_words(input: &str) -> String {
    // TODO: Reverse the order of whitespace-separated words.
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverses_word_order() {
        assert_eq!(reverse_words("one two three"), "three two one");
        assert_eq!(reverse_words("  rust   strings  "), "strings rust");
    }
}
