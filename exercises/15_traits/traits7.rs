trait Named { fn name(&self)->&str; fn greeting(&self)->String{ format!("Hello, {}", self.name()) } }
struct User(String);
impl Named for User { // TODO: Implement name.
}
fn main(){}
#[cfg(test)]mod tests{use super::*;#[test]fn default_method(){assert_eq!(User("Ferris".into()).greeting(),"Hello, Ferris");}}
