// Practice module visibility and paths.

mod api {
    pub mod parent {
        pub mod child {
            pub fn ping() -> &'static str {
                "pong"
            }
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn module_item_is_reachable() {
        assert_eq!(api::parent::child::ping(), "pong");
    }
}
