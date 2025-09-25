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
    let rect = Rectangle::square(2);
    println!("square x: {} y: {} is {:#?}", rect.width, rect.height, rect);
}
