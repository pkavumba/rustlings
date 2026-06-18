// `chars().map(...)` can transform text one character at a time.

fn redact_vowels(input: &str) -> String {
    input
        .chars()
        .map(|ch| match ch {
            'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => '*',
            ch => ch,
        })
        .collect()
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
