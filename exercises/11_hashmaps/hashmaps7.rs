use std::collections::HashMap;
fn group_by_first(words: &[&str]) -> HashMap<char, Vec<String>> { // TODO: Group words by their first character.
}
fn main() {}
#[cfg(test)] mod tests { use super::*; #[test] fn groups_words() { let g=group_by_first(&["rust","ruby","go"]); assert_eq!(g[&'r'], ["rust", "ruby"]); assert_eq!(g[&'g'], ["go"]); } }
