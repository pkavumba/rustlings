// Convert Option to Result when you need an error message.
fn require_name(name: Option<String>) -> Result<String, String> { // TODO: Return Ok(name) or Err("missing name").
}
fn main() {}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn converts_to_result() {
        assert_eq!(require_name(Some("Ferris".into())), Ok("Ferris".into()));
        assert_eq!(require_name(None), Err("missing name".into()));
    }
}
