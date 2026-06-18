struct Reader<'a>{remaining:&'a str}
impl<'a> Reader<'a>{fn take_all(&mut self)->&'a str{ // TODO: Return remaining and empty it.
}}
fn main(){}
#[cfg(test)]mod tests{use super::*;#[test]fn takes_all(){let mut r=Reader{remaining:"abc"};assert_eq!(r.take_all(),"abc");assert_eq!(r.remaining,"");}}
