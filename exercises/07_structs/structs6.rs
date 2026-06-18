// Derived traits can make structs easier to compare, clone, print and create
// with defaults. Complete the derives and constructor below.

// TODO: Add derives so `Settings` can be debug-printed, cloned, compared for
// equality, and created with `Settings::default()`.
struct Settings {
    volume: u8,
    notifications: bool,
    theme: String,
}

impl Settings {
    fn quiet_dark_mode() -> Self {
        // TODO: Start from the default settings with struct update syntax,
        // changing only `volume` and `theme`.
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
