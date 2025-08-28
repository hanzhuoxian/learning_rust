fn main() {
    let s = Some(5);
    // let Some(x) = s;
    if let Some(x) = s {
        println!("{x}");
    }

    if let x = 5 {
        println!("{x}");
    }
}
