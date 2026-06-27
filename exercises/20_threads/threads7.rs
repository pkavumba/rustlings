use std::sync::{Arc, Mutex};
use std::thread;
fn threaded_increment() -> i32 { // TODO: Spawn two threads that increment shared counter.
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
