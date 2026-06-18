use std::collections::HashMap;
fn add_score(scores: &mut HashMap<String, u32>, name: &str, points: u32) { // TODO: Add points to name, inserting 0 first if needed.
}
fn main() {}
#[cfg(test)] mod tests { use super::*; #[test] fn updates_entries() { let mut s=HashMap::new(); add_score(&mut s,"Ferris",3); add_score(&mut s,"Ferris",4); assert_eq!(s["Ferris"],7); } }
