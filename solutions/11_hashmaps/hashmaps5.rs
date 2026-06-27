use std::collections::HashMap;
fn add_score(scores: &mut HashMap<String, u32>, name: &str, points: u32) {
    *scores.entry(name.to_string()).or_insert(0) += points;
}
fn main() {}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn updates_entries() {
        let mut s = HashMap::new();
        add_score(&mut s, "Ferris", 3);
        add_score(&mut s, "Ferris", 4);
        assert_eq!(s["Ferris"], 7);
    }
}
