///* trait：定义共享的行为
// 使用 trait bounds 指定泛型是任何拥有特定行为的类型。
// ** trait 类似于其他语言中的常被称为 接口（interfaces）的功能，虽然有一些不同。

// 定义 trait
pub trait Summary {
    // 一行一个方法签名且都以分号结尾。
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// 为类型实现 trait: impl <Trait> for <Stuct/Enum> {...}
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// main
// 必须引入 Summary
use aggregator::{Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    // 使用 tweet 的 trait 方法
    println!("1 new tweet: {}", tweet.summarize());
}

///* 实现 trait 的约束
// 可以在某个类型上实现某个 trait 的前提条件：
// - 这个类型或这个 trait 是在本地 crate 里定义的
// 无法为外部类型来实现外部的 trait：
// - 这个限制是程序属性的一部分（也就是一致性）
// - 更具体的说是孤儿原则：之所以这样命令是因为父类型不存在
// - 此规则确保其他人的代码不能破坏您的代码，反之亦然
// - 如果没有这个规则，两个 crate 可以为同一个类型实现同一个 trait，Rust 就不知道应该使用哪个实现了

//
///* 默认实现
// ** overwrite 的方法，不能够再调用 默认实现
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
impl Summary for NewsArticle {}

// ------------------------------------------
// 默认实现中可以调用 其他trait方法
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        // 默认实现中可以调用 其他方法
        format!("(Read more from {}...)", self.summarize_author())
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    // summarize 使用 默认实现
}

use chapter10::{self, Summary, Tweet};
// ------

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}

//
///* trait 作为参数
// impl trait 语法：适用于简单情况
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// trait bound 语法: 用于复杂情况
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// * 通过 + 指定多个 trait bound
pub fn notify(item: &(impl Summary + Display)) {}
pub fn notify<T: Summary + Display>(item: &T) {}

// * 通过 where 简化 trait bound
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
// where 后置限定类型
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
}

///* 返回实现了 trait 的类型
// 但是只适用于单一类型(返回只能是某一种类型)
// 如果还返回 NewsArticle或Tweet 就会报错（虽然他们都实现了 Summary）
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

///* 使用 trait bounds 来修复 largest 函数
// 实现 PartialOrd，用于 比较 >
// 实现 Copy，用于赋值
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

// * 如果 vec 存的是 String，没有实现 Copy，只有 Clone，以上方法不适用

// 使用 Clone trait
fn largest<T: PartialOrd + Clone>(list: &[T]) -> T {
    // list 成员的赋值，必须使用 clone，因为他是在 heap 上的，没有 copy 方法
    let mut largest = list[0].clone();

    for item in list.iter() {
        if item > &largest {
            largest = item.clone();
        }
    }

    largest
}

// 另一种 largest 的实现方式是返回在 slice 中 T 值的引用。如果我们将函数返回值从 T 改为 &T 并改变函数体使其能够返回一个引用，我们将不需要任何 Clone 或 Copy 的 trait bounds 而且也不会有任何的堆分配
// 返回引用
fn largest<T: PartialOrd + Copy>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let str_list = vec![String::from("hello"), String.from("word")];
    let result = largest(&str_list);
    println!("The largest char is {}", result);
}

//
///* 使用 trait bound 有条件地实现方法
// 通过使用带有 trait bound 的泛型参数的 impl 块，可以有条件地只为那些实现了特定 trait 的类型实现方法
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// 特定类型，添加特定方法
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

// * 也可以对任何实现了特定 trait 的类型有条件地实现 trait
// 为 Display 的 T 实现 ToString 方法
impl<T: Display> ToString for T {
    // --snip--
}
// 因为标准库有了这些 blanket implementation，我们可以对任何实现了 Display trait 的类型调用由 ToString 定义的 to_string 方法。例如，可以将整型转换为对应的 String 值，因为整型实现了 Display：
let s = 3.to_string();

// blanket implementation 会出现在 trait 文档的 “Implementers” 部分。