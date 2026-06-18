struct Counter { next: u32, end: u32 }
impl Iterator for Counter { type Item = u32; fn next(&mut self) -> Option<Self::Item> { // TODO: Yield numbers from next through end inclusive.
} }
fn main(){}
#[cfg(test)]mod tests{use super::*;#[test]fn custom_iterator(){assert_eq!(Counter{next:1,end:3}.collect::<Vec<_>>(),[1,2,3]);}}
