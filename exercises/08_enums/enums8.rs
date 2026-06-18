// Methods are useful on enums too. Use `self`, `&self`, and `&mut self` where
// each method needs a different kind of access.

#[derive(Debug, PartialEq)]
enum Door {
    Open { knocks: u8 },
    Closed { knocks: u8 },
    Locked,
}

impl Door {
    fn is_open(&self) -> bool {
        // TODO: Return true only for `Door::Open { .. }`.
    }

    fn knock(&mut self) {
        // TODO: Increment `knocks` for Open and Closed doors. Locked doors do
        // not record knocks.
    }

    fn close(self) -> Self {
        // TODO: Consume the door. Open doors become Closed while preserving
        // knocks. Other states are returned unchanged.
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enum_methods_choose_receivers() {
        let mut door = Door::Open { knocks: 0 };
        assert!(door.is_open());

        door.knock();
        door.knock();
        assert_eq!(door, Door::Open { knocks: 2 });

        door = door.close();
        assert_eq!(door, Door::Closed { knocks: 2 });
        assert!(!door.is_open());

        door.knock();
        assert_eq!(door, Door::Closed { knocks: 3 });

        let mut locked = Door::Locked;
        locked.knock();
        assert_eq!(locked.close(), Door::Locked);
    }
}
