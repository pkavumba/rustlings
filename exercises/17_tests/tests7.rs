// Write or fix tests while keeping production code simple.
fn add(a:i32,b:i32)->i32{ a+b }

#[cfg(test)]
mod tests { use super::*; #[test] fn test_add_7() { // TODO: Assert that add(7, 7) is 14.
} }
fn main(){}
