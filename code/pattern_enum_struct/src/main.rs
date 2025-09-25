#[allow(dead_code)]
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}
#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn main() {
    let message = Message::ChangeColor(Color::Rgb(0, 160, 255));
    match message {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move: {x} {y}"),
        Message::Write(s) => println!("Write: {s}"),
        Message::ChangeColor(Color::Rgb(r, g, b)) => println!("ChangeColor Rgb {r} {g} {b}"),
        Message::ChangeColor(Color::Hsv(r, g, b)) => println!("ChangeColor Hsv {r} {g} {b}"),
    }
}
