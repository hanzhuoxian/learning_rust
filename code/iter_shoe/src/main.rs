use iter_shoe::{Shoe, shoe_in_size};

fn main() {
    let shoe = Shoe {
        size: 10,
        style: String::from("sneaker"),
    };
    let shoes = shoe_in_size(vec![shoe], 10);
    println!("shoes {:?}", shoes);
}
