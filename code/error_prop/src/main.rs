use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
    match read_username_from_file() {
        Ok(username) => {
            println!("{}", username)
        }
        Err(e) => {
            println!("{e:?}");
        }
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut user_name_file = match username_file_result {
        Ok(f) => f,
        Err(e) => {
            return Err(e);
        }
    };

    let mut username = String::new();
    match user_name_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => {
            return Err(e);
        }
    }
}
