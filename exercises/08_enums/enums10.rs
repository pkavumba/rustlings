// Generic enums can model success and failure. Complete a tiny `Outcome` type
// and transform only its successful value.

#[derive(Debug, PartialEq)]
enum Outcome<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> Outcome<T, E> {
    fn map<U>(self, f: fn(T) -> U) -> Outcome<U, E> {
        // TODO: If `self` is `Ok(value)`, apply `f` to the value. If it is
        // `Err(error)`, keep the error unchanged.
    }
}

fn double(n: i32) -> i32 {
    n * 2
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_only_success_values() {
        let success: Outcome<i32, &str> = Outcome::Ok(21);
        assert_eq!(success.map(double), Outcome::Ok(42));

        let failure: Outcome<i32, &str> = Outcome::Err("boom");
        assert_eq!(failure.map(double), Outcome::Err("boom"));
    }
}
