fn bar() -> ! {
    panic!("This function never returns!");
}
fn main() {
    bar();
}
