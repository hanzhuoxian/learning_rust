# 变量

变量是用于管理和存储数据空间。变量可以用来表示值、表达式或指向数据的引用。

变量三要素

- 名称: 变量的名称是唯一标识符，代表它所存储的数据。 
- 类型: 变量的类型决定了其存储的数据类型，例如整数、字符串、布尔值等。
- 值: 变量的值是其实际存储的数据。

## 可变与不可变

Rust 声明的变量默认是不可变的。只能读取不能进行赋值。

代码 1: 不可变变量

```rust
fn main() {
    let x: i32 = 10; // 声明不可变变量
    println!("{}", x); // 10
    // x = 20; // 为不可变变量赋值-编译错误
}
```

可变变量需要使用 `let mut` 声明，可变变量即可以读取也可以赋值。

代码 2: 可变变量

```rust
fn main() {
    let mut x: i32 = 10; // 可变变量
    println!("{}", x); // 10
    x = 20; // 为可变变量赋值
    println!("{}", x); // 20
}
```

## 变量隐藏

我们可以定义一个与之前同名的新变量，第一个变量被第二个隐藏了。这意味着当你使用变量的名称时，编译器将看到第二个变量。

代码 3: 变量隐藏

```rust
fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x + 2;
        println!("The value of x in inner scope is : {}", x); // 8
    }
    println!("The value of x : {}", x); // 6
}
```

## 常量

常量是绑定到一个名称的不允许改变的值，声明常量使用 `const`，并且必须注明值类型。常量总是不可变的。常量只能被设置为常量表达式。

常量命名规范：大写字母加下划线

代码 4: 常量

```rust
fn main() {
    const THREE_HOUR_IN_SECONDS: u32 = 3 * 60 * 60;
    println!("{}", THREE_HOUR_IN_SECONDS); // 10800
}
```

## 数据类型

每一个值都属于某一个数据类型，指明何种数据以便明确数据处理方式。数据类型可以分为：标量（scalar）和符合类型（compound）。

Rust 是静态类型语言，在编译时必须知道所有变量的类型。
根据值及其使用方式，编译器通常可以推断出我们想要的类型。

### 标量类型

#### 整型

是一个没有小数部分的数字。

表格 1： Rust 中的整型

|长度|有符号|无符号|
|:-----:|:----:|:----:|
|8 bit|i8|u8|
|16 bit|i16|u16|
|32 bit|i32|u32|
|64 bit|i64|u64|
|128 bit|i128|u128|
|arch|isize|usize|

代码 5: Rust 中的整型

```rust
fn main() {
    let xi8 = 127i8;
    let xu8: u8 = 255;
    println!("xi8: {}, xu8: {}", xi8, xu8);
    let xi16 = 127i16;
    let xu16: u16 = 255;
    println!("xi16: {}, xu16: {}", xi16, xu16);
    let xi32 = 127i32;
    let xu32: u32 = 255;
    println!("xi32: {}, xu32: {}", xi32, xu32);
    let xi64 = 127i64;
    let xu64: u64 = 25_5;
    println!("xi64: {}, xu64: {}", xi64, xu64);
    let xi128 = 127i128;
    let xu128: u128 = 100_100;
    println!("xi128: {}, xu128: {}", xi128, xu128);
    let x_isize = 127isize;
    let x_usize: usize = 255;
    println!("xisize: {}, xusize: {}", x_isize, x_usize);
}

```

表格 2： Rust 中整型字面值

|数字字面值|例子|
|--|--|
|Decimal(十进制)|98_222|
|Hex(十六进制)|0xff|
|Octal(八进制)|0o77|
|Binary(二进制)|0b1111_0000|
|Byte(单字节字符)|b'A'|

可以使用类型后缀来指定类型，如 `3u32`。

代码 6: 整型字面值

```rust
fn main() {
    let x_decimal = 98_222;
    println!("x_decimal: {}", x_decimal);
    let x_hex = 0xff;
    println!("x_hex: {}", x_hex);
    let x_octal = 0o77;
    println!("x_octal: {}", x_octal);
    let x_binary = 0b1111_0000;
    println!("x_binary: {}", x_binary);
    let x_byte  = b'A';
    println!("x_byte: {}", x_byte);

}
```

#### 浮点型

浮点数是带小数点的数字。浮点数类型为 `f32` 和 `f64`。默认是 `f64`。

代码 7: 浮点型

```rust
fn main() {
    let f = 2.0;
    println!("f {}", f);
    let f:f32 = 3.0;
    println!("f {}", f);
}
```

#### 布尔型

布尔类型用来表示真和假，使用 `bool` 关键字，只有两个肯能的值 `true` 和 `false`

代码 8: 布尔型

```rust
fn main() {
    let t = true;
    println!("t {}", t);
    let t: bool = false;
    println!("t {}", t);
}
```

#### 字符型

字符类型是语言中最原生的字母类型，用关键字 `char` 表示。`char` 字面量用单引号声明。

代码 9： 字符型

```rust
fn main() {
    let c = 'z';
    let z: char ='ℤ'; // with 
    let heart_eyed_cat ='😻'
}
```

### 复合类型

将多个值组合成一个复合类型。Rust 有两个原生的复合类型，元组（tuple）和数组（array）。

#### 元组

元组是一个将多个其他类型的值组合进一个复合类型的主要方式。元组长度固定，一旦声明其长度不会增大或缩小。

使用使用在圆括号中的逗号分隔的值列表来创建一个元组。元组的每一个位置都有一个类型，而且这些不同值的类型也不必时相同的。

代码 10: 元组

```rust
fn main() {
    // 元组
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{} {} {}", tup.0, tup.1, tup.2);

    // 使用模式匹配来解构元组
    let (x, y, z) = tup;
    println!("{} {} {}", x, y, z);
}

```

不带任何值的元组被称为 单元(unit) 元组。这种值以及对应的值都写做 `()`。表示空值或者空的返回类型。如果表达式不返回其他任何值，则会隐式返回单元值。

#### 数组

数组可以包含相同类型的多个值，数组的长度式固定的。

可以像这样来编写数组类型 `[i32; 5]` ，在方括号中包含每个元素的类型，后跟分号，再跟数组元素的数量。

可以像这样来创建数组的值 `[1, 2, 3]`，使用在方括号中用逗号分隔的值列表来创建一个数组。还可以使用 `[3; 5]` 来创建每个元素都为相同值的数组。

--end--
