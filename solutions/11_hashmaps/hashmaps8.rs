use std::collections::HashMap;
fn merge_counts(a: &mut HashMap<String, usize>, b: HashMap<String, usize>) { for (k,v) in b { *a.entry(k).or_insert(0)+=v; } }
fn main() {}
#[cfg(test)] mod tests { use super::*; #[test] fn merges_maps() { let mut a=HashMap::from([("x".into(),1)]); let b=HashMap::from([("x".into(),2),("y".into(),3)]); merge_counts(&mut a,b); assert_eq!(a["x"],3); assert_eq!(a["y"],3); } }
