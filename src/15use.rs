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

/// * 使用 pub use 重导出名称 （类似 export）
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

/// * 嵌套路径来消除大量的 use 行
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

/// * self
// 上面使用到了模块章节提到的 self 关键字，用来替代模块自身，结合上一节中的 self，可以得出它在模块中的两个用途：
// use self::xxx，表示加载当前模块中的 xxx。此时 self 可省略
// use xxx::{self, yyy}，表示，加载当前路径下模块 xxx 本身，以及模块 xxx 下的 yyy

/// * 受限的可见性
// 如果我们想要让某一项可以在整个包中都可以被使用，那么有两种办法：
// - 在包根中定义一个非 pub 类型的 X(父模块的项对子模块都是可见的，因此包根中的项对模块树上的所有模块都可见)
// - 在子模块中定义一个 pub 类型的 Y，同时通过 use 将其引入到包根

mod a {
    pub mod b {
        pub fn c() {
            // 使用 X
            println!("{:?}", crate::X);
        }

        #[derive(Debug)]
        pub struct Y;
    }
}

#[derive(Debug)]
struct X;
use a::b::Y;
fn d() {
    println!("{:?}", Y);
}

// * 限定 a 只导出部分，通知共享内部变量 J 给 semisecret 方法
// 因为 semisecret 会被 pub 方法调用，同时 use self::b::c::J 需要访问 J
fn main() {
    pub mod a {
        pub const I: i32 = 3;

        fn semisecret(x: i32) -> i32 {
            use self::b::c::J;
            x + J
        }

        pub fn bar(z: i32) -> i32 {
            semisecret(I) * z
        }
        pub fn foo(y: i32) -> i32 {
            semisecret(I) + y
        }

        mod b {
            // * 通过 pub(in crate::a) 的方式，我们指定了模块 c 和常量 J 的可见范围都只是 a 模块中，a 之外的模块是完全访问不到它们的。
            pub(in crate::a) mod c {
                pub(in crate::a) const J: i32 = 4;
            }
        }
    }
}

/// * 限制可见性语法
// pub 意味着可见性无任何限制
// pub(crate) 表示在当前包可见
// pub(self) 在当前模块可见
// pub(super) 在父模块可见
// pub(in <path>) 表示在某个路径代表的模块中可见，其中 path 必须是父模块或者祖先模块

// 一个名为 `my_mod` 的模块
mod my_mod {
    // 模块中的项默认具有私有的可见性
    fn private_function() {
        println!("called `my_mod::private_function()`");
    }

    // 使用 `pub` 修饰语来改变默认可见性。
    pub fn function() {
        println!("called `my_mod::function()`");
    }

    // 在同一模块中，项可以访问其它项，即使它是私有的。
    pub fn indirect_access() {
        print!("called `my_mod::indirect_access()`, that\n> ");
        private_function();
    }

    // 模块也可以嵌套
    pub mod nested {
        pub fn function() {
            println!("called `my_mod::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called `my_mod::nested::private_function()`");
        }

        // 使用 `pub(in path)` 语法定义的函数只在给定的路径中可见。
        // `path` 必须是父模块（parent module）或祖先模块（ancestor module）
        pub(in crate::my_mod) fn public_function_in_my_mod() {
            print!("called `my_mod::nested::public_function_in_my_mod()`, that\n > ");
            public_function_in_nested()
        }

        // 使用 `pub(self)` 语法定义的函数则只在当前模块中可见。
        pub(self) fn public_function_in_nested() {
            println!("called `my_mod::nested::public_function_in_nested");
        }

        // 使用 `pub(super)` 语法定义的函数只在父模块中可见。
        pub(super) fn public_function_in_super_mod() {
            println!("called my_mod::nested::public_function_in_super_mod");
        }
    }

    pub fn call_public_function_in_my_mod() {
        print!("called `my_mod::call_public_funcion_in_my_mod()`, that\n> ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    // `pub(crate)` 使得函数只在当前包中可见
    pub(crate) fn public_function_in_crate() {
        println!("called `my_mod::public_function_in_crate()");
    }

    // 嵌套模块的可见性遵循相同的规则
    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called `my_mod::private_nested::function()`");
        }
    }
}

fn function() {
    println!("called `function()`");
}

fn main() {
    // 模块机制消除了相同名字的项之间的歧义。
    function();
    my_mod::function();

    // 公有项，包括嵌套模块内的，都可以在父模块外部访问。
    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();

    // pub(crate) 项可以在同一个 crate 中的任何地方访问
    my_mod::public_function_in_crate();

    // pub(in path) 项只能在指定的模块中访问
    // 报错！函数 `public_function_in_my_mod` 是私有的
    //my_mod::nested::public_function_in_my_mod();
    // 试一试 ^ 取消该行的注释

    // 模块的私有项不能直接访问，即便它是嵌套在公有模块内部的

    // 报错！`private_function` 是私有的
    //my_mod::private_function();
    // 试一试 ^ 取消此行注释

    // 报错！`private_function` 是私有的
    //my_mod::nested::private_function();
    // 试一试 ^ 取消此行的注释

    // 报错！ `private_nested` 是私有的
    //my_mod::private_nested::function();
    // 试一试 ^ 取消此行的注释
}
