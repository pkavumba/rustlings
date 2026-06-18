// Enum variants can carry different shapes of data. Destructure each variant
// in a match arm and use its fields to compute the total area.

#[derive(Debug, PartialEq)]
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Square(f64),
}

fn area(shape: Shape) -> f64 {
    const PI: f64 = 3.141592653589793;

    match shape {
        Shape::Circle { radius } => PI * radius * radius,
        Shape::Rectangle { width, height } => width * height,
        Shape::Square(side) => side * side,
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close_enough(left: f64, right: f64) {
        assert!((left - right).abs() < 0.000_001, "{left} != {right}");
    }

    #[test]
    fn destructures_data_variants() {
        close_enough(area(Shape::Circle { radius: 2.0 }), 12.566_370_614_359_172);
        close_enough(area(Shape::Rectangle { width: 3.0, height: 4.0 }), 12.0);
        close_enough(area(Shape::Square(5.0)), 25.0);
    }
}
