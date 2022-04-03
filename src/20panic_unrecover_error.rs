///* 错误分类
// - 可恢复: 文件未找到，可以再次尝试
// - 不可恢复：bug，索引超出范围

// 可恢复错误：Result<T, E>
// 不可恢复: panic! 宏

///* 不可恢复的错误 与  panic!
// panic! 当执行这个宏时:
// - 程序会打印出一个错误信息
// - 展开(unwind)并清理调用栈(stack)
// - 然后接着退出

// 默认展开：当出现 panic 时，程序默认会开始 展开（unwinding），这意味着 Rust 会回溯栈并清理它遇到的每一个函数的数据，不过这个回溯并清理的过程有很多工作
// ** [可选] 直接 终止（abort）：这会不清理数据就退出程序，那么程序所使用的内存需要由操作系统来清理；**可减少包大小**

// 如果你需要项目的最终二进制文件越小越好，panic 时通过在 Cargo.toml 的 [profile] 部分增加 panic = 'abort'，可以由展开切换为终止。例如，如果你想要在release模式中 panic 时直接终止：
// [profile.release]
// panic = 'abort'

fn main() {
    panic!("crash and burn");
}

fn main() {
    let v = vec![1, 2, 3];
    // 越界
    v[99];
    // hread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:4:5
    // ** note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
}


///* 查看详细的回溯信息，打印调用堆栈
// $ RUST_BACKTRACE=1 cargo run
// $ RUST_BACKTRACE=full cargo run

// 为了获取带有这些信息的 backtrace，必须启用 debug 标识。当不使用 --release 参数运行 cargo build 或 cargo run 时 debug 标识会默认启用