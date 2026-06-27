fn longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
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
