use std::thread;
fn move_to_thread(text:String)->String{ // TODO: Move text into a thread and return it.
}
fn main(){}
#[cfg(test)]mod tests{use super::*;#[test]fn moves_ownership(){assert_eq!(move_to_thread("rust".into()),"rust");}}
