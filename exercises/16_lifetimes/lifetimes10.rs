use std::fmt::Display;
fn announce<'a, T: Display>(label: &'a str, value: T) -> String { // TODO: Format label and value.
}
fn main(){}
#[cfg(test)]mod tests{use super::*;#[test]fn combines_lifetime_and_bound(){assert_eq!(announce("score",9),"score: 9");}}
