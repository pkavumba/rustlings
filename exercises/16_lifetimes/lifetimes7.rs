struct Split<'a> {
    left: &'a str,
    right: &'a str,
}
fn split_once_colon(s: &str) -> Option<Split<'_>> { // TODO: Split on the first colon.
}
fn main() {}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn splits_borrowed() {
        let s = split_once_colon("a:b").unwrap();
        assert_eq!(s.left, "a");
        assert_eq!(s.right, "b");
    }
}
