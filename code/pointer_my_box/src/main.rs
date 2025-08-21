use std::ops::Deref;

#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> Self {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T; // 关联类型

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let t = 3;
    let three = MyBox::new(t);
    assert_eq!(3, three.0);
    assert_eq!(3, *three); // 实际执行了 *(three.deref())

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}


fn hello(name: &str) {
    println!("hello {}!", name)
}