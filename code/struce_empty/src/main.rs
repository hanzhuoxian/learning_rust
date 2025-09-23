#[derive(Debug)]
struct SayHello;
trait Hello {
    fn hello();
}
impl Hello for SayHello {
    fn hello() {
        println!("hello");
    }
}
fn main() {
    let subject = SayHello;
    println!("{:?}", subject);
    SayHello::hello();
}
