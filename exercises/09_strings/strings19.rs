// Sometimes a parser should borrow pieces from the original input instead of
// allocating. Return string slices for bracketed tags like "[todo]".

fn bracketed_tags(input: &str) -> Vec<&str> {
    // TODO: Return all substrings found between `[` and `]`. Ignore incomplete
    // tags. For "[a] text [b]", return ["a", "b"].
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn borrows_tags_from_input() {
        assert_eq!(
            bracketed_tags("[todo] write [rust] code [oops"),
            ["todo", "rust"]
        );
    }
}
