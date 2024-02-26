// errors3.rs
// 当我们尝试使用在上个练习中完成的函数 `total_cost` 时，遇到了问题。
// 它并没有工作！为什么不？我们应该怎么修复它？
// 执行 `rustlings hint errors3` 或在观察模式下使用 `hint` 子命令来获取提示。

use std::error::Error;
use std::num::ParseIntError;

//fn main() {// case1：确保只有返回()，拒绝 ？
//fn main() -> Result<(), ParseIntError>{//case2:修改返回值，但是对于错误类型，只有一种
fn main() -> Result<(), Box<dyn Error>>{//case3:修改返回值，但是错误类型，可支持多种错误
    let mut tokens = 100;
    let pretend_user_input = "8";

    //let cost = total_cost(pretend_user_input).unwrap();
    let cost = total_cost(pretend_user_input)?;
    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
    }
    Ok(())
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}
