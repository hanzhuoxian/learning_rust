fn main() {
    // 元组
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{} {} {}", tup.0, tup.1, tup.2);

    // 使用模式匹配来解构元组
    let (x, y, z) = tup;
    println!("{} {} {}", x, y, z);
}
