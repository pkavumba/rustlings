// Generic methods can introduce their own type parameters.

struct Pair<T> {
    left: T,
    right: T,
}

impl<T> Pair<T> {
    fn zip<U>(self, other: Pair<U>) -> Pair<(T, U)> {
        // TODO: Pair up the left values and the right values.
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn zips_pairs_of_different_types() {
        let p = Pair { left: 1, right: 2 }.zip(Pair {
            left: "a",
            right: "b",
        });
        assert_eq!(p.left, (1, "a"));
        assert_eq!(p.right, (2, "b"));
    }
}
