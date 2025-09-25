struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn xx(&self) -> &T {
        &self.x
    }

    fn yy(&self) -> &T {
        &self.y
    }
}

impl Point<f32> {
    fn x(&self) -> &f32 {
        &self.x
    }

    fn y(&self) -> &f32 {
        &self.y
    }
}

fn main() {
    let p = Point { x: 1, y: 2 };

    println!("{}", p.xx());
    println!("{}", p.yy());

    let p2 = Point { x: 1.0, y: 2.0 };
    println!("{}", p2.x());
    println!("{}", p2.y());
}
