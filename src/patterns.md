# 模式匹配

## 模式

模式是 `Rust` 中的特殊语法，它用来匹配类型中的结构，无论类型是简单还是复杂。结合使用模式和 match 表达式以及其他结构可以提供更多对程序控制流的支配权。

模式由以下内容组成：

- 字面值
- 已解构的数组、枚举、结构体或元组
- 变量
- 通配符
- 占位符

## 所有可能会用到模式的位置

### match 分支

在形式上 match 表达式由 match 关键字、用于匹配的值和一个或多个分支构成，这些分支包含一个模式和在值匹配分支的模式时运行的表达式，如下所示：

```rust
match {
    pattern => expression,
    pattern => expression,
    pattern => expression,
}
```
`None` 和 `Some(i)` 
```rust
match x {
    None => None,
    Some(i) => Some(i + 1),
}
```

### if let 条件表达式

只关心一个情况的 match 表达式的简写。可以组合并匹配 `if let`、 `else if` 、`else if let` 并不要求他们互相关联。


### while let 条件循环

只要模式匹配就一直执行 while 循环。

### for 循环

### let 语句

```rust
let PATTERN = EXPRESSION;
```

变量名位于 PATTERN 所的位置，变量名是形式特别朴素的模式。我们将表达式与模式比较，并为任何找到的名称赋值。`let x = 5` 中 ，
x 代表的是 “将匹配到的值绑定到变量 x” 的模式。同时因为 x 是整个模式，这个模式实际上等于 “将任何值绑定到变量x，不管值是什么”。

### 函数参数

函数参数也可以是模式。


## 可反驳性

Refutability （可反驳性）模式是否会匹配失效。

模式有两种模式：refutable （可反驳的）和 irrefutable (不可反驳的)。能匹配任何传递的可能值的模式被称为是“不可反驳的（irrefutable）”。
对某些可能的值进行匹配会失败的模式被称为是 "可反驳的（refutable）"。

函数参数 、 let、 for 只能接受不可反驳模式，if let 、while let 可以接受可反驳和不可反驳的。