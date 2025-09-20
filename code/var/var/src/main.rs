fn main() {
    let x: i32 = 10; // 声明不可变变量
    println!("{}", x); // 10
    // x = 20; // 为不可变变量赋值-编译错误

    let mut x: i32 = 10; // 可变变量
    println!("{}", x); // 10
    x = 20; // 为可变变量赋值
    println!("{}", x); // 20

    const THREE_HOUR_IN_SECONDS: u32 = 3 * 60 * 60;
    println!("{}", THREE_HOUR_IN_SECONDS); // 10800
}
