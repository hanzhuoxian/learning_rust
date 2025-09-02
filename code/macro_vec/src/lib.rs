#[macro_export] // 表明只要导入了该crate就可以使用该宏
macro_rules! my_vec { // macro_rules! 定义宏 vec 宏名称
    ($($x:expr),*) => { // 与 match 类似，$x:expr 表示匹配一个表达式，$()表示重复，*表示重复0次或多次
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
