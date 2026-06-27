struct Wrapper(String);
impl std::ops::Deref for Wrapper {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        &self.0
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
