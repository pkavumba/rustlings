// `and_then` chains computations that can fail.
fn half_even(n: Option<i32>) -> Option<i32> {
    n.and_then(|n| if n % 2 == 0 { Some(n / 2) } else { None })
}
fn main() {}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn chains_optional_checks() {
        assert_eq!(half_even(Some(8)), Some(4));
        assert_eq!(half_even(Some(7)), None);
        assert_eq!(half_even(None), None);
    }
}
