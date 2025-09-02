struct Point {
    x: u32,
    y: u32,
}
fn main() {
    let point = Point { x: 1, y: 0 };
    let Point { x: a, y: b } = point;
    println!("a = {} b = {}", a, b);

    let Point { x, y } = point;
    println!("x = {} y = {}", x, y);

    match point {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => println!("On neither axis: {x}, {y}"),
    }
}
