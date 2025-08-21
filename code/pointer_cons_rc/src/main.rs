use crate::List::{Cons, Nil};
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count: {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a)); // b 会克隆 a 包含的 Rc<List>, 引用计数由 1 变为 2
    println!("count: {}", Rc::strong_count(&a));

    {
        let c = Cons(4, Rc::clone(&a)); // c 会克隆 a 包含的 Rc<List>, 引用计数由 2 变为 3
        println!("c : {c:?}");
        println!("inner count: {}", Rc::strong_count(&a));
    } // c 超出作用域, 引用计数由 3 变成 2
    println!("count: {}", Rc::strong_count(&a));
    println!("b : {b:?}");
}
