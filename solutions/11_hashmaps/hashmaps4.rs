use std::collections::HashMap;
fn word_counts(words: &[&str]) -> HashMap<String, usize> {
    let mut m = HashMap::new();
    for w in words {
        *m.entry((*w).to_string()).or_insert(0) += 1;
    }
    m
}
fn main() {}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn counts_words() {
        let counts = word_counts(&["a", "b", "a"]);
        assert_eq!(counts["a"], 2);
        assert_eq!(counts["b"], 1);
    }
}
