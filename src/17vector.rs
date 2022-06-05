///*  Vec<T>，也被称为 vector。
// vector 允许我们在一个单独的数据结构中储存多于一个的值
// 它在内存中彼此相邻地排列所有的值。
// vector 只能储存相同类型的值。
// 它们在拥有一系列项的场景下非常实用，例如文件中的文本行或是购物车中商品的价格。

fn main1() {
    // new 无法推断类型，需要显示指定
    let v: Vec<i32> = Vec::new();

    // 使用 vec! 宏创建，指定推断类型
    let v = vec![1, 2, 3];
}

/// 新增 mut
// 必须使用 mut 关键字使其可变。
// 放入其中的所有值都是 i32 类型的，而且 Rust 也根据数据做出如此判断，所以不需要 Vec<i32> 注解。
fn main2() {
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
}

///* 丢弃 vector 时也会丢弃其所有元素
// 当 vector 被丢弃时，所有其内容也会被丢弃，这意味着这里它包含的整数将被清理
fn main3() {
    {
        let v = vec![1, 2, 3, 4];

        // 处理变量 v
    } // <- 这里 v 离开作用域并被丢弃
}

/// 读取 vector 的元素
// 索引语法或者 get 方法：
fn main4() {
    let v = vec![1, 2, 3, 4, 5];

    // 索引越界会 panic
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    // Option<&T>，越界返回 None
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}

//! 不可运行：同时存在 不可变引用 和 可变引用
fn main5() {
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0]; // 不可变
    v.push(6); //! 可变: cannot borrow `v` as mutable because it is also borrowed as immutable
    //* 原因在于：数组的大小是可变的，当旧数组的大小不够用时，Rust 会重新分配一块更大的内存空间，然后把旧数组拷贝过来。这种情况下，之前的引用显然会指向一块无效的内存

    println!("The first element is: {}", first);
}

/// Vec<T> 类型的更多实现细节 https://doc.rust-lang.org/nomicon/vec/vec.html

/// 遍历 vector 中的元素
// 不可变
fn main6() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
}

// 可变
fn main7() {
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        // 为了修改可变引用所指向的值，在使用 += 运算符之前必须使用解引用运算符（*）获取 i 中的值
        *i += 50;
    }
    // [150, 82, 107]
}

///* 使用枚举来储存多种类型
fn main8() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    // 存放多种类型
    // 前提是，定义好类型，明确存储空间大小（通过枚举来告诉编译器）
    // Rust 在编译时就必须准确的知道 vector 中类型的原因在于它需要知道储存每个元素到底需要多少内存
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    // 如果在编写程序时不能确切无遗地知道运行时会储存进 vector 的所有类型，枚举技术就行不通了
}

// vec API 文档: https://doc.rust-lang.org/std/vec/struct.Vec.html


/// * 基于特征对象的实现: 储存多种类型
trait IpAddr {
    fn display(&self);
}

struct V4(String);
impl IpAddr for V4 {
    fn display(&self) {
        println!("ipv4: {:?}",self.0)
    }
}
struct V6(String);
impl IpAddr for V6 {
    fn display(&self) {
        println!("ipv6: {:?}",self.0)
    }
}

fn main() {
    //* 必需手动的指定类型：Vec<Box<dyn IpAddr>>，表示数组 v 存储的是特征 IpAddr 的对象，这样就实现了在数组中存储不同的类型。
    let v: Vec<Box<dyn IpAddr>> = vec![
        Box::new(V4("127.0.0.1".to_string())),
        Box::new(V6("::1".to_string())),
    ];

    for ip in v {
        ip.display();
    }
}
