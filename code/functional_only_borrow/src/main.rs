use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure {list:?}");

    let only_borrows = || {
        println!("Before defining closure {list:?}");
    };

    println!("Before call closure {list:?}");
    only_borrows();
    println!("After call closure {list:?}");

    println!("---------------------");

    let mut list = vec![1,2,3];
    println!("After call closure {list:?}");
    let mut mut_borrows = || list.push(7);
    mut_borrows();
    println!("After call closure {list:?}");

    let mut list = vec![1,2,3];

    println!("Before defining closure {list:?} thread");
    thread::spawn(move ||{
        list.push(7);
        println!("thread {list:?}");
    }).join().unwrap();

}
