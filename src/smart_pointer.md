# 智能指针

指针是一个包含内存地址的变量的通用概念。

智能指针是一类数据结构，他们的表现类似指针，但是也拥有额外的元数据和功能。

## `Box<T>`

最简单直接的智能指针是 `box`，其类型是 `Box<T>` 。box 允许你将一个值放在堆上而不是栈上，留在栈上的是指向堆数据的指针。

应用场景：

1. 当有一个在编译时大小未知的类型。而又想要在需要确切大小的上下文中使用这个类型值的时候。
2. 当有大量数据并希望确保数据不被拷贝的情况下转移所有权的时候。
3. 当希望拥有一个值并只关心他的类型是否实现了特定 trait 而不是具体类型的时候。

```rust
fn main() {
    let b = 5;
    println!("b = {b}");

    let c = Box::new(3);
    println!("box c = {c}");
}
```

我们可以像数据是存储在栈上的那样访问数据。正如任何拥有数据所有权的值那样。当 c 离开作用域的时候，它将被释放，这个释放作用域 box 本身（栈内存）和指向的数据（堆内存）。

### Box 允许创建递归类型

```rust
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};
fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("{:?}", list);
}

```

### `Deref trait`

实现 `Deref trait` 允许我们重载解引用运算符（dereference operator）*。通过这种方式实现 `Deref trait` 的智能指针可以被当作常规引用来对待。

```rust
fn main() {
    let x = 5;
    let y = &x; // y 等于 x 的一个引用

    assert_eq!(5, x);
    assert_eq!(5, *y); // 使用 *y 来访问引用指向的值。
}

```

Rust 使用 `DerefMut trait` 用于重载可变引用的 * 运算符。

Rust 在发现类型和 trait 实现满足三种情况时会进行 Deref 强制转换。

- 当 `T: Deref<Target=U>` 时从 `&T` 到 `&U`。
- 当 `T: Deref<Target=U>` 时从 `&mut T` 到 `&mut U`。
- 当 `T: Deref<Target=U>` 时从 `&mut T` 到 `&U`。

类型强制转化的一个例子

```rust
use std::{fmt::Display, ops::{Deref, DerefMut}};

#[derive(Debug)]
struct One<T: Display> {
    inner: OneOne<T>,
}

#[derive(Debug)]
struct OneOne<T: Display> {
    v: T,
}

impl<T: Display> Deref for One<T> {
    type Target = OneOne<T>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl <T:Display> DerefMut for One<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

fn main() {
    let one = One {
        inner: OneOne { v: 1 },
    };
    print_one_one(&one);

    let mut mut_one = One{
        inner: OneOne { v: 2 },
    };
    print_one_one(&mut mut_one);

    update_one_one(&mut mut_one, 3);

    print_one_one(&mut mut_one);

}

fn print_one_one<T: Display>(one_one: &OneOne<T>) {
    println!("{}", one_one.v);
}


fn update_one_one<T:Display>(one_one: &mut OneOne<T>, u: T) {
    one_one.v = u
}
```

### `rop trait` 运行清理代码

对于智能指针模式来说，第二个重要的 trait 是 Drop 允许我们在变量离开作用域时执行一些代码。

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Drop CustomSmartPointer data with: `{}`!", self.data);
    }
}
fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointer created");
}

```

## `Rc<T>`

```rust
use crate::List::{Cons, Nil};
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count: {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a)); // b 会克隆 a 包含的 Rc<List>, 引用计数由 1 变为 2
    println!("count: {}", Rc::strong_count(&a));

    {
        let c = Cons(4, Rc::clone(&a)); // c 会克隆 a 包含的 Rc<List>, 引用计数由 2 变为 3
        println!("c : {c:?}");
        println!("inner count: {}", Rc::strong_count(&a));
    } // c 超出作用域, 引用计数由 3 变成 2
    println!("count: {}", Rc::strong_count(&a));
    println!("b : {b:?}");
}
```

## `RefCell<T>` 内部可变性模式

允许在即使有不可变引用时也可以改变数据。这通常是借用规则不允许的。为了改变数据，该数据结构使用 unsafe 来模糊 Rust 可变性和借用规则。使用 unsafe 表明我们在手动检查这些规则而不是让编译器检查。

不同于 `Rc<T>` `RefCell<T>` 代表数据的唯一所有权。对于 Rc Box 不可变性作用域编译时，而对于`RefCell<T>` 不可变性作用于运行时。对于引用如果运行时违反这些规则，程序会 panic并退出。

borrow_mut 返回了 RefMut<T> 类型的智能指针， borrow 返回了 Ref<T> 类型的智能指针。这两个类型都实现了 Deref，可以当作常规引用对待。

`RefCell<T>` 记录当前有多少个活动的 `Ref<T>` 和 `RefMut<T>` 智能指针。每次调用 `borrow` ，`RefCell<T>` 将活动的不可变借用计数加一。当 `Ref<T>` 值离开作用域时，不可变借用计数减一。就像编译时借用规则一样，`RefCell<T>` 在任何时候只允许有多个不可变借用或一个可变借用。