use std::arch::x86_64;

enum Color {
    RGB(i32, i32, i32),
    Hsv(i32, i32, i32),
}
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}
fn main() {
    let message = Message::ChangeColor(Color::RGB(0, 160, 255));
    match message {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move: {x}{y}"),
        Message::Write(s) => println!("Write: {s}"),
        Message::ChangeColor(Color::RGB(r, g, b)) => println!("ChangeColor RGB {r} {g} {b}"),
        Message::ChangeColor(Color::Hsv(r, g, b)) => println!("ChangeColor Hsv {r} {g} {b}"),
    }
}
