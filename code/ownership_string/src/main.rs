fn main() {
    let s1 = String::from("Hello");
    let s2 = s1;
    // println!("{}", s1); s1 变量无效
    let s3 = s2.clone();
    println!("{} {}", s2, s3);
}
