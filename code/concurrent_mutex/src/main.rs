use std::sync::Mutex;

fn main() {
    let m = Mutex::new(1);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("{:?}", m);
}
