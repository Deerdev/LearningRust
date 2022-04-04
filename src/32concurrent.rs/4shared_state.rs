///* 共享状态并发
// 虽然消息传递是一个很好的处理并发的方式，但并不是唯一一个。Rust 支持通过共享状态来实现并发（共享内存）

// Channel 通道都类似于单所有权，因为一旦将一个值传送到通道中，将无法再使用这个值。
// 共享内存类似于多所有权：多个线程可以同时访问相同的内存位置。

///* 互斥器 Mutex 一次只允许一个线程访问数据
// 互斥器（mutex）是 mutual exclusion 的缩写，也就是说，任意时刻，其只允许一个线程访问某些数据。
// 访问互斥器中的数据:
// - 线程首先需要通过获取互斥器的 锁（lock）来表明其希望访问数据。
// - - 锁是一个作为互斥器一部分的数据结构，它记录谁有数据的排他访问权(谁对数据拥有独占访问权)。
// 因此，Mutex 通常被描述为 通过锁系统 保护（guarding）其数据。

// * Mutex 两条规则
// - 在使用数据之前尝试获取锁。
// - 处理完被互斥器所保护的数据之后，必须解锁数据，这样其他线程才能够获取锁。

///* Mutex<T>的 API
// 使用关联函数 Mutex::new 来创建一个 Mutex<T>
// - Mutex<T> 是一个智能指针

// 访问数据前，通过 lock方法来获取锁
// - 会阻塞当前线程
// - lock 可能会失败
// - 返回的是 MutexGuard （智能指针，实现了 Deref 和 Drop）
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
        // MutexGuard 生命周期结束时（drop）会自动解锁
    }

    println!("m = {:?}", m);
}

///* 在线程间共享 Mutex<T>
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Mutex::new(0);
    let mut handles = vec![];

    for _ in 0..10 {
        //! 报错，第一个循环已经把 counter 所有权转移了
        // move occurs because `counter` has type `Mutex<i32>`, which does not implement the `Copy` trait
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

///* 多线程和多所有权 -- Rc<T> 不能使用在多线程上
///* 原子引用计数 Arc<T>
// Arc<T> 和 Rc<T> 类似，它可以用于并发场景
// - A: atomic 原子的
// - 有性能损耗
// Arc<T> 和 Rc<T> API 相同
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // 不可变（可变）  Rc(RefCell)
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // 创建原子引用
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}

///* RefCell<T>/Rc<T> 与 Mutex<T>/Arc<T> 的相似性
// 因为 counter 是不可变的，不过可以获取其内部值的可变引用；这意味着 Mutex<T> 提供了内部可变性，就像 Cell 系列类型那样。正如使用 RefCell<T> 可以改变 Rc<T> 中的内容那样，同样的可以使用 Mutex<T> 来改变 Arc<T> 中的内容
// Rust 不能避免使用 Mutex<T> 的全部逻辑错误: 死锁（deadlock）