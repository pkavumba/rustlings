// Advanced generic APIs often combine bounds and closures.

fn apply_twice<T, F>(value: T, f: F) -> T
where
    F: Fn(T) -> T,
{
    // TODO: Apply `f` to `value`, then apply `f` to the result.
}

fn main() {}

#[cfg(test)]
mod tests { use super::*; #[test] fn applies_closure_twice() { assert_eq!(apply_twice(3, |n| n + 1), 5); assert_eq!(apply_twice(String::from("a"), |mut s| { s.push('!'); s }), "a!!"); } }
