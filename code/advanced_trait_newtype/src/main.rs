use std::ops::Deref;

struct Wrapper(Vec<String>);

impl std::fmt::Display for Wrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

impl Deref for Wrapper {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


fn main() {
    let wrapped = Wrapper(vec![String::from("Hello"), String::from("world")]);
    println!("{}", wrapped);

    for s in wrapped.iter() {
        println!("{}", s);
    }
}
