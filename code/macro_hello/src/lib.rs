use macro_hello_derive::HelloMacro;

pub trait HelloMacro {
    fn hello_macro();
}


#[derive(HelloMacro)]
pub struct Pancakes;