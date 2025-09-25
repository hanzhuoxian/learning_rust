fn main() {
    let a = [1, 2, 3];
    print!("array {:?}", a);

    // 数组越界访问 panic
    // println!("{}", a[4]);

    // 手动调用 panic!
    panic!("crash and burn");
}
