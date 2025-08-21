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
