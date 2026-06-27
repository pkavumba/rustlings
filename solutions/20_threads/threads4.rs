use std::thread;
fn run_thread() -> i32 {
    thread::spawn(|| 7).join().unwrap()
}
fn main() {}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn joins_thread() {
        assert_eq!(run_thread(), 7);
    }
}
