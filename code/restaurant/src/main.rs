use restaurant::{eat_at_restaurant, front_of_house::hosting::seat_at_table};

use crate::home::watch_tv;

mod home {
    pub fn watch_tv() {
        println!("watch tv")
    }
}

fn main() {
    seat_at_table();
    watch_tv();
    eat_at_restaurant();
}
