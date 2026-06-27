// Sometimes a parser should borrow pieces from the original input instead of
// allocating. Return string slices for bracketed tags like "[todo]".

fn bracketed_tags(input: &str) -> Vec<&str> {
    let mut tags = Vec::new();
    let mut rest = input;

    while let Some(start) = rest.find('[') {
        rest = &rest[start + 1..];
        if let Some(end) = rest.find(']') {
            tags.push(&rest[..end]);
            rest = &rest[end + 1..];
        } else {
            break;
        }
    }

    tags
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
