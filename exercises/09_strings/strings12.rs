// Iterate over characters when you mean user-visible scalar values, not bytes.

fn initials(name: &str) -> String {
    // TODO: Return the uppercase first character of each whitespace-separated
    // word. For "ferris the crab", return "FTC".
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
