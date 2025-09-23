use std::{
    fs::File,
    io::{ErrorKind, Read},
};

fn main() {
    let f = File::open("hi.log");
    let mut file = match f {
        Ok(f) => f,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create("hi.log") {
                Ok(f) => f,
                Err(e) => panic!("Problem create the file {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error}");
            }
        },
    };

    let mut content = String::new();
    let _ = file.read_to_string(&mut content);
}
