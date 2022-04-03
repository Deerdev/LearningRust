/// 引用和借用
fn main1() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    // s1 依然有效
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    // s指向s1，但是没有所有权，return 后不会销毁s1
    s.len()
}// 这里，s 离开了作用域。但因为它并不拥有引用值的所有权， 所以什么也不会发生

// 注意：与使用 & 引用相反的操作是 解引用（dereferencing），它使用解引用运算符，*

// 我们将创建一个引用的行为称为 借用（borrowing）。正如现实生活中，如果一个人拥有某样东西，你可以从他那里借来。当你使用完毕，必须还回去。我们并不拥有它。
// 不可以修改借用的变量
fn main2() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world"); // error
}

// 可变引用
// 使用 mut 声明
fn main3() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}


// 可变引用有一个很大的限制：在同一时间只能有一个对某一特定数据的可变引用。这些尝试创建两个 s 的可变引用的代码会失败：
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s; // error: cannot borrow `s` as mutable more than once at a time

println!("{}, {}", r1, r2);
// 这个限制的好处是 Rust 可以在编译时就避免数据竞争。数据竞争（data race）类似于竞态条件，它可由这三个行为造成：
// 1 两个或更多指针同时访问同一数据。
// 2 至少有一个指针被用来写入数据。
// 3 没有同步数据访问的机制。

// 数据竞争会导致未定义行为，难以在运行时追踪，并且难以诊断和修复；Rust 避免了这种情况的发生，因为它甚至不会编译存在数据竞争的代码！(满足上述三种情况，编译报错)

// 规避：以使用大括号来创建一个新的作用域，以允许拥有多个可变引用，只是不能 同时 拥有
fn main4() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用

    let r2 = &mut s;
}

// 另一个限制: Rust 在同时使用可变与不可变引用时也采用的类似的规则。这些代码会导致一个错误：
fn main5() {
    let mut s = String::from("hello");

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    let r3 = &mut s; // error: 大问题 cannot borrow `s` as mutable because it is also borrowed as immutable

    println!("{}, {}, and {}", r1, r2, r3);
}

// 作用域没有重叠 就不会报错
fn main6() {
    let mut s = String::from("hello");

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{} and {}", r1, r2);
    // 此位置之后 r1 和 r2 不再使用

    let r3 = &mut s; // 没问题
    println!("{}", r3);
}



/// 悬垂（悬空）引用（Dangling References）
// 悬垂指针是其指向的内存可能已经被分配给其它持有者
// 在 Rust 中编译器确保引用永远也不会变成悬垂状态：当你拥有一些数据的引用，编译器确保数据不会在其引用之前离开作用域。
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String { //err: dangle 返回一个字符串的引用 missing lifetime specifier
    let s = String::from("hello");
    &s // 返回字符串 s 的引用, 但是这里 s 离开作用域并被丢弃。其内存被释放。&s 非法引用
}

/// 引用的规则
// 在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
// 引用必须总是有效的。