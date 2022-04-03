/// use 类似 import
// 在作用域中增加 use 和路径类似于在文件系统中创建软连接（符号连接，symbolic link）。
// 通过在 crate 根增加 use crate::front_of_house::hosting，现在 hosting 在作用域中就是有效的名称了，如同 hosting 模块被定义于 crate 根一样。
// 通过 use 引入作用域的路径也会检查私有性，同其它路径一样。

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// 绝对路径
use crate::front_of_house::hosting;
// 相对路径
// use front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

/// 引入习惯
// 函数：一般引入函数的父级模块到作用域
// 使用 use 引入结构体、枚举和其他项时，习惯是指定它们的完整路径
use std::collections::HashMap;

fn main1() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

// 同名冲突时，引入上级
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}

/// as: 重命名解决同名冲突
use std::fmt::Result;
use std::io::Result as IoResult;

fn function3() -> Result {
    // --snip--
}

fn function4() -> IoResult<()> {
    // --snip--
}

/// 使用 pub use 重导出名称 （类似 export）
// “重导出（re-exporting）”：我们不仅将一个名称导入了当前作用域，还允许别人把它导入他们自己的作用域

// 外部代码引入后，可以使用 hosting
// 文件名: src/lib.rs
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

/// 使用外部包
// 在 Cargo.toml 的 dependencies 中引入 `rand = "0.8.3"`
// 会自动从 https://crates.io/ 下载依赖
// 包缓存目录在 `~/.cargo`
use rand::Rng; // 引入使用

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);
}

/// 嵌套路径来消除大量的 use 行
use std::cmp::Ordering;
use std::io;
// 简化：{A, B}
use std::{cmp::Ordering, io};

use std::io;
use std::io::Write;
// 简化：{self, B}
use std::io::{self, Write};

/// 通过 glob 运算符将所有的公有定义引入作用域
// glob 运算符经常用于测试模块 tests 中，这时会将所有内容引入作用域；
// glob 运算符有时也用于 prelude（预加载） 模式；
use std::collections::*;
