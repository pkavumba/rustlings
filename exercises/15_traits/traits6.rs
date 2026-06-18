trait Summary { fn summarize(&self) -> String; }
struct Article { title: String }
impl Summary for Article { // TODO: Implement summarize.
}
fn main(){}
#[cfg(test)]mod tests{use super::*;#[test]fn summarizes(){assert_eq!(Article{title:"Rust".into()}.summarize(),"Article: Rust");}}
