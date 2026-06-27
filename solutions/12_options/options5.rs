// `map` transforms the value inside `Some` and leaves `None` alone.
fn shout(message: Option<String>) -> Option<String> {
    message.map(|s| s.to_uppercase())
}
fn main() {}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn maps_optional_message() {
        assert_eq!(shout(Some("hi".into())), Some("HI".into()));
        assert_eq!(shout(None), None);
    }
}
