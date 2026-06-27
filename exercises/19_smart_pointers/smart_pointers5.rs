fn boxed_value(n: i32) -> Box<i32> { // TODO: Put n on the heap with Box.
}
fn main() {}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn boxes_value() {
        assert_eq!(*boxed_value(5), 5);
    }
}
