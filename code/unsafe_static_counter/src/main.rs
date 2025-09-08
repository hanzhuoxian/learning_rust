static mut COUNTER: i32 = 0;
fn add_to_count(inc: i32) {
    unsafe {
        COUNTER += inc;
    }
}


fn main() {
    add_to_count(3);
    unsafe {
        // println!("COUNTER: {}", COUNTER);
    }
}
