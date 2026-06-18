// Strings can be extended in place. Practice `push_str` for string slices
// and `push` for single characters.

fn add_signature(mut message: String) -> String {
    message.push('\n');
    message.push_str("-- ");
    message.push('🦀');
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
        assert_eq!(add_signature(String::from("Keep learning")), "Keep learning\n-- 🦀");
    }
}
