fn main() {
    let s = Some(5);
    // let Some(x) = s;
    if let Some(x) = s {
        println!("{x}");
    }

    #[allow(irrefutable_let_patterns)]
    if let x = 5 {
        println!("{x}");
    }
}
