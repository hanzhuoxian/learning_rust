fn main() {
    {
        // 变量尚未声明，`s` 无效
        let s = "hello"; // 从此处起，变量`s`是有效的
        println!("{}, world!", s);
    } // 变量离开作用域，变量 `s`` 无效
}
