struct Split<'a>{left:&'a str,right:&'a str}
fn split_once_colon(s:&str)->Option<Split<'_>>{ let (left,right)=s.split_once(':')?; Some(Split{left,right}) }
fn main(){}
#[cfg(test)]mod tests{use super::*;#[test]fn splits_borrowed(){let s=split_once_colon("a:b").unwrap();assert_eq!(s.left,"a");assert_eq!(s.right,"b");}}
