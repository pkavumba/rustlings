fn first_nonempty<'a>(items: &[&'a str]) -> Option<&'a str> { items.iter().copied().find(|s| !s.is_empty()) }
fn main(){}
#[cfg(test)] mod tests{use super::*;#[test]fn finds_borrow(){assert_eq!(first_nonempty(&["","ok"]),Some("ok"));}}
