# 测试

测试函数是用来验证非测试代码是否能按照预期的方式进行。

测试函数的流程

- 设置任何所需的数据或者状态。
- 运行需要测试的代码
- 断言结果

Rust 中测试函数就是带有 test 属性注解的函数。

## 最简单的测试函数

```rust

#[test]
fn assert() {
    assert!(true);
}
```

## 测试模块组织的测试函数

```rust
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }
}
```

## 自定义错误消息

`assert!` 第二个之后的参数都会传给 `format!`

```rust
    #[test]
    fn assert_msg() {
        assert!(false, "I am false {}", "ff");
    }
```

## 断言 panic

expected 参数可以指定 panic 的错误信息。

```rust
    #[test]
    #[should_panic(expected="div_zero")]
    fn div_zero() {
        panic!("div_zero");
    }
```

## 返回 `Result<T, E>` 用于测试

在测试通过时返回 `Ok` 失败时返回 `Err`

```rust
    #[test]
    fn result() -> Result<(), String> {
        let i = 4; 
        if i == 3 {
            Ok(())
        } else {
            Err(String::from("error"))
        }
    }
```

## 控制测试运行

cargo test 在测试模式下编译代码并运行生成的测试二进制文件。cargo test 默认并发运行所有的测试。
可以将一部分参数传递给 cargo test ，另一部分传递给测试二进制程序。使用 -- 进行分割。

## 显示函数输出

在测试中调用了 `println!` 而测试通过了我们不会看到任何输出，如果测试失败了将看到所有标准输出和其他错误信息。

## 指定名称运行部分测试

```bash
# 运行单个测试 名称支持全匹配和部分匹配
cargo test assert_msg
```

## 忽略测试

```rust

#[test]
#[ignore]
fn assert() {
    assert!(true);
}
```

## 只运行忽略的测试

```bash
# 运行全部测试不包含 ignored 的测试
cargo test
# 只运行 ignored 的测试
cargo test -- --ignored
# 运行所有测试
cargo test -- --include-ignored
```

## 单元测试

单元测试的目的是在与其他部分隔离的环境中测试每一个单元的代码，以便于快速而准确地验
证某个单元的代码功能是否符合预期。单元测试与它们要测试的代码共同存放在位于 src 目录
下相同的文件中。规范是在每个文件中创建包含测试函数的 tests 模块，并使用 cfg(test)
标注模块。

`#[cfg(test)]` 注解告诉 Rust 只在执行 cargo test 时才编译和运行测试代码。而在运行 cargo build 时不需要

## 集成测试

集成测试需要新建 `tests` 目录与 `src` 目录同级。可以在这个目录中新建测试文件，会将每个文件当作一个 crate 来编译。

```bash

cargo test --test 集成测试文件名
```