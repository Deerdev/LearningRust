///* 多线程的问题
// - 竞争状态（Race conditions），多个线程以不一致的顺序访问数据或资源
// - 死锁（Deadlocks），两个线程相互等待对方停止使用其所拥有的资源，这会阻止它们继续运行
// - 只会发生在特定情况且难以稳定重现和修复的 bug

///* 实现线程的方式
// 通过调用 OS 的 API 来创建线程 - 1:1模型 (一个 OS 线程对应一个语言线程)
// - 需要较小的运行时
// 语言自己实现的线程（绿色线程）- M:N 模型
// - 需要更大的运行时

// Rust 需要权衡运行时的支持，提供尽可能小的运行时
// - 任何语言都是运行时，c/c++ 运行时比较小，所以打包的二进制也小，操作系统基本也会自带 C 的运行时
// - java 和 go 之类的是增加自己的运行时，来提供更多更丰富的功能

// * Rust 标准库只提供了 1:1 线程实现；有一些 crate 实现了其他有着不同取舍的线程模型(M:N 模型，三方实现)。

///* 使用 spawn 创建新线程
// 调用 thread::spawn 函数并传递一个闭包
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    // 主线程结束 即退出
}

///* 使用 join 等待所有线程结束
// thread::spawn 函数返回的类型是 JoinHandle
// JoinHandle 持有值的所有权
// - 当对其调用 join 方法时，它会等待其线程结束
// * 调用 handle 的 join 会阻塞当前线程直到 handle 所代表的线程结束。阻塞（Blocking） 线程意味着阻止该线程执行工作或退出
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    // 等待线程结束，block 主线程
    handle.join().unwrap();
}

///* 线程与 move 闭包
// move 闭包通常和 thread::spawn 函数一起使用，它运行你使用其它线程的数据
// 创建线程时，把值的所有权从一个线程转移到另一个线程
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        // 使用 move，否则报错
        // closure may outlive the current function, but it borrows `v`, which is owned by the current function
        println!("Here's a vector: {:?}", v);
    });

    // move之后，无法再使用 v 了
    handle.join().unwrap();
}
