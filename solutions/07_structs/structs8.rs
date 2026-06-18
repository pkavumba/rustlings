// Some methods consume `self` and return an updated value. This style is common
// in builder APIs because calls can be chained.

#[derive(Debug, PartialEq)]
struct Sandwich {
    bread: String,
    filling: String,
    toasted: bool,
}

impl Sandwich {
    fn new(bread: String) -> Self {
        Self {
            bread,
            filling: String::new(),
            toasted: false,
        }
    }

    fn with_filling(mut self, filling: String) -> Self {
        self.filling = filling;
        self
    }

    fn toast(mut self) -> Self {
        self.toasted = true;
        self
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_by_chaining_consuming_methods() {
        let sandwich = Sandwich::new(String::from("sourdough"))
            .with_filling(String::from("tomato"))
            .toast();

        assert_eq!(
            sandwich,
            Sandwich {
                bread: String::from("sourdough"),
                filling: String::from("tomato"),
                toasted: true,
            },
        );
    }
}
