use std::arch::x86_64;

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
fn main() {
    let message = Message::ChangeColor(0, 160, 255);
    match message {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move: {x}{y}"),
        Message::Write(s) => println!("Write: {s}"),
        Message::ChangeColor(r, g, b) => println!("ChangeColor {r} {g} {b}")
    }
}
