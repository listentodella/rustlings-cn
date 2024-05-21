// clippy3.rs
// 这里有几个更简单的 Clippy 修复，因此你可以看到它的实用性。

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    // if my_option.is_some() {
    //     my_option.unwrap();
    // }
    if let Some(x) = my_option {
        println!("get some option");
    }

    let my_arr = &[-1, -2, -3, -4, -5, -6];
    println!("My array! Here it is: {:?}", my_arr);

    //个人理解是,resize方法是直接修改的vec,然后返回(), 使用直接赋值的方式, my_empty_vec 只能得到()
    // 至于为啥换成 clear(), clippy 识别到了本次resize没有意义,不如直接clear()
    //let my_empty_vec = vec![1, 2, 3, 4, 5].resize(0, 5);
    let mut tmp = vec![1, 2, 3, 4, 5];
    tmp.clear();
    let my_empty_vec = tmp;
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // 让我们交换它俩！
    // value_a = value_b;
    // value_b = value_a;
    std::mem::swap(&mut value_a, &mut value_b);
    println!("value a: {}; value b: {}", value_a, value_b);
}
