use std::fmt::Display;

fn main() {
    let a = "abcd";
    let b = "xyz";
    let result = longest(a, b, "hello: ");
    println!("{}", result);
}

fn longest<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    print!("{}", ann);
    if x.len() > y.len() { x } else { y }
}
