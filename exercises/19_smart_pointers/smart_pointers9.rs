use std::sync::{Arc, Mutex};
fn increment(shared: &Arc<Mutex<i32>>) { // TODO: Lock and increment.
}
fn main() {}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn shared_mutex() {
        let v = Arc::new(Mutex::new(0));
        increment(&v);
        assert_eq!(*v.lock().unwrap(), 1);
    }
}
