use std::borrow::Cow;
fn ensure_suffix(input: &str) -> Cow<'_, str> {
    if input.ends_with('!') {
        Cow::Borrowed(input)
    } else {
        Cow::Owned(format!("{input}!"))
    }
}
fn main() {}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn cow_borrows_or_owns() {
        assert!(matches!(ensure_suffix("hi!"), Cow::Borrowed(_)));
        assert_eq!(ensure_suffix("hi"), "hi!");
    }
}
