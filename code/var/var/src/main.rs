fn main() {
    let mut x: i32 = 10;
    println!("{}", x); // 10
    x = 20;
    println!("{}", x); // 2

    const THREE_HOUR_IN_SECONDS: u32 = 3 * 60 * 60;
    println!("{}", THREE_HOUR_IN_SECONDS); // 10800
}
