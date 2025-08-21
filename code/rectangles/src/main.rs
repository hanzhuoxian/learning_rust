fn rect_var() {
    let width1 = 30;
    let height1 = 20;
    println!("The area of Rectangles is {}", area(width1, height1));
}
fn area(width: u32, height: u32) -> u32 {
    return width * height;
}

fn rect_tuple() {
    let rect = (30, 40);
    println!("The area of Rectangles is {}", area_tuple(rect));
}

fn area_tuple(rectangles: (u32, u32)) -> u32 {
    return rectangles.0 * rectangles.1;
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 { // &self 是 self: &Self 的缩写 Self 是 impl 类型的别名
        self.height * self.width
    }

    fn area_full(self: &Self) -> u32 {
        self.height * self.width
    }
    fn area_type(self: &Rectangle) -> u32 {
        self.height * self.width
    }

    fn can_hold(&self, rect: &Self) -> bool {
        self.width >= rect.width && self.height >= rect.height
    }
}

fn rect_struct() -> Rectangle {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("{}", area_struct(&rect));
    println!("{:?}", rect);
    println!("{:#?}", rect);
    dbg!(rect)
}

fn area_struct(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

fn rect_method() {
    let rect = Rectangle {
        width: 20,
        height: 30,
    };

    println!("rect_method area: {}", rect.area());
    println!("rect_method area_full: {}", rect.area_full());
    println!("rect_method area_type: {}", rect.area_type());
}

fn can_hold() {
let rect1 = Rectangle{
    width: 30,
    height: 50,
};
let rect2 = Rectangle{
    width: 10,
    height: 40,
};
let rect3 = Rectangle {
    width: 60,
    height: 45,
};

println!("Can rect1 hold rect2 {}", rect1.can_hold(&rect2));
println!("Can rect1 hold rect3 {}", rect1.can_hold(&rect3));
}

fn main() {
    rect_var();
    rect_tuple();
    rect_struct();
    rect_method();
    can_hold();
}
