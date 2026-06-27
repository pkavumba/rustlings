// Trait bounds let generic code require specific behavior.

fn largest<T: Ord + Copy>(items: &[T]) -> T {
    // TODO: Return the largest item in a non-empty slice.
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn finds_largest_generic_value() {
        assert_eq!(largest(&[3, 7, 2]), 7);
        assert_eq!(largest(&['a', 'z', 'm']), 'z');
    }
}
