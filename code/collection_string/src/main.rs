fn main() {

    let s = "Hello";
    let s = s.to_string();
    println!("{s}");

    let s = String::from("Hello");
    println!("{s}");


    let mut s = String::new();
    s.push_str("Hello");
    s.push('!');

    let hello = String::from("hello");
    let world = String::from("world");
    let hello_world = hello + &world;
    // + 号之后 hello 将被移动
    println!("{hello_world}");


    let hello = String::from("hello");
    let world = String::from("world");
    let hello_world = format!("{}{}", hello, world);
    println!("{hello_world}");


}
