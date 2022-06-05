/// 模块系统
/// 层级关系 从外到内
// - Package 包: Cargo 的特效，让你构建 测试 共享 crate
// - Crate 单元包: 一个模块树，它可产生一个 library 或可执行文件
// - Module 模块、use: 让你控制代码的组织、作用域、私有路径
// - Path 路径: 为 struct function 或  module 等项命名的方式

/// * Package 和 Crate

// Crate 类型：binary / library
// Crate Root: 是源代码文件，Rust 编译器从这里开始，组成你的 Crate 的根 Module

// 一个 Package:
// 包含一个 Cargo.toml, 它描述了如何构建这些 Crates
// 只能包含 0-1 个 library crate
// 可包含任意数量 binary crate
// 但至少包含一个 crate（library 或 binary）

/// Cargo 的惯例（不用配置）
// binary: `cargo new my-project`
// 文件 src/main.rs:
// - binary crate 的 crate root
// - crate 名与 package 名相同

// library: `cargo new my-lib --lib`
// 文件 src/lib.rs:
// - package 包含一个 library crate
// - library crate 的 crate root
// - crate 名与 package 名相同

// Cargo 把 crate root 文件交给 rustc 来构建 library 或 binary

// 一个 Package 可以同时包含 src/main.rs 和 src/lib.rs
// - 一个 binary crate，一个 library crate
// - 名称与 Package 名相同

// 一个 Package 可以有多个 binary crate:
// - 文件放在 src/bin
// - 每个文件是单独的 binary crate

/// * 牢记 Package 是一个项目工程，而包只是一个编译单元，基本上也就不会混淆这个两个概念了：src/main.rs 和 src/lib.rs 都是编译单元，因此它们都是包。
/*
.
├── Cargo.toml
├── Cargo.lock
├── src
│   ├── main.rs
│   ├── lib.rs
│   └── bin
│       └── main1.rs
│       └── main2.rs
├── tests
│   └── some_integration_tests.rs
├── benches
│   └── simple_bench.rs
└── examples
    └── simple_example.rs

唯一库包：src/lib.rs
默认二进制包：src/main.rs，编译后生成的可执行文件与 Package 同名
其余二进制包：src/bin/main1.rs 和 src/bin/main2.rs，它们会分别生成一个文件同名的二进制可执行文件
集成测试文件：tests 目录下
基准性能测试 benchmark 文件：benches 目录下
项目示例：examples 目录下
*/

/// * Crate 的作用
// 将相关功能组合到一个作用域内，便于在项目间进行共享，防止冲突
// 例如 rand crate, 访问它的功能需要通过它的名字: rand

/// * 定义 module 来控制作用域和私有性
// Module：
// - 在一个 crate 内，将代码进行分组
// - 增加可读性，易于复用
// - 控制项目(item)的私有性，public、private

// 建立 module：
// - 使用 mod 关键字来创建新模块，后面紧跟着模块名称
// - 模块可以嵌套，这里嵌套的原因是招待客人和服务都发生在前厅，因此我们的代码模拟了真实场景
// - 模块中可以定义各种 Rust 类型，例如函数、结构体、枚举、特征等
// - 所有模块均定义在同一个文件中

// 使用模块，我们可以将相关的定义分组到一起，并指出他们为什么相关
// src/lib.rs
mod front_of_house {
    // 块还可以保存一些定义的其他项，比如子模块、结构体、枚举、常量、特性、或者函数。
    mod hosting {
        fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

// src/main.rs 和 src/lib.rs 叫做 crate 根。之所以这样叫它们是因为这两个文件的内容都分别在 crate 模块结构的根组成了一个名为 crate 的模块，该结构被称为 模块树（module tree）。
/*
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
*/

// 这个树展示了一些模块是如何被嵌入到另一个模块的（例如，hosting 嵌套在 front_of_house 中）。
// 这个树还展示了一些模块是互为 兄弟（siblings） 的，这意味着它们定义在同一模块中（hosting 和 serving 被一起定义在 front_of_house 中）。继续沿用家庭关系的比喻，如果一个模块 A 被包含在模块 B 中，我们将模块 A 称为模块 B 的 子（child），模块 B 则是模块 A 的 父（parent）。注意，整个模块树都植根于名为 crate 的隐式模块下。
