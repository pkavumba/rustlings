fn parse_numbers(items: &[&str]) -> Result<Vec<i32>, std::num::ParseIntError> { // TODO: Parse all strings, stopping on the first error.
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
