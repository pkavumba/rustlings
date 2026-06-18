// Advanced structs can borrow data instead of owning it. The lifetime parameter
// says that every borrowed field in the struct must remain valid long enough.

#[derive(Debug, PartialEq)]
struct Highlight<'a> {
    // TODO: Add fields for `text` and `tag` that borrow string slices with the
    // same lifetime as the struct.
}

impl<'a> Highlight<'a> {
    fn new(text: &'a str, tag: &'a str) -> Self {
        // TODO: Store both borrowed string slices.
    }

    fn render(&self) -> String {
        format!("[{}] {}", self.tag, self.text)
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn borrows_text_without_taking_ownership() {
        let note = String::from("review lifetimes");
        let tag = String::from("todo");
        let highlight = Highlight::new(note.as_str(), tag.as_str());

        assert_eq!(highlight.text, "review lifetimes");
        assert_eq!(highlight.tag, "todo");
        assert_eq!(highlight.render(), "[todo] review lifetimes");
        assert_eq!(note, "review lifetimes");
        assert_eq!(tag, "todo");
    }
}
