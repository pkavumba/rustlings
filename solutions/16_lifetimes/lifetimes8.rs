fn static_message() -> &'static str {
    "always here"
}
fn main() {}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn returns_static() {
        assert_eq!(static_message(), "always here");
    }
}
