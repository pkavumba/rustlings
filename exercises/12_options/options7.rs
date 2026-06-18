// `take` moves a value out of an Option, leaving None behind.
struct Todo { current: Option<String> }
impl Todo { fn finish_current(&mut self) -> Option<String> { // TODO: Take the current task.
} }
fn main() {}
#[cfg(test)] mod tests { use super::*; #[test] fn takes_optional_field() { let mut todo = Todo { current: Some("learn".into()) }; assert_eq!(todo.finish_current(), Some("learn".into())); assert_eq!(todo.current, None); } }
