use std::sync::{Arc, Mutex};
use std::thread;
fn threaded_increment() -> i32 {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();
    for _ in 0..2 {
        let c = Arc::clone(&counter);
        handles.push(thread::spawn(move || *c.lock().unwrap() += 1));
    }
    for h in handles {
        h.join().unwrap();
    }
    *counter.lock().unwrap()
}
fn main() {}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn increments_from_threads() {
        assert_eq!(threaded_increment(), 2);
    }
}
