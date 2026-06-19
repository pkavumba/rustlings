// Learn how `serde` maps JSON data onto Rust structs.

use serde::Deserialize;

#[derive(Debug, PartialEq)]
// TODO: Derive `Deserialize` for this struct.
struct AppConfig {
    app_name: String,
    retries: u8,
    features: Vec<String>,
}

fn parse_config(input: &str) -> serde_json::Result<AppConfig> {
    // TODO: Deserialize `input` into an `AppConfig`.
    todo!()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_json_config() {
        let config = parse_config(
            r#"{
                "app_name": "rustlings",
                "retries": 3,
                "features": ["watch", "hint"]
            }"#,
        )
        .unwrap();

        assert_eq!(config.app_name, "rustlings");
        assert_eq!(config.retries, 3);
        assert_eq!(config.features, ["watch", "hint"]);
    }
}
