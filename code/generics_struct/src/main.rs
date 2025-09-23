#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}
fn main() {
    let p = Point { x: 1, y: 2 };
    println!("{:?} x={}, y={}", p, p.x, p.y);
    let p = Point { x: 1.1, y: 2.2 };
    println!("{:?} x={}, y={}", p, p.x, p.y);
}
