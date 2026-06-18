// Build a new string while preserving ownership boundaries between borrowed
// input and owned output.

fn title_case(input: &str) -> String {
    // TODO: Uppercase the first character of each word and lowercase the rest.
    // Words are separated by ASCII whitespace in this exercise.
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn title_cases_words() {
        assert_eq!(title_case("hELLo rUST"), "Hello Rust");
        assert_eq!(title_case("ferris"), "Ferris");
    }
}
