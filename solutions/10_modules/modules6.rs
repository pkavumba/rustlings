// Practice module visibility and paths.

mod api {
    pub(crate) fn hidden() -> bool {
        true
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn module_item_is_reachable() {
        assert!(api::hidden());
    }
}
