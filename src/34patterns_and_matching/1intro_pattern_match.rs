///* 模式与模式匹配
// 模式是 Rust 中特殊的语法，它用来匹配类型中的结构，无论类型是简单还是复杂
// 结合使用模式和 match 表达式以及其他结构可以提供更多对程序控制流的支配权。

// 模式由如下一些内容组合而成：

// 字面值
// 解构的数组、枚举、结构体或者元组
// 变量
// 通配符
// 占位符

/*
写一个模式 let x
传一个值 5
匹配：将模式和值匹配，值 赋给 模式 let x = 5
*/

///* 所有可能会用到模式的位置
///* match 分支 / Match 的 Arm
// 在形式上 match 表达式由 match 关键字、用于匹配的值和一个或多个分支构成，这些分支包含一个模式和在值匹配分支的模式时运行的表达式：
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
}
// 详尽：match 表达式必须是 穷尽（exhaustive）的，意为 match 表达式所有可能的值都必须被考虑到
// _下划线：有一个特定的模式 _ 可以匹配所有情况，不过它从不绑定任何变量。通常用于 match 的最后一个 arm 分支 或 用于忽略某些值



///* if let 条件表达式
// if let 表达式主要是作为一种简短的方式来等价的代替只有一个匹配项的 match
// if let 可选的拥有 eles, 包括： else if / else if let
// 但是，if let 不会检查穷尽性
fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        // Ok(age) 是模式，age重新赋值，是 Ok 内的 age
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

///* while let 条件循环
// 允许只要模式匹配就一直进行 while 循环
fn main() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);
    // Some(top) 是模式，匹配出 top 的值
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

/// * for 循环
// 在 for 循环中，模式是 for 关键字直接跟随的值，正如 for x in y 中的 x。

fn main() {
    let v = vec!['a', 'b', 'c'];
    // 使用 for 循环来解构，或拆开一个元组作为 for 循环的一部分：
    // (index, value) 是模式
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}


///* let 语句 也是模式
// let PATTERN = EXPRESSION;
fn main() {
    // 这里将一个元组与模式匹配 (x,y,z)是模式，(1,2,3)来匹配模式
    let (x, y, z) = (1, 2, 3);
}



///* 函数参数 也可以是模式
// x 部分就是一个模式
fn foo(x: i32) {
    // code goes here
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn main() {
    let point = (3, 5);
    // 值 &(3, 5) 会匹配模式 &(x, y)，如此 x 得到了值 3，而 y得到了值 5。
    print_coordinates(&point);
}
