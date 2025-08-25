use std::thread;

fn main() {
    let vec = vec![1,2,3];

    let handle = thread::spawn(move ||{
        println!("{:?}", vec);
    });

    handle.join().unwrap();
}
