fn main() {
    // 数组越界访问 panic
    let a = vec![1,2,3];
    println!("{}", a[4]);

    // 手动调用 panic!
    panic!("crash and burn");
}
