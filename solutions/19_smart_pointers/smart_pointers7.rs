use std::cell::RefCell;
fn push_cell(cell: &RefCell<Vec<i32>>, value: i32) {
    cell.borrow_mut().push(value);
}
fn main() {}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn interior_mutability() {
        let c = RefCell::new(vec![1]);
        push_cell(&c, 2);
        assert_eq!(*c.borrow(), [1, 2]);
    }
}
