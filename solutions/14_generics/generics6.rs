// `where` clauses can make complex bounds easier to read.
use std::fmt::Display;

fn surround<T>(value: T, left: char, right: char) -> String
where
    T: Display,
{
    format!("{left}{value}{right}")
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn formats_display_values() {
        assert_eq!(surround(10, '[', ']'), "[10]");
        assert_eq!(surround("hi", '<', '>'), "<hi>");
    }
}
