// Generic enums can model values that may or may not be ready.

#[derive(Debug, PartialEq)]
enum MaybeReady<T> { Ready(T), Pending }

impl<T> MaybeReady<T> { fn into_option(self) -> Option<T> { // TODO: Convert Ready to Some and Pending to None.
} }

fn main() {}

#[cfg(test)]
mod tests { use super::*; #[test] fn converts_generic_enum() { assert_eq!(MaybeReady::Ready(5).into_option(), Some(5)); let pending: MaybeReady<i32> = MaybeReady::Pending; assert_eq!(pending.into_option(), None); } }
