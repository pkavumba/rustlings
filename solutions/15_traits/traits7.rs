trait Named { fn name(&self)->&str; fn greeting(&self)->String{ format!("Hello, {}", self.name()) } }
struct User(String);
impl Named for User { fn name(&self)->&str { &self.0 } }
fn main(){}
#[cfg(test)]mod tests{use super::*;#[test]fn default_method(){assert_eq!(User("Ferris".into()).greeting(),"Hello, Ferris");}}
