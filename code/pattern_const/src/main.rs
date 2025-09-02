fn main() {
    let x = 7;
    match x {
        1 => println!("one"),               // 普通字面值
        2 | 3 => println!("two or three"),  // 多个模式
        4..=5 => println!("4-5"),           // 范围范围-右闭区间
        6..7 => println!("6"),              // 匹配范围-右开区间
        other => println!("other {other}"), // 匹配剩余并绑定变量
        _ => println!("other"),             // 匹配剩余不绑定变量
    }
}
