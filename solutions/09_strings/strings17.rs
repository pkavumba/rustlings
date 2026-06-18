// Build a new string while preserving ownership boundaries between borrowed
// input and owned output.

fn title_case(input: &str) -> String {
    input
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => {
                    let mut titled = String::new();
                    titled.extend(first.to_uppercase());
                    titled.push_str(&chars.as_str().to_lowercase());
                    titled
                }
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
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
