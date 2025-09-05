# 不安全的 Rust

rust 不执行强制安全保证。通过使用 `unsafe` 来切换到不安全的 Rust。这里有五类能在不安全的 Rust
而不能在安全的 Rust 操作。

1. 解引用裸指针。
2. 调用不安全的函数或方法。
3. 访问或修改可变静态变量。
4. 实现不安全 trait。
5. 访问 union 字段。

unsafe 并不会关闭借用检查器或禁用其他任何安全检查。

## 解引用裸指针

不可变裸指针 `*const T` 可变裸指针 `*mut T` ，`*` 不是解引用运算符而是类型名称的一部分。可以在不安全的代码外创建裸指针，知识不能在不安全的代码解

裸指针特性：

1. 允许忽略借用规则，可以同时拥有不可变和可变的指针，或多个指向相同位置的可变指针。
2. 不保证指向有效的内存。
3. 允许为空。
4. 不能实现任何自动清理。

```rust
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

```

## 调用不安全函数或方法

```rust
fn main() {
    unsafe fn dangerous() {
        let num = 5;
        let r = &num as *const i32;
        unsafe {
            println!("dangerous is {}", *r);
        }
    }
    unsafe {
        dangerous();
    }
}
```

## 使用 extern 函数调用外部代码

extern 有助于创建和使用外部函数接口(Foreign Function Interface, FFI)。外部函数接口是一个编程语言用一定义函数的方式，其允许不同外部编程语言调用这些函数。

```rust
unsafe extern "C" {
   unsafe fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -5 is {}", abs(-5));
    }
}
```

## 访问或修改可变静态变量

静态变量只能存储拥有 `'static` 生命周期的引用，这意味着 Rust 编译器可以自己计算出其生命周期而无需显示标注。访问不可变静态变量是安全的。



```rust
static mut COUNTER: i32 = 0;
fn add_to_count(inc: i32) {
    unsafe {
        COUNTER += inc;
    }
}


fn main() {
    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

```

## 实现不安全 trait

当 `trait` 中至少有一个方法中包含编译器无法验证的不变式时 `trait` 是不安全的。可以在 `trait` 之前增加 `unsafe` 关键字将 `trait` 声明为 `unsafe`。
同时 `trait` 的实现也必须标记为 `unsafe`。

## 访问联合体中的字段

仅适用于 unsafe 的最后一个操作是访问 联合体中的字段，union 和 struct 类似，但是在一个实例中同时只能使用一个声明的字段。联合体主要用于和 C 代码中的联合体交互。