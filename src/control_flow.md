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
