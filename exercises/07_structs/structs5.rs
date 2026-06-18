// Methods can borrow a struct immutably with `&self` or mutably with
// `&mut self`. Pick the receiver that matches what each method needs to do.

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // TODO: Add an associated function `new` that creates a `Rectangle`.

    // TODO: Add an `area` method that reads from `self` without taking ownership.

    // TODO: Add a `scale` method that changes both fields in place.
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn computes_and_mutates_rectangle() {
        let mut rect = Rectangle::new(3, 4);

        assert_eq!(rect.area(), 12);
        rect.scale(2);
        assert_eq!(rect.width, 6);
        assert_eq!(rect.height, 8);
        assert_eq!(rect.area(), 48);
    }
}
