// 函数和变量名使用 snake case 规范风格, 所有字母都是小写并使用下划线分隔单词
fn another_function() {
    println!("Another function.");
}

// 必须指明参数类型
fn another_function2(x: i32) {
    println!("The value of x is: {}", x);
}

// 语句（Statements）是执行一些操作但不返回值的指令。
// 表达式（Expressions）计算并产生一个值
fn another_function3() {
    let x = 6;
    let y = {
        let x = 1;
        x + 3; // 加上';'就是语句，没有返回值, 返回 y = ()
        x + 3 // 没有 ';' 就是表达式，返回 1+3=4，y=4
    }
}


// 指定返回值
fn plus_one(x: i32) -> i32 {
    x + 1 // （不加 ';'）最后一个表达式，默认作为返回，类似 ruby
}