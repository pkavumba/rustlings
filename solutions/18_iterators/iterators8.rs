fn sum_lengths(words: &[&str]) -> usize {
    words.iter().fold(0, |acc, word| acc + word.len())
}
fn main() {}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn folds_lengths() {
        assert_eq!(sum_lengths(&["hi", "rust"]), 6);
    }
}
