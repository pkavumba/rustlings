struct Counter { next: u32, end: u32 }
impl Iterator for Counter { type Item = u32; fn next(&mut self) -> Option<Self::Item> { if self.next > self.end { None } else { let value=self.next; self.next += 1; Some(value) } } }
fn main(){}
#[cfg(test)]mod tests{use super::*;#[test]fn custom_iterator(){assert_eq!(Counter{next:1,end:3}.collect::<Vec<_>>(),[1,2,3]);}}
