// Recursive enums need indirection because Rust must know the size of every
// type at compile time. Use `Box` to give the recursive variant a known size.

#[derive(Debug, PartialEq)]
enum IntList {
    Empty,
    Node(i32, Box<IntList>),
}

impl IntList {
    fn len(&self) -> usize {
        // TODO: Return the number of nodes in the list.
    }

    fn sum(&self) -> i32 {
        // TODO: Return the sum of all values in the list.
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn traverses_recursive_enum() {
        let list = IntList::Node(
            10,
            Box::new(IntList::Node(
                -2,
                Box::new(IntList::Node(7, Box::new(IntList::Empty))),
            )),
        );

        assert_eq!(list.len(), 3);
        assert_eq!(list.sum(), 15);
    }
}
