fn main() {
    let s = String::from("Hello World!");
    takes_ownership(s); // 此行过后 s 变量已经被无效
    // println!("{}", s); // 编译错误： borrow of moved value: `s` value borrowed here after move
    let x = 5;
    makes_copy(x);
    println!("{}", x);
}
fn takes_ownership(ss: String) {
    println!("{}", ss);
} // 调用 ss.drop 释放内存

fn makes_copy(y: i32) {
    println!("{}", y);
} // 这里 y 移出作用域，没有特殊之处。
