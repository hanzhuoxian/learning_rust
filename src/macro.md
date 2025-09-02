# 宏

宏是一种为写其他代码而写代码的方式，即所谓的元编程。宏在编译器翻译代码前展开。

## 声明宏
使用 marco_rules! 的声明宏用于通用元编程。

```rust
#[macro_export] // 表明只要导入了该crate就可以使用该宏
macro_rules! my_vec { // macro_rules! 定义宏 vec 宏名称
    ($($x:expr),*) => { // 与 match 类似，$x:expr 表示匹配一个表达式，$()表示重复，*表示重复0次或多次
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
```

## 过程宏

他们接受一个代码作为输入，在这些代码上进行操作，然后产生另一些代码作为输出。而非像声明宏那样匹配对应模式然后以另一部分代码替换当前代码。

有三种类型的过程宏：
自定义派生（derive）
类属性
类函数

```rust

```