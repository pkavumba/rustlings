use std::thread;
fn run_thread() -> i32 { // TODO: Spawn a thread that returns 7 and join it.
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
