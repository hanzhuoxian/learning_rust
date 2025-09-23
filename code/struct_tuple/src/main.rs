// 声明元组结构体
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
fn main() {
    // 实例化元组结构体
    let black = Color(0, 0, 0);
    let point = Point(1, 1, 1);

    println!("{} {} {}", black.0, black.1, black.2);
    println!("{} {} {}", point.0, point.1, point.2);
}
