# 控制流

根据条件是否为真来决定是否执行某些代码，以及根据条件是否为真来重复运行一段代码的能力。

## `if` 表达式

允许根据条件执行不同的代码分支。`if` 只会执行第一个分支为 `true` 的代码

```rust
fn main() {
    let number = 6;
    // `if` 只会执行第一个分支为 `true` 的代码
    if number % 4 == 0 {
        println!("number {} is divisible by 4", number);
    } else if number % 3 == 0 {
        println!("number {} is divisible by 3", number);
    } else if number % 2 == 0 {
        println!("number {} is divisible by 2", number);
    } else {
        println!("number {} is not divisible by 4,3, or 2", number);
    }
}
```

因为 `if` 是一个表达式，我们可以在 let 语句的右侧使用它。所有分支中的代码块都必须返回相同的类型。

```rust
fn main() {
    let condition = false;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");
}
```

## 循环

多次执行同一段代码一个循环执行循环体中的代码直到结尾并紧接着返回开头继续执行。

### `loop` 循环表达式

```rust
fn main() {
    let mut number = 0;
    loop {
        number = number + 1;
        if number == 2 {
            continue; // 跳过本次循环
        }
        if number > 4 {
            break; // 退出所有循环
        }
        println!("number is {number}!");
    }
    println!("number loop is end!");

    println!("loop return value start!");
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

}

```

循环标签

```rust
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up; // 退出 'counting_up 标签
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);
}
```

### `while` 条件循环

当 `while` 的表达式不为 `true` 时结束循环

```rust
fn main() {
    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
    println!("LIFTOFF!");
}

```

### `for` 循环

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("The value is {}", element)
    }

    // 使用 Range 生成迭代器
    for i in 1..4 {
        println!("i {}", i)
    }
}

```

### `match` 控制流

`match` 允许我们将一个值与一系列的模式相比较，并根据相匹配的模式执行相应的代码。模式可由字面量、变量、通配符和许多其他内容构成。它的力量来源于模式的表现力及编译器的检查，它确保了所有可能的情况都得到了处理。

`match` 执行时会将表达式的值按顺序与每一个分支的模式进行比较，如果模式匹配了这个值，那么相关联的代码将会被执行。如果不匹配就继续检查下一个分支。每一个分支关联的是一个表达式，而表达是的结果值将作为 `match` 表达式的返回值。如果分支代码多的话需要使用大括号，并且逗号可以省略不写。

```rust

match 表达式 {
    模式1 => 代码,
    模式2 => 代码,
}

```

枚举举例

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_coin(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
fn main() {
    let penny = Coin::Penny;
    let nickel = Coin::Nickel;
    let dime = Coin::Dime;
    let quarter = Coin::Quarter;
    println!("penny: {}", value_in_coin(penny));
    println!("nickel: {}", value_in_coin(nickel));
    println!("dime: {}", value_in_coin(dime));
    println!("quarter: {}", value_in_coin(quarter));
    
}
```

#### 绑定值模式

匹配分支的另一个有用的功能是绑定匹配的模式的部分值。这也就是如何从枚举成员中提取值的。

```rust

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn value_in_coin(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("{:?}", state);
            25
        }
    }
}

fn main() {
    let penny = Coin::Penny;
    let nickel = Coin::Nickel;
    let dime = Coin::Dime;
    let alabama_quarter = Coin::Quarter(UsState::Alabama);
    let alaska_quarter = Coin::Quarter(UsState::Alaska);
    println!("penny: {}", value_in_coin(penny));
    println!("nickel: {}", value_in_coin(nickel));
    println!("dime: {}", value_in_coin(dime));
    println!("alabama_quarter: {}", value_in_coin(alabama_quarter));
    println!("alaska_quarter: {}", value_in_coin(alaska_quarter));
}
```

#### 匹配 `Option<T>`

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1)
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("{}", six.unwrap());
    if none == None {
        println!("None");
    }

}
```

#### 通配符

```rust
fn main() {
    let dice_roll = 9;
    let dice = match dice_roll {
        1..=8 => 8, // 匹配一个范围
        9 => 9, // 匹配单独的值
        other => other, // 匹配所有未匹配的值
        // _ => 0, // 匹配所有未匹配的值但是不需要使用该值
    };
    println!("{}", dice);
}
```

### `if let`

```rust
let x = Some(3u8)
if let Some(s) = x {
    println!("{}", x);
}
```
