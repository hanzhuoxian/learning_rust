fn main() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let hello1 = &s[..5]; // 与  &s[0..5] 等价
    let world = &s[6..11];
    let world1 = &s[6..]; // 与 &s[6..11] 等价
    println!("{hello} {hello1} {world} {world1}")
}
