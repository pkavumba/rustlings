use std::fmt::Display;
fn stringify_all<T: Display>(items:&[T])->Vec<String>{ items.iter().map(ToString::to_string).collect() }
fn main(){}
#[cfg(test)]mod tests{use super::*;#[test]fn uses_bound(){assert_eq!(stringify_all(&[1,2]),["1","2"]);}}
