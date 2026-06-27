// String parsers often return owned values after validating separators.

fn parse_assignment(input: &str) -> Option<(String, String)> {
    let (key, value) = input.split_once('=')?;
    let key = key.trim();
    let value = value.trim();

    if key.is_empty() || value.is_empty() {
        None
    } else {
        Some((key.into(), value.into()))
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_key_value_pairs() {
        assert_eq!(
            parse_assignment(" theme = dark "),
            Some(("theme".into(), "dark".into()))
        );
        assert_eq!(parse_assignment("missing"), None);
        assert_eq!(parse_assignment("empty=   "), None);
    }
}
