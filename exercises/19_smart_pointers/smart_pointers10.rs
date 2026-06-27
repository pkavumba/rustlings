struct Wrapper(String);
impl std::ops::Deref for Wrapper {
    type Target = str;
    fn deref(&self) -> &Self::Target { // TODO: Deref to inner str.
    }
}
fn main() {}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn deref_coerces() {
        let w = Wrapper("rust".into());
        assert_eq!(w.len(), 4);
    }
}
