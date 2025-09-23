use std::vec;

fn main() {
    let mut vector = vec![1, 2, 3, 4, 5, 6];
    let (a, b) = split_at_mut(&mut vector, 3);
    println!("a: {:?}, b: {:?}", a, b);
}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    assert!(mid <= len);
    let ptr = values.as_mut_ptr();
    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
