/// 集成测试对于你的库来说则完全是外部的。它们与其他外部代码一样，通过相同的方式使用你的代码，只测试公有接口而且每个测试都有可能会测试多个模块。
// 集成测试在不同的目录，不需要 #[cfg(test)] 标注
// 集成测试的目的是测试库的多个部分能否一起正常工作

// * 创建一个 tests 目录, 和 src 同级
// 在这个目录中创建任意多的测试文件，Cargo 会将每一个文件当作单独的 crate 来编译。

// 文件名: tests/integration_test.rs
use adder; // 引入 src 的代码

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}

///* 运行指定的集成测试
// 指定集成测试名称
cargo test --test integration_test


///* 集成测试中的子模块
// 因为一个测试就是一个 crate，所以需要提取公共代码，所有集成测试共享
// 创建 tests/common/mod.rs, 书写公共方法；common 是一种 Rust 的命名规范，这样命名告诉 Rust 不要将 common 看作一个集成测试文件

// 文件名: tests/integration_test.rs
use adder;

mod common; // 引入公共模块

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}

///* 二进制 crate 的集成测试
// 如果项目是二进制 crate 并且只包含 src/main.rs 而没有 src/lib.rs，这样就不可能在 tests 目录创建集成测试并使用 extern crate 导入 src/main.rs 中定义的函数。
// 只有库 crate 才会向其他 crate 暴露了可供调用和使用的函数；二进制 crate 只意在单独运行。

// 为什么 Rust 二进制项目的结构明确采用 src/main.rs 调用 src/lib.rs 中的逻辑的方式？因为通过这种结构，集成测试 就可以 通过 extern crate 测试库 crate 中的主要功能了，而如果这些重要的功能没有问题的话，src/main.rs 中的少量代码也就会正常工作且不需要测试。






