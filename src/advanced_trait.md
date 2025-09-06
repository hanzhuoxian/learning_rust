# 高级 trait

## 关联类型在 trait 定义中指定占位符类型

关联类型（associated types）是一个将类型占位符与 trait 相关联的方式，这样 trait 的方法签名中就可以使用这些占位符类型。
trait 的实现者会针对特定的实现在这个占位符类型置顶相应的具体类型。如此可以定义一个可以使用多种类型的 trait，知道实现此 trait 时
都无需知道这些类型具体时什么。

```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

Item 是一个占位符类型，同时 next 方法定义表明它返回 Option<Self::Item> 类型的值。这个 trait 的实现者会指定 Item 的具体类型。
然而不管实现者置顶何种类型，next 方法都会返回一个包含了此具体类型值的 Option。

关联类型看起来像一个类似泛型的概念，因为它允许定义一个函数而不指定其可以处理的类型。让我们通过在一个 Counter 结构体上实现 Iterator 
trait 的例子来检视其中的区别。这哥是现在宏指定了 Item 的类型为 u32:

## 默认泛型类型参数和运算符重载

`<Rhs = Self>` 这个语法叫做默认类型参数（default type parameters）。Rhs 是一个泛型类型参数（“right hand side”）的缩写。
它用于定义 add 方法中的 rhs 参数。如果实现 Add trait 时不指定 Rhs 的具体类型， Rhs 的类型将时默认的 Self 类型。也就是在骑上实现的 Add 类型。

扩展类型而不破坏现有代码。
在大部分用户都不需要的特定情况下进行自定义。

```rust
pub trait Add<Rhs = Self> {
    type Output;
    fn add(self, rhs: Rhs) -> Self::Output;
}
```

## 完全限定语法与消歧义：调用相同名称的方法

### 完全限定语法

```rust
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    println!("A baby dog is called a {}", Dog::baby_name());
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
}

```

## newtype 模式 用以在外部类型上实现外部trait

## 类型别名用来创建类型同义词

类型别名不是一新的单独的类型，完全被当作相同的类型对待。

```rust
fn main() {
    type Kilometers = i32;
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);
}

```

## 动态大小类型和 Size trait

有时被称为 `DST` 或 `unsized types` ，这些类型允许我们处理只有在运行时才知道大小的类型。
为了处理 DST ，Rust 提供了 Sized trait 来决定一个类型的是否在编译时可知。这个 trait 自动为编译器在编译时就知道大小的类型实现。 Rust 
