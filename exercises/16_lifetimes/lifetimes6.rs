fn first_nonempty<'a>(items: &[&'a str]) -> Option<&'a str> { // TODO: Return first non-empty slice.
}
fn main() {}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn finds_borrow() {
        assert_eq!(first_nonempty(&["", "ok"]), Some("ok"));
    }
}
