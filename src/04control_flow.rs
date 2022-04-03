
fn if_test() {
    let number = 6;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // 报错，number 类型必须是 bool
    if number {
        println!("number was three");
    }

    let condition = true;
    // 行内使用：因为 if 是一个表达式，我们可以在 let 语句的右侧使用它
    let number = if condition { 5 } else { 6 };
    // 报错：返回类型必须相同
    let number2 = if condition { 5 } else { "six" };
}


// loop 关键字告诉 Rust 一遍又一遍地执行一段代码直到你明确要求停止。
// break 和 continue
fn loop_test() {
    loop {
        println!("again!");
        break;
    }

    // 中断外层循环，通过 循环标签（loop label）: `'counting_up`
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                // 中断外层 counting_up
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);

    // 从循环返回值
    let mut counter = 0;
    // result = 20
    let result = loop {
        counter += 1;

        if counter == 10 {
            // 使用 break 关键字返回值 counter * 2
            break counter * 2;
        }
    };
}


fn while_test() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }
}

// for...in
fn for_test() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }

    // element 是一个指针 &i32
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // range (1..4)
    // rev 反转
    for number in (1..4).rev() {
        println!("{}!", number);
        // 3!
        // 2!
        // 1!
    }
    println!("LIFTOFF!!!");
}