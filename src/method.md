# 方法

方法与函数类似，它使用 `fn` 关键字和名称声明，可以拥有参数和返回值，同时包含在某处调用该方法时会执行的代码。不过方法与函数不同的是，它们在结构体（枚举或 trait 对象）的上下文中被定义。并且它们第一个参数名总是 `self`。它代表调用该方法的结构体实例。

`&self` 是 `self: &Self` 的缩写 `Self` 是 `impl` 类型的别名。`self` 参数支持所有借用规则，可以转移所有权，可以不可变借用，也可以可变借用。

```rust
fn rect_var() {
    let width1 = 30;
    let height1 = 20;
    println!("The area of Rectangles is {}", area(width1, height1));
}
fn area(width: u32, height: u32) -> u32 {
    return width * height;
}

fn rect_tuple() {
    let rect = (30, 40);
    println!("The area of Rectangles is {}", area_tuple(rect));
}

fn area_tuple(rectangles: (u32, u32)) -> u32 {
    return rectangles.0 * rectangles.1;
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 { // &self 是 self: &Self 的缩写 Self 是 impl 类型的别名
        self.height * self.width
    }

    fn area_full(self: &Self) -> u32 {
        self.height * self.width
    }
}

fn rect_struct() -> Rectangle {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("{}", area_struct(&rect));
    println!("{:?}", rect);
    println!("{:#?}", rect);
    dbg!(rect)
}

fn area_struct(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

fn rect_method() {
    let rect = Rectangle {
        width: 20,
        height: 30,
    };

    println!("rect_method area: {}", rect.area());
    println!("rect_method area_full: {}", rect.area_full());
}

fn main() {
    rect_var();
    rect_tuple();
    rect_struct();
    rect_method();
}

```

## 关联函数

所有在 `impl` 块中定义的函数被称为关联函数，因为它们与 `impl` 后面命名的类型相关。我们可以定义不以 `self` 为第一个参数的关联函数。因为它们并不作用一个结构体的实例。

不是方法的关联函数经常被用作返回一个结构体新实例的构造函数。这些函数的名称通常为 `new`
。

使用结构体名和 :: 来调用这个关联函数。

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Self{
            width:size,
            height:size,
        }
    }
}
fn main() {
    println!("square is {:#?}", Rectangle::square(2));
}

```

每个结构体都可以拥有多个 `impl` 块。