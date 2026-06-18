// Generic code can use more than one type parameter.

fn pair_to_strings<A: ToString, B: ToString>(a: A, b: B) -> (String, String) {
    // TODO: Convert both values to strings.
}

fn main() {}

#[cfg(test)]
mod tests { use super::*; #[test] fn converts_two_types() { assert_eq!(pair_to_strings(7, true), ("7".into(), "true".into())); } }
