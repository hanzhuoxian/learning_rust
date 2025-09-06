use std::ops::Add;

struct Millimeters(u32);
struct Meters(u32);

impl  Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
    
}
fn main() {
    let one_meter = Meters(1);
    let two_millimeters = Millimeters(2);
    let result = two_millimeters + one_meter;
    println!("Result: {} mm", result.0);
}
