use std::collections::HashMap;

fn main() {
    let mut m = HashMap::new();
    m.insert("Blue", 10);
    m.insert("Yellow", 50);

    println!("{}", m.get("Blue").copied().unwrap_or(0));

    for (key, value) in m {
        println!("{} {}", key, value);
    }

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut m = HashMap::new();
    // 所有权会被移动值 map
    m.insert(field_name, field_value);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Red");
    m.insert(field_name.clone(), field_value);

    for (key, value) in m.into_iter() {
        println!("{} {}", key, value)
    }

    let mut m = HashMap::new();
    m.entry(field_name).or_insert("Yellow".to_string());

    println!("{m:?}");
}
