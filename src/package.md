# 包管理

## 包

包：Cargo 的一个功能允许构建测试分享 crate。

包是提供一系列功能的一个或多个 crate。一个包会包含一个 cargo.toml 文件。阐述如何去构建这些 crate。包中可以包含至多一个库文件。任意多个二进制文件。但是必须至少包含一个 crate。

## Crates

Crates：一个模块的树形结构，它形成了库或二进制项目。

crate 是 rust 编译时最小的代码单位。crate 可以包含模块，模块可以定义在其他文件中。crate 有两种形式：二进制项和库。二进制项可以被编译为可执行文件，必须包含 main 函数。库不包含 main 函数，也不会被编译为可执行文件。

crate root 是一个源文件，rust 编译器以它为起点。并构成你 crate 的跟模块。

rust 遵循一个约定就是 src/main.rs 就是一个与包同名的二进制 crate 的 crate根。src/lib.rs 表明包带有与库同名的 crate 库。且 src/lib.rs 是 crate 根。

## 模块和 use

模块和 use：允许你控制作用域和路径的私有性。

从 crate 的根节点开始，当编译一个 crate ，编译器首先在 crate 根文件寻找需要被编译的代码。

声明模块：在 crate 根文件中你可以声明一个新的模块。比如你用 `mod garden`,声明了一个叫做 garden 的模块，编译器会在下列路径寻找代码。

- 内联：在大括号中，mod garden 后方不是一个分号而是大括号。
- src/garden.rs
- src/garden/mod.rs

声明子模块：比如你在 src/garden.rs 中用 `mod vegetables` 声明了一个叫做 vegetables 的模块。

- 内联：在大括号中，mod vegetables 后方不是一个分号而是大括号。
- src/garden/vegetables.rs
- src/garden/vegetables/mod.rs

使用 use 关键词时一般只引用到父模块，然后通过父模块调用，用来区别本地函数。

可以使用 as 关键词提供新名称


```rust
use std::io::Result as IoResult;
use std::fmt::Result;
```

使用 pub use 重新导出名称

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

使用 glob 运算符将所有公有定义引入作用域。经常用于测试模块

```rust
use std::collections::*;
```

### 模块树

```rust
mod front_of_house {
    mod hosting {
        fn add_to_wait_list() {}
        fn seat_at_table() {}
    }
    mod serving {
        fn take_order() {}

        fn server_order() {}

        fn take_payment() {}
    }
}
```

src/main.rs 和 src/lib.rs 被称为模块树， 之所以这样叫它们是因为这
两个文件的内容都分别在 crate 模块结构的根组成了一个名为 crate 的模块，该结构被称为
模块树（module tree）

```text
crate
   front_of_house 
        hosting
            add_to_wait_list
            seat_at_table
        serving
            take_order
            server_order
            take_payment

```

## 模块中的代码路径

路径：一个命名例如结构体、函数或模块的方式。一旦一个模块是你 crate 的一部分，你可以在隐私规则的允许下，在同一 crate 的任意地方通过代码路径引用改模块的代码。vegetables 下的 Asparagus 类型可以被 crate::garden::vegetables::Asparagus 找到。

一个模块里的代码默认对父模块私有，为了使模块公有需要使用 pub mod 代替 mod。

use 关键字，在一个作用域内，use 关键字创建了一个成员的快捷方式，用来减少长路径的重复。

## 引用模块项目的路径

### 绝对路径

使用 crate 开头的绝对路径。

### 相对路径

使用 self、super或定义在当前模块中的标识符。
