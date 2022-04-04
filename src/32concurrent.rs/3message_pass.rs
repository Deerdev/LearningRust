///* 使用消息传递在线程间传送数据
// 消息传递（message passing）: 一种很流行且能保证安全并发的技术
// - 线程或 actor 通过发送包含数据的消息来相互沟通
// Go: “不要通过共享内存来通讯；而是通过通讯来共享内存。”

///* Rust 中一个实现消息传递并发的主要工具是 通道（channel）
// 通道有两部分组成，一个发送者（transmitter）和一个接收者（receiver）
// 当发送者或接收者任一被丢弃时可以认为通道被 关闭（closed）了。

///* 创建 Channel
// 使用 mpsc::channel 函数创建一个新的通道；
// - mpsc 是 多个生产者，单个消费者（multiple producer, single consumer）的缩写。
// - Rust 标准库实现通道的方式意味着一个通道可以有多个产生值的 发送（sending）端，但只能有一个消费这些值的 接收（receiving）端
// * mpsc::channel 函数返回一个元组：第一个元素是发送端，而第二个元素是接收端。
// - 历史原因，tx 和 rx 通常作为 发送者（transmitter）和 接收者（receiver）的缩写
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
    // 阻塞主线程执行直到从通道中接收一个值
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

// *发送端 send 方法
// - 参数 想要发送的数据
// - send 方法返回一个 Result<T, E> 类型，所以如果接收端已经被丢弃了，将没有发送值的目标，所以发送操作会返回错误

// *接收端的方法
// recv方法，它是 receive 的缩写。这个方法会阻塞主线程执行直到从通道中接收一个值。
// - 一旦发送了一个值，recv 会在一个 Result<T, E> 中返回它。
// - 当通道发送端关闭，recv 会返回一个错误表明不会再有新的值到来了。

// try_recv 不会阻塞
// - 它立刻返回一个 Result<T, E>：Ok 值包含可用的信息，而 Err 值代表此时没有任何消息。
// * 如果线程在等待消息过程中还有其他工作时使用 try_recv 很有用：可以编写一个循环来频繁调用 try_recv，在有可用消息时进行处理，其余时候则处理一会其他工作直到再次检查。

///* 通道与所有权转移
// 所有权规则在消息传递中扮演了重要角色，其有助于我们编写安全的并发代码。
// 防止并发编程中的错误是在 Rust 程序中考虑所有权的一大优势。
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        //! 报错，无法继续使用 val，send 之后，所有权转移
        println!("val is {}", val);
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

///* 发送多个值并观察接收者的等待
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // 在主线程中，不再显式调用 recv 函数：而是将 rx 当作一个迭代器。对于每一个接收到的值，我们将其打印出来。当通道被关闭时，迭代器也将结束。
    for received in rx {
        println!("Got: {}", received);
    }
}

///* 通过克隆发送者来创建多个生产者
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // --snip--

    let (tx, rx) = mpsc::channel();

    // clone 创建多个发送端
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }

    // --snip--
}
