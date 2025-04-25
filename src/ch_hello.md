# 你好 Rust

## 安装

第一步是 Rust 安装，我们将通过 `rustup` 来安装 Rust。

### 通过 rustup 安装

```bash
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

如果安装成功将出现下面这行

```bash
Rust is installed now. Great!
```

### 更新

```bash
rustup update
```

### 卸载

```bash
rustup self uninstall
```

## 编写程序

### 使用 cargo 创建项目

```bash
cargo new hello_world
cd hello_world
```

### 构建并运行

#### 构建程序

```bash
cargo build
```

输出

```plaintext
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
```

#### 执行程序

```bash
./target/debug/hello_world
```

输出

```plaintext
Hello, world!
```

#### 发布构建

当项目最终准备好发布时，可以使用 `cargo build --release` 来优化编译项目。
这会在 `target/release` 而不是 `target/debug` 下生成可执行文件。

```bash
cargo build --release
```

#### 线上执行程序

```bash
./target/release/hello_world
```

输出

```plaintext
Hello, world!
```

#### cargo run 构建并运行

```bash
cargo run 
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/hello_world`
```

输出

```plaintext
Hello, world!
```
