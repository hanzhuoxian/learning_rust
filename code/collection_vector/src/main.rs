fn main() {
    // 新建空列表
    let mut v: Vec<i32> = Vec::new();
    // 向列表插入值
    v.push(1);
    // 从列表中取出值
    match v.pop() {
        Some(i) => {
            println!("{}", i)
        }
        None => {}
    }

    // 使用初始值初始化列表
    let v = vec![1, 2, 3, 4, 5];
    // 使用索引值访问列表，超过索引值将 panic
    println!("{}", v[2]);
    // 使用 get 方法访问值，超过将返回 None
    match v.get(100) {
        Some(i) => {
            println!("{}", i);
        }
        None => {
            println!("none");
        }
    }

    // 遍历元素
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![1, 2, 3];
    for i in &mut v {
        *i += 50;
    }
    for i in &v {
        println!("{}", i);
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let cells = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(3.4),
        SpreadsheetCell::Text(String::from("rust")),
    ];

    // 丢弃 vec 时的中元素也被丢弃
}
