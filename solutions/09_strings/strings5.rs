// Build an owned `String` from borrowed string slices.

fn greeting(name: &str) -> String {
    format!("Hello, {name}!")
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_owned_greeting() {
        assert_eq!(greeting("Ferris"), "Hello, Ferris!");
    }
}
