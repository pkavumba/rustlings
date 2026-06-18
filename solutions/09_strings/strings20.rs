// Put many string skills together: search, slicing, allocation, and repeated
// replacement of simple placeholders.

fn render_template(template: &str, name: &str, language: &str) -> String {
    template
        .replace("{name}", name)
        .replace("{language}", language)
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_known_placeholders() {
        assert_eq!(
            render_template("Hi {name}, keep learning {language}!", "Ferris", "Rust"),
            "Hi Ferris, keep learning Rust!",
        );
        assert_eq!(render_template("{name} + {name}", "crab", "Rust"), "crab + crab");
    }
}
