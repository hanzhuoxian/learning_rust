fn main() {
    unsafe fn dangerous() {
        let num = 5;
        let r = &num as *const i32;
        unsafe {
            println!("dangerous is {}", *r);
        }
    }
    unsafe {
        dangerous();
    }
}
