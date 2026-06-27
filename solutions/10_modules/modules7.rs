// Practice module visibility and paths.

mod api {
    pub use inner::value;
    mod inner {
        pub fn value() -> u8 {
            7
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn module_item_is_reachable() {
        assert_eq!(api::value(), 7);
    }
}
