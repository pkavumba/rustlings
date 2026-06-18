// Write or fix tests while keeping production code simple.
fn add(a:i32,b:i32)->i32{ a+b }

#[cfg(test)]
mod tests { use super::*; #[test] fn test_add_5() { assert_eq!(add(5, 5), 10); } }
fn main(){}
