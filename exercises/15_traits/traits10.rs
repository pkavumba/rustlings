trait Parser { type Output; fn parse(&self, input:&str)->Self::Output; }
struct Length;
impl Parser for Length { type Output=usize; fn parse(&self,input:&str)->Self::Output{ // TODO: Return input length.
} }
fn main(){}
#[cfg(test)]mod tests{use super::*;#[test]fn associated_type(){assert_eq!(Length.parse("rust"),4);}}
