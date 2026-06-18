// Generic structs store values whose concrete type is chosen by the caller.

struct Slot<T> { value: T }

impl<T> Slot<T> { fn new(value: T) -> Self { // TODO: Store `value` in a Slot.
} }

fn main() {}

#[cfg(test)]
mod tests { use super::*; #[test] fn stores_any_type() { assert_eq!(Slot::new(42).value, 42); assert_eq!(Slot::new(String::from("rust")).value, "rust"); } }
