fn parse_numbers(items: &[&str]) -> Result<Vec<i32>, std::num::ParseIntError> {
    items.iter().map(|s| s.parse()).collect()
}
fn main() {}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn collects_result() {
        assert_eq!(parse_numbers(&["1", "2"]).unwrap(), [1, 2]);
        assert!(parse_numbers(&["x"]).is_err());
    }
}
