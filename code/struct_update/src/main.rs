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
        ..user1 // 必须放在最后
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
