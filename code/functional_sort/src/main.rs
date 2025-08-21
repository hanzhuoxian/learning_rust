#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];
    list.sort_by_key(|r| r.width);
    println!("sorted {list:?}");

    let mut sort_operations: Vec<String> = vec![];
    let value = String::from("closure called");
    list.sort_by_key(|r| {
        // sort_operations.push(value); // 会移动所有权，编译报错
        r.width
    });
    println!("sorted {list:?}");
}
