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

impl<f32> Point<f32> {
    fn x(&self) -> &f32 {
        &self.x
    }

    fn y(&self) -> &f32 {
        &self.y
    }
}

fn main() {
    let p = Point{x: 1, y: 2};

    println!("{}", p.x());
    println!("{}", p.yy());
}
