fn main() {
    let s1 = gives_ownership();
    println!("{s1}");
    let s2 = String::from("hello"); // s2 进入作用域
    let s3 = takes_and_gives_back(s2); // s2 被移动到函数中它也将值返回值移动给 s3。
    // println!("{s2}");
    println!("{s3}");
} // s1、s3 移出作用域，s2 也移出作用域，但是已经被移走什么都不会发生。

#[allow(clippy::let_and_return)]
fn gives_ownership() -> String {
    // 将返回值移动值调用他的函数
    let some_string = String::from("yours"); // some_string 进入作用域
    some_string // 返回 some_string 并移出给调用它的函数
}

fn takes_and_gives_back(s: String) -> String {
    s
}
