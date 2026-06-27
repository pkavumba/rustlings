// `split_whitespace` handles repeated spaces, tabs, and newlines. Join the
// pieces back together with one regular space.

fn normalize_whitespace(input: &str) -> String {
    input.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collapses_whitespace() {
        assert_eq!(
            normalize_whitespace("  Rust\tstrings\nrock  "),
            "Rust strings rock"
        );
    }
}
