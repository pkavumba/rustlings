// Generic functions can operate on many concrete types.

fn choose_first<T>(left: T, _right: T) -> T {
    left
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn chooses_first_value() {
        assert_eq!(choose_first(1, 2), 1);
        assert_eq!(choose_first("a", "b"), "a");
    }
}
