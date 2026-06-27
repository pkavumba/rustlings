use std::collections::HashMap;
fn invert(map: HashMap<String, String>) -> HashMap<String, String> { // TODO: Swap keys and values.
}
fn main() {}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn swaps_keys_values() {
        let mut m = HashMap::new();
        m.insert("k".into(), "v".into());
        let inv = invert(m);
        assert_eq!(inv["v"], "k");
    }
}
