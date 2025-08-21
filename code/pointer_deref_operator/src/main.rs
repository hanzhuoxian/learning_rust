fn main() {
    let mut x = 5;
    let y = &x; // y 等于 x 的一个引用

    assert_eq!(5, x);
    assert_eq!(5, *y); // 使用 *y 来访问引用指向的值。

    let z = Box::new(x); // 使用 Box 智能指针 拷贝值并新建对象
    assert_eq!(5, *z);

    x = 2;
    assert_eq!(2, x);
    assert_eq!(5, *z); 
}
