use std::vec;

fn main() {
    let v1 = vec![1, 2, 3];

    let iter_v1 = v1.iter(); // 创建迭代器
    for val in iter_v1 {
        // 消费迭代器
        println!("got {val}");
    }

    let iter_v2 = v1.iter();
    let sum: i32 = iter_v2.sum();
    println!("sum {sum}");

    let iter_v3 = v1.iter();
    let iter_v4: Vec<_> = iter_v3.map(|x| x + 1).collect();
    assert_eq!(iter_v4, vec![2, 3, 4]);
}
