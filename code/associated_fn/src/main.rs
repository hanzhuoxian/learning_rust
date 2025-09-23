#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Self {
            width: size,
            height: size,
        }
    }
}
fn main() {
    println!("square is {:#?}", Rectangle::square(2));
}
