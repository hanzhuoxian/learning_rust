use crate::front_of_house::hosting::seat_at_table;

pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_wait_list() {}
        pub fn seat_at_table() {
            println!("sit down!")
        }
    }
    pub mod serving {
        pub fn take_order() {}

        pub fn server_order() {}

        pub fn take_payment() {}
    }
}

fn deliver_order() {}

mod back_of_house {
    pub fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); // super 开头从父模块开始调用
    }

    fn cook_order() {}

    // 结构体成员需要单独表示是否公开
    pub struct Breakfast {
        pub toast: String,
        #[allow(dead_code)]
        pub seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // 枚举成员会继承枚举的公开属性。
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let mut meat = back_of_house::Breakfast::summer("Rye");
    meat.toast = String::from("Wheat");

    println!("I'd like {} toast", meat.toast);

    // 成员未公开，不能访问
    // println!("I'd like {} fruit", meat.seasonal_fruit);

    let _order1 = back_of_house::Appetizer::Soup;
    let _order2 = back_of_house::Appetizer::Salad;
    // 绝对路径
    crate::front_of_house::hosting::add_to_wait_list();
    // 相对路径
    front_of_house::hosting::add_to_wait_list();

    back_of_house::fix_incorrect_order();
    // 使用 use 创建别名
    seat_at_table();

    front_of_house::serving::take_order();
    front_of_house::serving::server_order();
    front_of_house::serving::take_payment();
}
