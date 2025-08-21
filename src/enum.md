# 枚举

结构体用来讲不通类型的字段几数组组合在一起。枚举用来表示有限集合中的成员，必须是可以枚举出所有可能的值。

我们定义的每一个枚举成员的名字也变成了一个构建枚举的实例的函数。作为定义枚举的结果这些构造函数会被自动定义。

```rust
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home_v4 = IpAddrKind::V4(127, 0, 0, 1);
    let home_v6 = IpAddrKind::V6(String::from("::1"));
    println!("{:?} {:?}", home_v4, home_v6);

    let std_v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
    let std_v6 = IpAddr::V6(Ipv6Addr::new(0,0,0,0,0,0,0,1));
    println!("{:?} {:?}", std_v4, std_v6);
}

```