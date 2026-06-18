// Practice module visibility and paths.

mod api {
    // TODO: Add the item needed by the test.
}

fn main() {}

#[cfg(test)]
mod tests { use super::*; #[test] fn module_item_is_reachable() { assert_eq!(api::parent::child::ping(), "pong"); } }
