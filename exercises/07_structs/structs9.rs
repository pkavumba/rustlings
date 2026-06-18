// Generic structs let the same shape hold different data types. Complete this
// tiny pair type and its methods.

#[derive(Debug, PartialEq)]
struct Pair<T> {
    left: T,
    right: T,
}

impl<T> Pair<T> {
    fn new(left: T, right: T) -> Self {
        // TODO: Create a pair from the two values.
    }

    fn swap(self) -> Self {
        // TODO: Return a new pair with left and right exchanged.
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pairs_any_single_type() {
        let numbers = Pair::new(1, 2).swap();
        assert_eq!(numbers, Pair { left: 2, right: 1 });

        let words = Pair::new(String::from("left"), String::from("right")).swap();
        assert_eq!(words.left, "right");
        assert_eq!(words.right, "left");
    }
}
