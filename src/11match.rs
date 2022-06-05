enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    // 依次匹配，匹配即返回
    match coin {
        // 返回函数体
        Coin::Penny => {
            println!("Penny")
            1
        },
        // 简写返回值
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

/// 绑定值的模式
#[derive(Debug)] // 这样可以立刻看到州的名称
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // state 绑定 Quarter 的值
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main1() {
    value_in_cents(Coin::Quarter(UsState::Alaska));
}

/// 匹配 Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
fn main2() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

/// match的匹配必须穷举所有类型
// 但是可以使用 通配模式和 _ 占位符 省略，类似 default
fn main3() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
}

fn main4() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn reroll() {}
}

/// matches!宏

// Rust 标准库中提供了一个非常实用的宏：matches!，它可以将一个表达式跟模式进行匹配，然后返回匹配的结果 true or false。
// 例如，有一个动态数组，里面存有以下枚举：

enum MyEnum {
    Foo,
    Bar
}

fn main5() {
    let v = vec![MyEnum::Foo,MyEnum::Bar,MyEnum::Foo];
}
// 现在如果想对 v 进行过滤，只保留类型是 MyEnum::Foo 的元素，你可能想这么写：
v.iter().filter(|x| x == MyEnum::Foo);
// 但是，实际上这行代码会报错，因为你无法将 x 直接跟一个枚举成员进行比较。好在，你可以使用 match 来完成，但是会导致代码更为啰嗦，是否有更简洁的方式？答案是使用 matches!：

v.iter().filter(|x| matches!(x, MyEnum::Foo));

// 很简单也很简洁，再来看看更多的例子：
let foo = 'f';
assert!(matches!(foo, 'A'..='Z' | 'a'..='z'));

let bar = Some(4);
assert!(matches!(bar, Some(x) if x > 2));



