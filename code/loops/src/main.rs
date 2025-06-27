fn main() {
    let mut number = 0;
    loop {
        number = number + 1;
        if number == 2 {
            continue; // 跳过本次循环
        }
        if number > 4 {
            break; // 退出所有循环
        }
        println!("number is {number}!");
    }
    println!("number loop is end!");

    println!("loop return value start!");
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

}
