// Splitting strings produces borrowed pieces. Collect owned, cleaned values
// when the caller should receive independent `String`s.

fn parse_csv_line(line: &str) -> Vec<String> {
    // TODO: Split on commas, trim each field, drop empty fields, and return owned
    // strings.
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_trimmed_fields() {
        assert_eq!(
            parse_csv_line("  red, green,, blue  "),
            ["red", "green", "blue"]
        );
    }
}
