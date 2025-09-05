fn main() {
    let mut num = 5;
    let r1 = &num as *const i32; // 将不可变引用转换为裸指针
    let r2 = &mut num as *mut i32; // 将可变应用转化为裸指针

    let address = 0x012345usize;
    let _r = address as *const i32; // 创建指向任意类型的裸指针

    // 只能在不安全块中解引用裸指针
    unsafe {
        println!("r1 {}", *r1);
        println!("r2 {}", *r2);
    }
}
