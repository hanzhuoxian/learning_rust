fn main() {
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("y = {y}"),
        _ => println!("default case x={:?}", x),
    }

    println!("main y={}", y);
}
