// `while let` can repeatedly handle Some values.
fn drain_stack(mut stack: Vec<i32>) -> Vec<i32> { let mut out = Vec::new(); while let Some(value) = stack.pop() { out.push(value); } out }
fn main() {}
#[cfg(test)] mod tests { use super::*; #[test] fn drains_until_none() { assert_eq!(drain_stack(vec![1,2,3]), vec![3,2,1]); } }
