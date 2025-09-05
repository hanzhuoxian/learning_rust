unsafe extern "C" {
   unsafe fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -5 is {}", abs(-5));
    }
}
