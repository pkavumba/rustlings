// Strings can be extended in place. Practice `push_str` for string slices
// and `push` for single characters.

fn add_signature(mut message: String) -> String {
    // TODO: Append a newline, "-- ", and the crab emoji 🦀 to `message`.
    message
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mutates_string_in_place() {
        assert_eq!(
            add_signature(String::from("Keep learning")),
            "Keep learning\n-- 🦀"
        );
    }
}
