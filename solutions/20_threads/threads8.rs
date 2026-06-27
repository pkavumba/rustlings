use std::sync::mpsc;
use std::thread;
fn collect_messages() -> Vec<String> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        tx.send("a".to_string()).unwrap();
        tx.send("b".to_string()).unwrap();
    })
    .join()
    .unwrap();
    rx.into_iter().collect()
}
fn main() {}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn receives_all() {
        assert_eq!(collect_messages(), ["a", "b"]);
    }
}
