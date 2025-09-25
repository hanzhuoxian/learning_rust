// # Safety: This code demonstrates the use of unsafe traits in Rust.
#[allow(clippy::missing_safety_doc)]
unsafe trait Foo {
    // methods go here
    fn bar(&self);
}

unsafe impl Foo for i32 {
    // method implementations go here
    fn bar(&self) {
        println!("i32 bar");
    }
}

fn main() {
    let x: &dyn Foo = &5;
    x.bar();
    println!("Hello, world!");
}
