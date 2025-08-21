struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, p: Point<X2, Y2>) -> Point<X1, Y2> {
        Point { x: self.x, y: p.y }
    }
}
fn main() {
    let p = Point { x: 1, y: 1.1 };
    let p2 = Point { x: "hello", y: 'y' };
    let p3 = p.mixup(p2);
    println!("{} {}", p3.x, p3.y)
}
