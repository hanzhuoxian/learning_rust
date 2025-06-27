# 结构体

struct 是一个自定义数据类型，允许包装和命名多个相关的值。从而形成一个有意义的组合。

```rust
// 定义结构体
struct User {
    active: bool,
    user_name: String,
    email: String,
    sign_in_count: u64,
}
```
