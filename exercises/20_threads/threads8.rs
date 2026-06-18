use std::sync::mpsc; use std::thread;
fn collect_messages()->Vec<String>{ // TODO: Send two messages from a spawned thread and collect them.
}
fn main(){}
#[cfg(test)]mod tests{use super::*;#[test]fn receives_all(){assert_eq!(collect_messages(),["a","b"]);}}
