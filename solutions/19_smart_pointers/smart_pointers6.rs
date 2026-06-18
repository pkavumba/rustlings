use std::rc::Rc;
fn clone_counted(value: Rc<String>) -> (Rc<String>, Rc<String>) { (Rc::clone(&value), value) }
fn main(){}
#[cfg(test)]mod tests{use super::*;#[test]fn rc_counts(){let rc=Rc::new("hi".to_string());let (a,b)=clone_counted(rc);assert!(Rc::ptr_eq(&a,&b));}}
