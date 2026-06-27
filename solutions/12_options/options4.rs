// `unwrap_or` provides a fallback for `None`.
fn port_or_default(port: Option<u16>) -> u16 {
    port.unwrap_or(8080)
}
fn main() {}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn uses_default_port() {
        assert_eq!(port_or_default(Some(3000)), 3000);
        assert_eq!(port_or_default(None), 8080);
    }
}
