// Practice module visibility and paths.

mod api { pub const NAME: &str = "rust"; }

fn main() {}

#[cfg(test)]
mod tests { use super::*; #[test] fn module_item_is_reachable() { assert_eq!(api::NAME, "rust"); } }
