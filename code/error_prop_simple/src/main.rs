use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
    match read_username_from_file() {
        Ok(username) => {
            println!("{}", username)
        },
        Err(e) => {
            println!("{e:?}");
        }
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    let mut username_file = File::open("hello.txt")?;
    username_file.read_to_string(&mut username)?;

    // 链式调用进一步简化代码
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
