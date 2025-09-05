struct Counter {
    count: u32,
}
impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }

    fn add(&mut self, inc: u32) {
        self.count += inc;
    }

    fn get(&self) -> u32 {
        self.count
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
fn main() {
    let mut counter = Counter::new();
    counter.add(1);
    println!("Counter: {}", counter.get());

    for i in counter {
        println!("Iterator: {}", i);
    }
}
