fn foo(_: i32, x: i32) {
    println!("x: {x}");
}
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    foo(3, 4);

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value!");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting_value is {:?}", setting_value);

    let _a = 5; // 忽略未使用变量警告

    let origin = Point { x: 0, y: 0, z: 0 };
    let Point { x, .. } = origin; // 用 .. 忽略剩余值
    println!("x: {}", x);
}
