fn main() {
    let v = ['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is iat index {}", value, index);
    }
}
