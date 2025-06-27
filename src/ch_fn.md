# 函数

`fn` 后面跟着函数名和一对圆括号来定义函数。大括号告诉编译器哪里式函数的开始和结尾。使用函数名后跟圆括号来调用任意函数。

有参数的函数，参数是特殊变量。是函数签名的一部分。当参数拥有形参时，可以为这些参数提供具体的值实参。

函数体由一系列的语句和一个可选的结尾表达式构成。
语句：执行一些操作但不返回值的指令。
表达式：计算并产生一个值。

返回值：

无需对返回值进行命名，但是需要在 `->` 后声明返回值的类型。函数的返回值等同于最后一个表达式的值。使用 `return` 和指定值，可以从函数中提前返回。

```rust
fn main() {
    println!("Hello, world!");
    another_function();
    another_function_x(5);
    print_labeled_measurement(5, 'h');
    println!("The value of x is: {}", five());
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

// 具有返回值的函数
fn five() -> i32 {
    return 5
}

```
