use crate::List::{Cons, Nil};
use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

fn main() {
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() = 10;

    println!("{a:?}");
    println!("{b:?}");
    println!("{c:?}");

    if let Cons(ref_cell, _) = b {
        let inner_value = *ref_cell.borrow();
        println!("inner_value: {inner_value}");
    }
    println!("{:?}", c);
}
