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
