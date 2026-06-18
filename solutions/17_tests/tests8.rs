// Write or fix tests while keeping production code simple.
fn add(a:i32,b:i32)->i32{ a+b }

#[cfg(test)]
mod tests { use super::*; #[test] fn test_add_8() { assert_eq!(add(8, 8), 16); } }
fn main(){}
