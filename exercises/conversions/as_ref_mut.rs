// AsRef 和 AsMut 允许廉价的 引用-到-引用 转换。
// 阅读更多： https://doc.rust-lang.org/std/convert/trait.AsRef.html
// 和 https://doc.rust-lang.org/std/convert/trait.AsMut.html，分别地。
// 执行 `rustlings hint as_ref_mut` 或在观察模式下使用 `hint` 子命令来获取提示。

// 获取给定参数的字节数（不是字符数）
fn byte_counter<T>(arg: T) -> usize
where
    T: AsRef<str>,
{
    arg.as_ref().as_bytes().len()
}

// 获取给定参数的字符数（不是字节数）
fn char_counter<T>(arg: T) -> usize
where
    T: AsRef<str>,
{
    arg.as_ref().chars().count()
}

// 使用 as_mut() 将数字平方。
fn num_sq<T>(arg: &mut T)
where
    T: AsMut<u32>,
{
    //solution1:ok
    *arg.as_mut() *= *arg.as_mut()

    //solution2: ok
    // arg is &u32 here
    // let arg = arg.as_mut();
    // *arg = arg.wrapping_pow(2);

    //solution3: error!
    //because arg is T, but right value is u32
    //*arg = arg.as_mut().wrapping_pow(2);
}

fn mult_box() {
    let mut num: Box<u32> = Box::new(3);
    num_sq(&mut num);
    assert_eq!(*num, 9);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mult_box() {
        let mut num = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}
