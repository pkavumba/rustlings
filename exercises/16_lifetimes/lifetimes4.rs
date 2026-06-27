fn longer<'a>(a: &'a str, b: &'a str) -> &'a str { // TODO: Return the longer string slice.
}
fn main() {}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn returns_longer() {
        assert_eq!(longer("rust", "crabacean"), "crabacean");
    }
}
