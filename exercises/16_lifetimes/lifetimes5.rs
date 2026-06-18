struct View<'a> { text: &'a str }
fn make_view(text: &str) -> View<'_> { // TODO: Store text in View.
}
fn main(){}
#[cfg(test)] mod tests{use super::*;#[test]fn stores_borrow(){assert_eq!(make_view("hi").text,"hi");}}
