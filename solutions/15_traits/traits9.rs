trait Area {
    fn area(&self) -> u32;
}
struct Square(u32);
fn total_area(shapes: &[Box<dyn Area>]) -> u32 {
    shapes.iter().map(|s| s.area()).sum()
}
impl Area for Square {
    fn area(&self) -> u32 {
        self.0 * self.0
    }
}
fn main() {}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn uses_trait_objects() {
        let v: Vec<Box<dyn Area>> = vec![Box::new(Square(2)), Box::new(Square(3))];
        assert_eq!(total_area(&v), 13);
    }
}
