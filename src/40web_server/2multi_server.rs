///* 使用线程池改善吞吐量
// 线程池（thread pool）是一组预先分配的等待或准备处理任务的线程。
// - 当程序收到一个新任务，线程池中的一个线程会被分配任务，这个线程会离开并处理任务。
// - 其余的线程则可用于处理在第一个线程处理任务的同时处理其他接收到的任务。
// - 当第一个线程处理完任务时，它会返回空闲线程池中等待处理新任务。
// - 线程池允许我们并发处理连接，增加 server 的吞吐量。
//
// 将池中线程限制为较少的数量，以防拒绝服务（Denial of Service， DoS）攻击；
// - 如果程序为每一个接收的请求都新建一个线程，某人向 server 发起千万级的请求时会耗尽服务器的资源并导致所有请求的处理都被终止。
// 线程池中将有固定数量的等待线程。当新进请求时，将请求发送到线程池中做处理。线程池会维护一个接收请求的队列。
// - 每一个线程会从队列中取出一个请求，处理请求，接着向对队列索取另一个请求。
// - 通过这种设计，则可以并发处理 N 个请求，其中 N 为线程数。如果每一个线程都在响应慢请求，之后的请求仍然会阻塞队列，不过相比之前增加了能处理的慢请求的数量。
// * 这个设计仅仅是多种改善 web server 吞吐量的方法之一。其他可供探索的方法有 fork/join 模型和单线程异步 I/O 模型。

// ---------------------------------------------------------------------
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        //* 选择 usize 作为 size 参数的类型，因为我们知道为负的线程数没有意义
        assert!(size > 0);

        //* Rust 所提供的通道实现是多 生产者，单 消费者 的。
        // 这意味着不能简单的克隆通道的消费端来解决问题。即便可以，那也不是我们希望使用的技术；
        // 我们希望通过在所有的 worker 中共享单一 receiver，在线程间分发任务
        let (sender, receiver) = mpsc::channel();
        // * 为了在多个线程间共享所有权并允许线程修改其值，需要使用 Arc<Mutex<T>>
        // Arc 使得多个 worker 拥有接收端，而 Mutex 则确保一次只有一个 worker 能从接收端得到任务
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        // 将通道的接收端放入一个 Arc 和一个 Mutex 中。对于每一个新 worker，克隆 Arc 来增加引用计数，如此这些 worker 就可以共享接收端的所有权了。
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }
    //* 定义 execute 函数来获取一个闭包参数
    // 闭包作为参数时可以使用三个不同的 trait：Fn、FnMut 和 FnOnce。
    // 这里应该使用哪种闭包 参考标准库的 thread::spawn 的定义，因为最终需要调用它
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        // * 为存放每一个闭包的 Box 创建一个 Job 类型别名，接着在通道中发出任务
        let job = Box::new(f);
        // *使用通道向线程发送请求
        self.sender.send(job).unwrap();
    }
}

//* Worker 结构体负责从 ThreadPool 中将代码传递给线程
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}
// --snip--

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            // *使用通道向线程发送请求
            // 启动worker 时，无法知道执行的内容，所以设计成消息机制，通过消息通知来执行具体的 job（闭包）
            // * 首先在 receiver 上调用了 lock 来获取互斥器，接着 unwrap 在出现任何错误时 panic。
            // 如果互斥器处于一种叫做 被污染（poisoned）的状态时获取锁可能会失败，这可能发生于其他线程在持有锁时 panic 了且没有释放锁。
            // 在这种情况下，调用 unwrap 使其 panic 是正确的行为。请随意将 unwrap 改为包含有意义错误信息的 expect。
            while let Ok(job) = receiver.lock().unwrap().recv() {
                // lock() 锁定了互斥器, 获得锁
                // 调用 recv 会阻塞当前线程，所以如果还没有任务，其会等待直到有可用的任务
                println!("Worker {} got a job; executing.", id);

                job();
            }
        });

        Worker { id, thread }
    }
}
// ---------------------------------------------------------------------

use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
// --snip--

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    // * 为每一个请求分配线程的代码结构
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // * 为有限数量的线程创建一个类似的接口
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    // --snip--

    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5)); // 增加慢请求，单线程会阻塞
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    // --snip--

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

///* 这段代码可以编译和运行，但是并不会产生所期望的线程行为：一个慢请求仍然会导致其他请求等待执行。
/// 其原因有些微妙：Mutex 结构体没有公有 unlock 方法，因为锁的所有权依赖 lock 方法返回的 LockResult<MutexGuard<T>> 中 MutexGuard<T> 的生命周期。
/// 这允许借用检查器在编译时确保绝不会在没有持有锁的情况下访问由 Mutex 守护的资源，不过如果没有认真的思考 MutexGuard<T> 的生命周期的话，也可能会导致比预期更久的持有锁。

///* 代码使用的 let job = receiver.lock().unwrap().recv().unwrap(); 
/// 之所以可以工作是因为对于 let 来说，当 let 语句结束时任何表达式中等号右侧使用的临时值都会立即被丢弃。
/// 然而 while let（if let 和 match）直到相关的代码块结束都不会丢弃临时值。
/// 在示例 中，job() 调用期间锁一直持续，这也意味着其他的 worker 无法接受任务。「如果 Job 是耗时任务，依然会”阻塞“，其他 work 无法执行任务」