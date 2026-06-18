// `chars().map(...)` can transform text one character at a time.

fn redact_vowels(input: &str) -> String {
    // TODO: Replace ASCII vowels (a, e, i, o, u in either case) with `*` and
    // keep every other character unchanged.
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn redacts_ascii_vowels() {
        assert_eq!(redact_vowels("Rustacean"), "R*st*c**n");
        assert_eq!(redact_vowels("Why?"), "Why?");
    }
}
