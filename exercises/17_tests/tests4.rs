// Write or fix tests while keeping production code simple.
fn add(a:i32,b:i32)->i32{ a+b }

#[cfg(test)]
mod tests { use super::*; #[test] fn test_add_4() { // TODO: Assert that add(4, 4) is 8.
} }
fn main(){}
