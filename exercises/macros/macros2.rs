// macros2.rs
// 执行 `rustlings hint macros2` 或在观察模式下使用 `hint` 子命令来获取提示。

fn main() {
    my_macro!();
}

//#[warn(unused_macros)]
//#[macro_use]
//上面两个看起来,如果不在模块内,就没有作用?
//如果不移动位置,可以用下面的方式
#[macro_export]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}
