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