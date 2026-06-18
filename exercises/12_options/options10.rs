// Nested options can be flattened.
fn flatten_score(score: Option<Option<u32>>) -> Option<u32> { // TODO: Remove one layer of Option.
}
fn main() {}
#[cfg(test)] mod tests { use super::*; #[test] fn flattens_nested_options() { assert_eq!(flatten_score(Some(Some(10))), Some(10)); assert_eq!(flatten_score(Some(None)), None); assert_eq!(flatten_score(None), None); } }
