/// 字符串切片
fn main1() {
    let s = String::from("hello world");
    // 区间[)
    let hello = &s[0..5];
    let world = &s[6..11];

    // 可省略开头和结尾
    let slice1 = &s[..]; // 全部
    let slice2 = &s[3..];
    let slice3 = &s[..2];
}
// 注意：字符串 slice range 的索引必须位于有效的 UTF-8 字符边界内，如果尝试从一个多字节字符的中间位置创建字符串 slice，则程序将会因错误而退出。出于介绍字符串 slice 的目的，本部分假设只使用 ASCII 字符集；

// 当拥有某值的不可变引用时，就不能再获取一个可变引用
// 因为 clear 需要清空 String，它尝试获取一个可变引用。在调用 clear 之后的 println! 使用了 word 中的引用，所以这个不可变的引用在此时必须仍然有效。
// Rust 不允许 clear 中的可变引用和 word 中的不可变引用同时存在，因此编译失败。
// Rust 不仅使得我们的 API 简单易用，也在编译时就消除了一整类的错误！
fn main2() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // 错误! cannot borrow `s` as mutable because it is also borrowed as immutable

    println!("the first word is: {}", word);
}
// 提取字符串第一个单词
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}


/// 字符串字面值就是 slice
// 这里 s 的类型是 &str：它是一个指向二进制程序特定位置的 slice。这也就是为什么字符串字面值是不可变的；&str 是一个不可变引用。
let s = "Hello, world!";


/// 函数参数建议使用 &strc
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main3() {
    let my_string = String::from("hello world");

    // `first_word` 适用于 `String`（的 slice），整体或全部
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` 也适用于 `String` 的引用，
    // 这等价于整个 `String` 的 slice
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` 适用于字符串字面值，整体或全部
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // 因为字符串字面值已经 是 字符串 slice 了，
    // 这也是适用的，无需 slice 语法！
    let word = first_word(my_string_literal);
}



/// 其他类型的 slice
// 这个 slice 的类型是 &[i32]。它跟字符串 slice 的工作方式一样，通过存储第一个集合元素的引用和一个集合总长度。
fn main() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}


/// 所有权、借用和 slice 这些概念让 Rust 程序在编译时确保内存安全。Rust 语言提供了跟其他系统编程语言相同的方式来控制你使用的内存，但拥有数据所有者在离开作用域后自动清除其数据的功能意味着你无须额外编写和调试相关的控制代码。

