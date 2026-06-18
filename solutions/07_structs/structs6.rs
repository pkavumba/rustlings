// Derived traits can make structs easier to compare, clone, print and create
// with defaults. Complete the derives and constructor below.

#[derive(Debug, Clone, PartialEq, Default)]
struct Settings {
    volume: u8,
    notifications: bool,
    theme: String,
}

impl Settings {
    fn quiet_dark_mode() -> Self {
        Self {
            volume: 10,
            theme: String::from("dark"),
            ..Self::default()
        }
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derives_and_updates_defaults() {
        let default = Settings::default();
        assert_eq!(default.volume, 0);
        assert!(!default.notifications);
        assert_eq!(default.theme, "");

        let quiet = Settings::quiet_dark_mode();
        assert_eq!(quiet.volume, 10);
        assert!(!quiet.notifications);
        assert_eq!(quiet.theme, "dark");

        let cloned = quiet.clone();
        assert_eq!(quiet, cloned);
        assert!(format!("{quiet:?}").contains("Settings"));
    }
}
