///* Result 枚举
// T 代表成功时返回的 Ok 成员中的数据的类型，
// E 代表失败时返回的 Err 成员中的错误的类型
enum Result<T, E> {
    Ok(T),
    Err(E),
}

use std::fs::File;

fn main1() {
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}

///* 匹配不同的错误
use std::fs::File;
use std::io::ErrorKind;

fn main2() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            // 不存在，继续 match 创建文件
            ErrorKind::NotFound => match File::create("hello.txt") {
                // 创建文件依然需要处理 err
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            // 非 NotFound 错误，直接 panic
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
}

// ** 使用闭包优化 match 的写法
fn main3() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

//
///* 失败时 panic 的简写：unwrap 和 expect
fn main4() {
    // unwrap: 如果 Result 值是成员 Ok，unwrap 会返回 Ok 中的值。如果 Result 是成员 Err，unwrap 会为我们调用 panic!
    let f = File::open("hello.txt").unwrap();
    // expect: 提供错误信息的自定义
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

///* 传播错误，抛出错误
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    // match 没有`;`, 是函数的返回结果
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

///* ?操作符
// 如果 Result 的值是 Ok，这个表达式将会返回 Ok 中的值而程序将继续执行。
// 如果值是 Err，Err 中的值将作为整个函数的返回值
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file2() -> Result<String, io::Error> {
    // ?: Ok 有值就赋值给 f，Err 就直接返回
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

///* ? 与 from
// ** 如果 ? 返回的 Err 类型不同呢？（不是 io:Error）
// ? 运算符所使用的错误值会被传递给了 from 函数 (from 定义于标准库的 From trait 中, 用来将错误从一种类型转换为另一种类型)
// 当 ? 运算符调用 from 函数时，收到的错误类型被转换为由错误当前函数返回类型所指定的类型（转换）

// ** ErrorA.from() -> ErrorB : ErrorA 必须实现转换为 ErrorB 的 trait
// 这在当函数返回单个错误类型来代表所有可能失败的方式时很有用，即使其可能会因很多种原因失败。
// 只要每一个错误类型都实现了 from 函数来定义如何将自身转换为返回的错误类型，? 运算符会自动处理这些转换。

// ** ?链式调用
fn read_username_from_file3() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

// 甚至还有一个更短的写法 fs 封装：
use std::fs;
use std::io;

fn read_username_from_file4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// 将文件读取到一个字符串是相当常见的操作，所以 Rust 提供了名为 fs::read_to_string 的函数，它会打开文件、新建一个 String、读取文件的内容，并将内容放入 String，接着返回它。
// 当然，这样做就没有展示所有这些错误处理的机会了，所以我们最初就选择了艰苦的道路。

// ** ? 运算符只能被用于返回值与 ? 作用的值相兼容的函数
// 函数返回类型为 Result 或 Option
// Option 的场景
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

///* main 函数也可以指定 Result 返回类型
use std::error::Error;
use std::fs::File;

// 空元组类型？
// Box<dyn Error> 类型是一个 trait 对象: 可理解为 “任何类型的错误”
// 进程的退出：main 成功退出的程序会返回整数 0，运行错误的程序会返回非 0 的整数
fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;

    Ok(())
}
