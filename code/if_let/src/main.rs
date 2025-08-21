fn main() {
    let x = Some(3u8);

    match x {
        Some(s) => {
            println!("{}", s);
        }
        _ => (),
    }

    if let Some(s) = x { // 是 match 的语法糖
        println!("{}", s);
    }
}
