// Iterate over characters when you mean user-visible scalar values, not bytes.

fn initials(name: &str) -> String {
    name.split_whitespace()
        .filter_map(|word| word.chars().next())
        .flat_map(char::to_uppercase)
        .collect()
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collects_initial_letters() {
        assert_eq!(initials("ferris the crab"), "FTC");
        assert_eq!(initials("  rust language  "), "RL");
    }
}
