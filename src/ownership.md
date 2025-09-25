# 所有权

所有权是 `Rust` 最为与众不同的特性，对语言的其他部分有着深刻的意义。让 `Rust` 无需垃圾回收也能保障内存安全。

所有权（ownership） 是 Rust 用于如何管理内存的一组规则。所有程序都必须管理运行时使用计算机内存的方式。一些语言中具有垃圾回收机制，在程序运行时有规律的寻找不再使用的内存。在另一些语言中，程序员必须手动分配和释放内存。`Rust` 通过所有权系统管理内存。

栈与堆都是代码在运行时可供使用的内存，但是他们的结构不同。栈以放入值的顺序存储并与相反的顺序取出值。这也被称作后进先出（last in， first out）。增加数据叫进栈，取出数据叫出栈。栈中的数据都必须占用已知且固定的大小。在编译时未知或者大小可能发生变化的，需要改为存储在堆上。堆时缺乏组织的，当向堆放入数据时，内存分配器在某处找到一个足够大的空位。把它标记为已使用并返回一个该位置地址的指针。这个过程称为在堆上分配内存。

入栈为什么比在堆上分配内存快：
1. 入栈无需搜索可用内存空间。其位置总是在栈顶。堆需要搜索内存空间，并记录。
2. 访问堆上的数据必须通过指针来访问，现代处理器在内存中跳转的越少就越快。

跟踪哪部分代码正在使用堆上的哪些数据，最大限度的减少堆上的重复数据的数量，以及清理堆上不再使用的数据确保不会耗尽空间，这些问题正是所有权系统要处理的。

## 所有权规则

1. Rust 中每一个值都有一个所有者（owner）。
2. 值在任何时刻有且只有一个所有者。
3. 当所有者离开作用域，这个值将被丢弃

### 变量作用域

当 `s` 进入到这个作用域时，它就是有效的。一直持续到它离开作用域为止。

```rust
fn main() {
    { // 变量尚未声明，`s` 无效
        let s = "hello"; // 从此处起，变量`s`是有效的
        println!("{}, world!", s);
    } // 变量离开作用域，变量 `s`` 无效
}
```

String 与 字符串字面值。对于字符串字面值，在编译时就知道确切的大小。会直接编码进最终的可执行文件中。这使得字符串字面值快速且高效。对于 String 来说为了支持可变可增长的文本，需要在堆上分配一个编译时未知大小的内容来存放内容。

- 必须在运行时向分配器请求内存。
- 当我们使用完 String 时将内存返还给分配起的方法。

`String::from` 在运行时申请内存。当变量离开作用域的时候 Rust 为我们调用一个特殊的函数 `drop`。在 `drop` 里作者可以放置释放内存的代码。

```rust
fn main() {
    let mut s = String::from("Hello");// 请求内存
    s.push_str(", world!");

    println!("{}", s);
} // 返还给分配器
```

### 变量与数据交互的方式：移动

```rust
let x = 5;
let y = x; // 生成一个 `x` 值的拷贝并绑定到 `y`
```

String 由三部分组成，ptr（字符串数据指针）、len（字符数组长度）、capacity（字符数组容量）。字符串的数据存储在堆中。我们将 `s1` 的值赋值给 `s2`。`String` 的数据被复制了，我们拷贝了 `ptr`、`len`、`capacity`，但是没有拷贝堆中的数据。`s1` 的变量被无效了，这个操作被称为移动。

```rust
let s1 = String::from("hello");
let s2 = s1;
```

### 变量与数据交互的方式：克隆

如果想要拷贝堆中的数据需要使用 `s1.clone()`，只在栈上的数据会拷贝而不需要克隆。

```rust
fn main() {
    let s1 = String::from("Hello");
    let s2 = s1;
    // println!("{}", s1); s1 变量无效
    let s3 = s2.clone();
    println!("{} {}", s2, s3);
}
```

### 所有权与函数

将值传递给函数与给变量赋值的原理相似。向函数传递值可能会移动或复制。就像赋值语句一样。

```rust
fn main() {
    let s = String::from("Hello World!");
    takes_ownership(s); // 此行过后 s 变量已经被无效
    // println!("{}", s); // 编译错误： borrow of moved value: `s` value borrowed here after move
    let x = 5;
    makes_copy(x);
    println!("{}", x);
}
fn takes_ownership(ss: String) {
    println!("{}", ss);
} // 调用 ss.drop 释放内存

fn makes_copy(y: i32) {
    println!("{}", y);
} // 这里 y 移出作用域，没有特殊之处。

```

### 返回值与作用域

```rust
fn main() {
    let s1 = gives_ownership();
    println!("{s1}");
    let s2 = String::from("hello"); // s2 进入作用域
    let s3 = takes_and_gives_back(s2); // s2 被移动到函数中它也将值返回值移动给 s3。
    // println!("{s2}");
    println!("{s3}");
} // s1、s3 移出作用域，s2 也移出作用域，但是已经被移走什么都不会发生。

fn gives_ownership() -> String { // 将返回值移动值调用他的函数
    let some_string = String::from("yours"); // some_string 进入作用域
    some_string // 返回 some_string 并移出给调用它的函数
}

fn takes_and_gives_back(s: String) -> String {
    s
}
```

### 引用与借用

引用（reference）像一个指针。它是一个地址，我们可以由此访问存储与其他变量的数据。与指针不同，引用确保指向指向某个特定类型的有效值。我们将创建引用的行为称为借用。引用只能读取其中的值，如果想要修改其中的值可以使用可变引用 `&mut`。有一个可变引用就不能再创建其他引用为了避免数据竞争。

悬垂引用（Dangling References），在具有指针的语言中，很容易在释放内存时保留一个指向它的指针而错误的生成一个悬垂指针。

Rust 中编译器确保引用永远不会成为悬垂引用。在未离开作用域之前内存永远是有效的。

```rust
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

fn calculate_length_ref(s: &String) -> usize {
    // s 是 String 的引用
    s.len()
} // 这里 s 离开作用域，但是它是引用并没有所有权，所以什么也不会发生。

fn change(s: &mut String) {
    s.push_str(", world");
}
```

### Slice 类型

slice 允许引用集合中一段连续的元素，而不用引用整个集合。slice 是一种引用，所以它没有所有权。使用由`s[start_index..end_index]` 中,中括号指定的 `range` 创建一个 `slice`。`start_index` 是 `slice` 的第一个位置。`end_index` 则是最后一个位置的后一个值。在 `slice` 内部存储了开始位置和长度。省略 `start_index` 则表示 0，省略 `end_index` 表示最后一个元素。

```rust
fn main() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{hello} {world}")
}
```

字符串字面值：&str 它是一个指向二进制程序位置的 slice。这也就是为什么字符串字面值是不可变的。&str 是一个不可变引用。
