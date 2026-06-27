// Practice module visibility and paths.

mod api {
    pub fn answer() -> u32 {
        42
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn module_item_is_reachable() {
        assert_eq!(api::answer(), 42);
    }
}
