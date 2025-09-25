fn main() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of {s2} is {len}");

    let len = calculate_length_ref(&s2);
    println!("The length of {s2} is {len}");

    let mut s1 = String::from("hello");
    change(&mut s1);
    println!("s1 is {s1}");

    let r1 = &mut s1;
    // let r2 = &mut s1; // cannot borrow `s1` as mutable more than once at a time second mutable borrow occurs here
    println!("{r1}")
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

#[allow(clippy::ptr_arg)]
fn calculate_length_ref(s: &String) -> usize {
    // s 是 String 的引用
    s.len()
} // 这里 s 离开作用域，但是它是引用并没有所有权，所以什么也不会发生。

fn change(s: &mut String) {
    s.push_str(", world");
}
