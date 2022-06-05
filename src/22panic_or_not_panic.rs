// 优先 Result
// 示例、代码原型和测试都非常适合 panic
// 在我们准备好决定如何处理错误之前，unwrap和expect方法在原型设计时非常方便 (占位 抛错)

fn main1() {
    use std::net::IpAddr;
    // 明确不会抛错，直接使用 unwrap，解出 OK
    let home: IpAddr = "127.0.0.1".parse().unwrap();
}

///* 错误处理指导原则
// 有害状态是非预期的行为，与偶尔会发生的行为相对，比如用户输入了错误格式的数据。
// 在此之后代码的运行依赖于不处于这种有害状态，而不是在每一步都检查是否有问题。
// 没有可行的手段来将有害状态信息编码进所使用的类型中的情况。

///* 创建自定义类型进行有效性验证
fn main2() {
    pub struct Guess {
        value: i32,
    }

    impl Guess {
        // 验证入参
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }

            Guess { value }
        }

        pub fn value(&self) -> i32 {
            self.value
        }
    }
}


/// * 线程 panic 后，程序是否会终止？
// 长话短说，如果是 main 线程，则程序会终止，如果是其它子线程，该线程会终止，但是不会影响 main 线程。因此，尽量不要在 main 线程中做太多任务，将这些任务交由子线程去做，就算子线程 panic 也不会导致整个程序的结束。

/// * panic 原理剖析
/*
本来不想写这块儿内容，因为真的难写，但是转念一想，既然号称圣经，那么本书就得与众不同，避重就轻显然不是该有的态度。

当调用 panic! 宏时，它会

1. 格式化 panic 信息，然后使用该信息作为参数，调用 std::panic::panic_any() 函数
2. panic_any 会检查应用是否使用了 panic hook，如果使用了，该 hook 函数就会被调用（hook 是一个钩子函数，是外部代码设置的，用于在 panic 触发时，执行外部代码所需的功能）
3. 当 hook 函数返回后，当前的线程就开始进行栈展开：从 panic_any 开始，如果寄存器或者栈因为某些原因信息错乱了，那很可能该展开会发生异常，最终线程会直接停止，展开也无法继续进行
4. 展开的过程是一帧一帧的去回溯整个栈，每个帧的数据都会随之被丢弃，但是在展开过程中，你可能会遇到被用户标记为 catching 的帧（通过 std::panic::catch_unwind() 函数标记），此时用户提供的 catch 函数会被调用，展开也随之停止：当然，如果 catch 选择在内部调用 std::panic::resume_unwind() 函数，则展开还会继续。

还有一种情况，在展开过程中，如果展开本身 panic 了，那展开线程会终止，展开也随之停止。

一旦线程展开被终止或者完成，最终的输出结果是取决于哪个线程 panic：对于 main 线程，操作系统提供的终止功能 core::intrinsics::abort() 会被调用，最终结束当前的 panic 进程；如果是其它子线程，那么子线程就会简单的终止，同时信息会在稍后通过 std::thread::join() 进行收集。
*/
