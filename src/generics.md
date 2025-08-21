# 泛型

我们可以使用泛型为像函数签名或结构体这样的项创建定义，这样它们就可以用于多种不同的
具体数据类型。

## 在函数中定义泛型

当使用泛型函数时，本来在函数签名中指定参数和返回值类型的地方，会使用泛型来表示。

```rust
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for i in list {
        if i > largest {
            largest = i;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for i in list {
        if i > largest {
            largest = i;
        }
    }
    largest
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for i in list {
        if i > largest {
            largest = i;
        }
    }
    largest
}

fn main() {
    let number_list = vec![10, 20, 30, 40, 50];
    println!("The largest number is {}", largest_i32(&number_list));
    println!("The largest number is {}", largest(&number_list));

    let char_list = vec!['y', 'm', 'a'];
    println!("The largest char is {}", largest_char(&char_list));
    println!("The largest char is {}", largest(&char_list));
}

```

## 在结构体定义泛型

同样也可以用 <> 语法来定义结构体，它包含一个或多个泛型参数类型字段。必须在结构体名称后面的尖括号中声明泛型参数的名称。接着在结构体定义中可以指定具体数据类型的位置使用泛型类型。

```rust
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}
fn main() {
    let p = Point{x:1, y:2};
    println!("{:?} x={}, y={}", p, p.x, p.y);
    let p = Point{x:1.1, y:2.2};
    println!("{:?} x={}, y={}", p, p.x, p.y);
}

```

## 在枚举中定义泛型

```rust
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

## 方法中定义泛型

在为结构体和枚举实现方法时也一样可以使用泛型。必须在 `impl` 后面声明 `T`。`rust` 就知道 `Point<T>` 尖括号中的 `T` 是泛型而不是具体的类型。
我们可以为泛型参数选择一个与结构体泛型参数不同的名字。不过依照惯例使用了相同的名称。

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}
fn main() {
    let p = Point{x: 1, y: 2};

    println!("{}", p.x());
    println!("{}", p.y());
}
```

与结构体参数名不同

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<U> Point<U> {
    fn x(&self) -> &U {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }
}

fn main() {
    let p = Point{x: 1, y: 2};

    println!("{}", p.x());
    println!("{}", p.y());
}
```

定义方法时也可以为泛型指定限制

```rust
impl<f32> Point<f32> {
    fn x(&self) -> &f32 {
        &self.x
    }

    fn y(&self) -> &f32 {
        &self.y
    }
}
```

方法的泛型与结构体或枚举不通

```rust
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, p: Point<X2, Y2>) -> Point<X1, Y2> {
        Point { x: self.x, y: p.y }
    }
}
fn main() {
    let p = Point { x: 1, y: 1.1 };
    let p2 = Point { x: "hello", y: 'y' };
    let p3 = p.mixup(p2);
    println!("{} {}", p3.x, p3.y)
}
```

## 泛型代码的性能

泛型代码不会增加运行时开销，通过在编译时将泛型单态化来保证效率。通过填充编译时使用的具体类型，将通用代码转化为特定代码的过程。