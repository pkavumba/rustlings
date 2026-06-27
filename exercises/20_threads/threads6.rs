use std::sync::mpsc;
fn send_message() -> String { // TODO: Send "hello" through a channel.
}
fn main() {}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn channel_message() {
        assert_eq!(send_message(), "hello");
    }
}
