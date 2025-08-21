# 错误处理

`Rust` 将错误分为两大类：可恢复（recoverable）和不可恢复（unrecoverable）错误。对于一个可恢复的错误，比如文件未找到我们可能只想向用户报告并重试操作。不可恢复的错误总是bug 出现的征兆。比如试图访问一个超过数组末端的位置。

## 不可恢复错误：panic

用 `panic!` 处理不可恢复的错误，panic 时程序默认会展开，`Rust` 会回溯并清理遇到的每一个函数的数据。panic 通过在 Cargo.toml 中增加 `panic = 'abort'` 程序使用的内存由操作系统来清理。

```toml
[profile.release]
panic = 'abort'
```

有两种方法会造成 `panic`

- 执行会造成 `panic` 的操作。比如执行数组越界访问。
- 调用 `panic!` 宏

```rust
fn main() {
    // 数组越界访问 panic
    let a = [1,2,3];
    println!("{}", a[4]);

    // 手动调用 panic!
    panic!("crash and burn");
}

```

## 可恢复错误：Result

大部分错误失败并没有严重到需要程序完全停止。有时候一个函数失败，仅仅就是因为一个容易理解和响应的原因。

```rust
enum Result <T, E> {
    Ok(T),
    Err(E),
}
```

`Result` 示例

```rust
use std::{fs::File, io::{ErrorKind, Read}};

fn main() {
    let f = File::open("hi.log");
    let mut file = match f {
        Ok(f) => {f}
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create("hi.log") {
                Ok(f) => f,
                Err(e) => panic!("Problem create the file {e:?}")
            } 
            other_error => {
                panic!("Problem opening the file: {other_error}");
            }
        },
    };

    let mut content = String::new();
    let _ = file.read_to_string(&mut content);
}
```

`Result<T, E>` 定义了很多方法来处理 `match` 复杂的问题。其中有一个方法教 `unwrap` ，如果 `Result` 的值是 `Ok` 那么 `unwrap` 会返回 `Ok` 中的值。如果 `Result` 的成员是 `Err`。`unwrap` 会为我们调用 `panic!` 。 `expect` 与 `unwrap` 相同只不过可以定义错误消息。

### 传播错误

当编写一个其实先会调用一些可能会失败的操作的函数时，除了在这个函数中处理错误外，还可以选择让调用者知道这个错误并决定该如何处理。这被称为 传播（propagating）错误，这样能更好的控制代码调用，因为比起你代码所拥有的上下文，调用者可能拥有更多信息或逻辑
来决定应该如何处理错误。

```rust
use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
    match read_username_from_file() {
        Ok(username) => {
            println!("{}", username)
        },
        Err(e) => {
            println!("{e:?}");
        }
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut user_name_file = match username_file_result {
        Ok(f) => f,
        Err(e) => {
            return Err(e);
        }
    };

    let mut username = String::new();
    match user_name_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => {
            return Err(e);
        }
    }
}
```

传播错误的简写 `?` 运算符

```rust
use std::{
    fs::File,
    io::{self, Read},
};

fn main() {
    match read_username_from_file() {
        Ok(username) => {
            println!("{}", username)
        },
        Err(e) => {
            println!("{e:?}");
        }
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();

    let mut username_file = File::open("hello.txt")?;
    username_file.read_to_string(&mut username)?;

    // 链式调用进一步简化代码
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
```

在 `Option<T>` 上也可以使用 `?` 运算符。

### `main` 函数的返回值

`main` 函数的返回值也接受 `Result` 类型的枚举。如果返回值为 `Ok` 程序将正常退出。如果返回 `Err` 将以非 0 值退出。 `main` 函数也可以返回 `sys::process::Termination Trait` 的类型。 

```rust
use std::error::Error;
use std::fs::File;
fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;
    Ok(())
}
```

