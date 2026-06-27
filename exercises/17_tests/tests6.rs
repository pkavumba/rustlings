// Write or fix tests while keeping production code simple.
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add_6() { // TODO: Assert that add(6, 6) is 12.
    }
}
fn main() {}
