fn main() {
    println!("Hello, world!");
    another_function();
    another_function_x(5);
    print_labeled_measurement(5, 'h');
    println!("The value of x is: {}", five());
    println!("The value of plus_one is: {}", plus_one(1));
}

// 无参数函数
fn another_function() {
    println!("Another function.");
}

// 单参数函数
fn another_function_x(x: i32) {
    println!("The value of x is: {}", x)
}

// 多参数函数
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {} {}", value, unit_label);
}

// 具有返回值的函数
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
