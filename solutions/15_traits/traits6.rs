trait Summary { fn summarize(&self) -> String; }
struct Article { title: String }
impl Summary for Article { fn summarize(&self) -> String { format!("Article: {}", self.title) } }
fn main(){}
#[cfg(test)]mod tests{use super::*;#[test]fn summarizes(){assert_eq!(Article{title:"Rust".into()}.summarize(),"Article: Rust");}}
