fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        //* 匹配命名变量
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}

fn main() {
    let x = 1;

    match x {
        //* 多个模式
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn main() {
    let x = 5;

    match x {
        //* 通过 ..= 匹配值的范围: ..= 语法允许你匹配一个闭区间范围内的值 1-5
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

//
///* 解构
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };
    // 结构结构体：a = x, b = y
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    // 直接使用同名变量 x,y
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
}

fn main() {
    let p = Point { x: 0, y: 7 };

    match p {
        // 第一个分支通过指定字段 y 匹配字面值 0 来匹配任何位于 x 轴上的点
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        // 第二个分支通过指定字段 x 匹配字面值 0 来匹配任何位于 y 轴上的点
        Point { x: 0, y } => println!("On the y axis at {}", y),
        // 第三个分支没有指定任何字面值，所以其会匹配任何其他的 Point 并为 x 和 y 两个字段创建变量
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}

/// 解构枚举
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        //* 解构枚举
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
    }
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn main() {
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        //* 解构嵌套的结构体和枚举
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
        _ => (),
    }
}

// 解构结构体和元组
fn main() {
    struct Point {
        x: i32,
        y: i32,
    }

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
}

///* 忽略模式中的值
// 使用 _ 忽略整个值
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

fn main() {
    foo(3, 4);
}

fn main() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        //* 使用嵌套的 _ 忽略部分值, 是 Some 就行，不管是什么值
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);
}
fn main() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        // 忽略某几个值
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        }
    }
}

//* 通过在名字前以一个下划线开头来忽略未使用的变量
fn main() {
    let _x = 5;
    let y = 10;

    let s = Some(String::from("Hello!"));

    //! _s 仍会将值绑定到变量
    if let Some(_s) = s {
        println!("found a string");
    }
    // 只有 _ 会被忽略
    // if let Some(_) = s {
    //     println!("found a string");
    // }
    //! 会得到一个错误，因为 s 的值仍然会移动进 _s，并阻止我们再次使用 s
    //! 使用 _ 解构 不会发生所有权转移
    println!("{:?}", s);
}

//* 用 .. 忽略剩余值
fn main() {
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        // 2, 32
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        }
    }

    match numbers {
        //! 报错，无法确定中间值的位置
        (.., second, ..) => {
            println!("Some numbers: {}", second)
        },
    }
}

///* 匹配守卫提供的额外条件
// 匹配守卫（match guard）是一个指定于 match 分支模式之后的额外 if 条件，它也必须被满足才能选择此分支。匹配守卫用于表达比单独的模式所能允许的更为复杂的情况。
fn main() {
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        // 可以在匹配守卫中使用外部的 y
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {}", x, y);
}

fn main() {
    let x = 4;
    let y = false;

    match x {
        //* 使用 或 运算符 | 来指定多个模式，同时匹配守卫的条件会作用于所有的模式
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}

///* @ 绑定
// at 运算符（@）允许我们在创建一个存放值的变量的同时测试其值是否匹配模式。
// 使用 @ 可以在一个模式中同时测试和保存变量值。
fn main() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            // @ 赋值给 id_variable: `Found an id in range: 5`
            // 通过在 3..=7 之前指定 id_variable @，我们捕获了任何匹配此范围的值并同时将该值绑定到变量 id_variable 上。
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}

/// @前绑定后解构(Rust 1.56 新增)
// 使用 @ 还可以在绑定新变量的同时，对目标进行解构：
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // 绑定新变量 `p`，同时对 `Point` 进行解构
    let p @ Point {x: px, y: py } = Point {x: 10, y: 23};
    println!("x: {}, y: {}", px, py);
    println!("{:?}", p);

 
    let point = Point {x: 10, y: 5};
    if let p @ Point {x: 10, y} = point {
        println!("x is 10 and y is {} in {:?}", y, p);
    } else {
        println!("x was not 10 :(");
    }
}
