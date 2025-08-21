# Trait

定义共同行为，trait 定义了某个特定类型拥有可能与其他类型共享的功能。可以通过 trait 以一种抽象的方式定义共同行为。可以使用 trait bounds 指定泛型是任何拥有特定行为的类型。

## 定义 trait

一个类型的行为由其可供调用的方法构成。如果可以对不同类型调用相同的方法的话，这些类
型就可以共享相同的行为了。trait 定义是一种将方法签名组合起来的方法，目的是定义一个实
现某些目的所必需的行为的集合。

使用 `trait` 来声明一个 trait，后面是 trait 的名字。大括号中是方法的签名。在方法签名后跟分号而不是大括号及其实现。接着每一个实现这个 trait 的类型都需要提供其自定义行为的方法体，编译器也会确保任何实现 Summary trait 的类型都拥有与这个
签名的定义完全一致的 summarize 方法。

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

## 实现 trait

当 trait 或者 类型至少有一个属于当前 crate 才能对类型实现该 crate。不能为外部类型实现外部 trait，这个被称为相干性。孤儿规则。

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} by {},{}", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

## 调用 trait

trait 必须和类型一起引入作用域以便使用额外的 trait 方法。

```rust
use crate::aggregator::{NewsArticle, Summary, Tweet};

mod aggregator;

fn main() {
    let news_article = NewsArticle{
        headline: "Look".to_string(),
        location: "Shan Xi".to_string(),
        author: "Han ZhuoXian".to_string(),
        content: "Look at the man".to_string(),
    };

    println!("news article: {}", news_article.summarize());

    let tweet = Tweet {
        username: "zhuoxian".to_string(),
        content: "I like you!".to_string(),
        reply:false,
        retweet:false,
    };

    println!("tweet: {}", tweet.summarize());
}

```

## 默认实现

为方法提供默认行为，为某个类型实现时可以选择覆盖默认行为。

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        "Read from ...".to_string()
    }
}
```

## Trait 作为参数

```rust
// Trait Bound 语法 适合更复杂的场景
pub fn notify_bound<T: Summary> (item : &T) {
    println!("News : {}", item.summarize());
}

// &impl 是 Trait Bound 语法的语法糖，适合简短的例子
pub fn notify(item: &impl Summary) {
    println!("News : {}", item.summarize());
}

// 使用 + 指定多个 trait bound
pub fn notify_bound_display<T: Summary + Display>(item: &T) {
    println!("News : {}", item.summarize());
}

// 多个 trait bound 的语法糖
pub fn notify_display(item: &(impl Summary + Display)) {
    println!("News : {}", item.summarize());
}

// 使用 where 子句简化函数签名
pub fn some<T, U>(t: &T, u: &U)
where
    T: Display + Clone,
    U: Clone + Debug,
{
    println!("some {} {:?}", t, u);
}

```

## 使用 `trait bound` 有条件的实现方法

通过使用带有 trait bound 的泛型参数的 impl 块，可以有条件地只为那些实现了特定 trait 的类型实现方法。

```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

struct Own {

}
// 为所有 Pair 结构体实现 new 方法
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x: x, y: y }
    }
}

// 限定只有实现 T 实现了 Display 和 PartialOrd 才能使用该方法。 
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("x >= y");
        } else {
            println!("x < y");
        }
    }
}

fn main() {
    let pair = Pair::new(1, 2);
    pair.cmp_display();

    let o_x = Own{};
    let o_y = Own{};
    let pair = Pair::new(o_x, o_y);
    // pair.cmp_display();// 不能调用该方法
    
}
```

对任何实现了特定 trait 的类型实现方法 blanket implementations。

```rust
impl <T: Display> ToString for T {

}
```
