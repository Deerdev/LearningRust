///* 控制测试如何运行
// 指定命令行参数来改变 cargo test 的默认行为
// 默认行为：并行测试、运行所有测试、捕获（不显示）所有的输出，是读取与测试结果相关的输出更容易

// * 可以将一部分命令行参数传递给 cargo test，而将另外一部分传递给生成的测试二进制文件
// 运行 cargo test --help 会提示 cargo test 的有关参数，
// 而运行 cargo test -- --help 可以提示在分隔符 -- 之后使用的有关参数。( --help 会传给执行的二进制，而不是 cargo 命令，类似 npm 的 scrip 传参)


///* 并行或连续的运行测试
// 当运行多个测试时， Rust 默认使用线程来并行运行
// 如果你不希望测试并行运行，或者想要更加精确的控制线程的数量
// 传递 --test-threads 参数和希望使用线程的数量给测试二进制文件
cargo test -- --test-threads=1

///* 显示函数输出
// 默认情况下，当测试通过时，Rust 的测试库会截获打印到标准输出的所有内容。
// 比如在测试中调用了 println! 而测试通过了，我们将不会在终端看到 println! 的输出
// --show-output 打印 println! 的输出
cargo test -- --show-output


///* 通过指定名字来运行部分测试
// 向 cargo test 传递所希望运行的测试名称的参数来选择运行哪些测试(默认是全部测试)
cargo test // 运行全部测试
cargo test one_hundred // 只测试 one_hundred 一个

// * 过滤运行多个测试
// (名称模糊匹配) 可以指定部分测试的名称，任何名称匹配这个名称的测试会被运行
cargo test add // 会运行 add_two_and_two 和 add_three_and_two 两个测试（名称开头匹配）



///* 忽略某些测试
// 使用 ignore 属性来标记耗时的测试并排除他们
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
#[ignore]
fn expensive_test() {
    // 需要运行一个小时的代码
}

// 单独运行 被忽略的测试
cargo test -- --ignored