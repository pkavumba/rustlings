use std::collections::HashMap;
fn group_by_first(words: &[&str]) -> HashMap<char, Vec<String>> { let mut m=HashMap::new(); for w in words { if let Some(c)=w.chars().next(){ m.entry(c).or_insert_with(Vec::new).push((*w).to_string()); } } m }
fn main() {}
#[cfg(test)] mod tests { use super::*; #[test] fn groups_words() { let g=group_by_first(&["rust","ruby","go"]); assert_eq!(g[&'r'], ["rust", "ruby"]); assert_eq!(g[&'g'], ["go"]); } }
