struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a, 'b> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    fn part(&'a self, x: &'b str) -> &'a str {
        println!("{}", x);
        self.part.split('.').next().unwrap()
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");

    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    println!("{}", i.part);
    println!("{}", i.part("left"));

    let s: &'static str = "hello";
    println!("{}", s);
}
