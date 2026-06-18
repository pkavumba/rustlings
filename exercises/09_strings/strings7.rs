// `format!` creates a new `String` without taking ownership of borrowed
// inputs. Use it when you need interpolation.

fn full_name(first: &str, last: &str) -> String {
    // TODO: Return "{last}, {first}".
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_without_moving_inputs() {
        let first = String::from("Grace");
        let last = String::from("Hopper");

        assert_eq!(full_name(&first, &last), "Hopper, Grace");
        assert_eq!(first, "Grace");
        assert_eq!(last, "Hopper");
    }
}
