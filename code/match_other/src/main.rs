

fn main() {
    let dice_roll = 9;
    let dice = match dice_roll {
        1..=8 => 8, // 匹配一个范围
        9 => 9, // 匹配单独的值
        other => other, // 匹配所有未匹配的值
        // _ => 0, // 匹配所有未匹配的值但是不需要使用该值
    };
    println!("{}", dice);
}
