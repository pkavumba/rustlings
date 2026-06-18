fn boxed_value(n:i32)->Box<i32>{ Box::new(n) }
fn main(){}
#[cfg(test)]mod tests{use super::*;#[test]fn boxes_value(){assert_eq!(*boxed_value(5),5);}}
