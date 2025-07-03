# 结构体

struct 是一个自定义数据类型，允许包装和命名多个相关的值。从而形成一个有意义的组合。

结构体简写语法：变量名如果与结构体字段名相同，可以省略字段名。

```rust
// 结构体定义
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // 结构体实例创建
    let mut user1 = User {
        active: true,
        username: String::from("Li Li"),
        email: String::from("lili@zhouyi.xin"),
        sign_in_count: 1,
    };

    // 结构体字段修改
    user1.email = String::from("li.li@zhouyi.xin");

    // 结构体访问
    println!("user1.active: {}", user1.active);
    println!("user1.username : {}", user1.username);
    println!("user1.email : {}", user1.email);
    println!("user1.sign_in_count : {}", user1.sign_in_count);

    let user2 = build_user(user1.email.clone(), user1.username.clone());
    println!("user2.active: {}", user2.active);
    println!("user2.username : {}", user2.username);
    println!("user2.email : {}", user2.email);
    println!("user2.sign_in_count : {}", user2.sign_in_count);
}

fn build_user(email: String, username: String) -> User {
    // 也可以作为表达式返回结构体
    User {
        active: true,
        username, // 由于参数名与字段名相同可以省略字段名。
        email,
        sign_in_count: 1,
    }
}

```

## 结构体更新语法

更新语法会转移结构体中的变量所有权

```rust
// 结构体定义
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let email = String::from("li.li@zhouyi@xin");
    let username = String::from("li.li");
    let user1 = build_user(email, username);
    let user2 = User {
        username: String::from("lili"),
        ..user1
    };
    println!("user2.active: {}", user2.active);
    println!("user2.username : {}", user2.username);
    println!("user2.email : {}", user2.email);
    println!("user2.sign_in_count : {}", user2.sign_in_count);
}

fn build_user(email: String, username: String) -> User {
    // 也可以作为表达式返回结构体
    User {
        active: true,
        username, // 由于参数名与字段名相同可以省略字段名。
        email,
        sign_in_count: 1,
    }
}
```

## 没有命名字段的元组结构体

元组结构体有着结构体名称提供的含义，但没有具体的字段名称。当你想给整个元组取一个名字，并使元组成为与其他元组不同的类型时。