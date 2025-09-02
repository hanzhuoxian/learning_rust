fn main() {
    // let num = Some(6);
    let num = None::<i32>;
    match num {
        Some(x) if x < 5 => println!("less than five: {x}"),
        Some(x) => println!("{x}"),
        None => (),
    }
}
