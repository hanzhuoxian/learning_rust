fn main() {
    let number = 6;
    // if 只会执行第一个分支为 true 的代码
    if number % 4 == 0 {
        println!("number {} is divisible by 4", number);
    } else if number % 3 == 0 {
        println!("number {} is divisible by 3", number);
    } else if number % 2 == 0 {
        println!("number {} is divisible by 2", number);
    } else {
        println!("number {} is not divisible by 4,3, or 2", number);
    }

    let condition = false;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
}
