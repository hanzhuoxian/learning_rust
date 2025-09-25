fn plus_one(x: Option<i32>) -> Option<i32> {
    #[allow(clippy::manual_map)]
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{}", six.unwrap());
    if Option::is_none(&none) {
        println!("None");
    }
}
