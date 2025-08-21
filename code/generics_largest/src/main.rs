fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for i in list {
        if i > largest {
            largest = i;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for i in list {
        if i > largest {
            largest = i;
        }
    }
    largest
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for i in list {
        if i > largest {
            largest = i;
        }
    }
    largest
}

fn main() {
    let number_list = vec![10, 20, 30, 40, 50];
    println!("The largest number is {}", largest_i32(&number_list));
    println!("The largest number is {}", largest(&number_list));

    let char_list = vec!['y', 'm', 'a'];
    println!("The largest char is {}", largest_char(&char_list));
    println!("The largest char is {}", largest(&char_list));
}
