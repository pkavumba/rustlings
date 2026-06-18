// Slicing by byte index can panic for UTF-8 text. Build prefixes by iterating
// over characters instead.

fn char_prefix(input: &str, count: usize) -> String {
    input.chars().take(count).collect()
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn takes_character_prefix_safely() {
        assert_eq!(char_prefix("crab", 2), "cr");
        assert_eq!(char_prefix("🦀rust", 2), "🦀r");
        assert_eq!(char_prefix("hi", 10), "hi");
    }
}
