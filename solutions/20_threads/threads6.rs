use std::sync::mpsc;
fn send_message() -> String {
    let (tx, rx) = mpsc::channel();
    tx.send("hello".to_string()).unwrap();
    rx.recv().unwrap()
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
