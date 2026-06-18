// Write or fix tests while keeping production code simple.
fn add(a:i32,b:i32)->i32{ a+b }

#[cfg(test)]
mod tests { use super::*; #[test] fn test_add_5() { // TODO: Assert that add(5, 5) is 10.
} }
fn main(){}
