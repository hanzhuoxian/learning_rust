fn main() {
    enum Message {
        Hello { id: i32 },
    }

    let message = Message::Hello{id: 12};
    match message {
        Message::Hello { 
            id: id_var @3..7,
         } => {
            println!("id_var: {}", id_var);
        },
        Message::Hello { id: 10..=12 } => {
            println!("10~12");
        }
        Message::Hello { id } => {
            println!("id {}", id)
        }
    }

}
