/*
* 并发模型
- OS 线程: 不需要对编程模型进行任何修改，这使你可非常方便的进行并发编程。 然而，在线程之间进行同步很困难，且带来的性能开销很大。线程池可减少一些开销 但并不足以满足海量的 I/O 密集工作负载。
- 事件驱动编程(Event-driven programming): 与回调结合来使用，可以非常高效， 但往往会导致冗长的，非线性的控制流。数据流和错误信息通常难以追踪。
- 协程Coroutines: 和线程一样，不需要对编程模型进行任何修改，这使得使用它变得非常简单。 同时和异步一样，它也可以支持海量的任务。但是，它抽象出了对系统编程来说很重要 的低级细节与自定义运行时的执行器。
- actor 模型: 将所有的并发计算划分成actor单元，这使得错误信息的传递变得简单 ，就和分布式系统一样。actor 模型可以有效的实现并发编程，但它留下了许多未解决的 实际问题，如流的控制和重试逻辑。
 */

/// * Rust 并发模型 - 提供技术实现，由上层选择单线程模式还是多线程模式
// - Futures是惰性的：Rust 中的 Futures 只有在进行轮询时，才会执行，删除 future 会停止其进一步 执行。
// - Rust 中 Async 是零开销/零成本 的，这意味着你只需要为你所使用的东西付出代价，具体来讲， 你没有在 async 过程中进行堆的分配和动态调度，这可使性能得到充分的发挥！ 这让你可以在资源有限的环境中使用 async，如嵌入式系统中。
// - Rust 并未提供内置运行时环境，而是由社区维护的 crates 提供。
// - Rust 中提供了 单线程和多线程 的运行时环境，它们各有不同的优势与缺点。

/// * Rust 中的异步 vs 线程
// Rust 中异步的首选替代是使用 OS 线程，可以直接通过 std::thread 或者间接通过线程池来使用。
// - OS 线程 适合少量任务，因为线程会有 CPU 和内存开销。生成和切换线程是代价相当昂贵，甚至闲置的线程也会消耗系统资源。一个线程池库可以减轻这些开销，但并不能全部健康。然而，线程能让你重新利用存在的同步代码，而不需要大改源代码——不需要特别的编程模型。一些操作系统中，你也可以改变线程的优先级，这对于驱动或者其他延迟敏感的应用很有用。
//
// - 异步 极大地降低了 CPU 和内存开销，尤其是在负载大量越过 IO 边界的任务，例如服务器和数据库。同样，你可以处理比 OS 线程更高数量级的任务，因为异步运行时使用少量（昂贵的）线程来处理大量（便宜的）任务。然而，异步 Rust 会导致更大的二进制体积，因为异步函数会生成状态机，并且每个可执行文件都会绑定一个异步运行时。
//
// - 最后一点，异步编程并没有 更优于 线程模型，不过它们是不一样的。如果你不需要由于性能原因使用异步，线程通常是个更简单的替换。
// * 线程并发下载
fn get_two_sites() {
    // 生成两个线程来下载网页.
    let thread_one = thread::spawn(|| download("https:://www.foo.com"));
    let thread_two = thread::spawn(|| download("https:://www.bar.com"));

    // 等待两个线程运行下载完成.
    thread_one.join().expect("thread one panicked");
    thread_two.join().expect("thread two panicked");
}

// * 异步下载
// 这里没有创建额外的线程。此外，所有函数调用都是静态分发的，也没有堆分配！
async fn get_two_sites_async() {
    // 创建两个不同的 "futures", 当创建完成之后将异步下载网页.
    let future_one = download_async("https:://www.foo.com");
    let future_two = download_async("https:://www.bar.com");

    // 同时运行两个 "futures" 直到完成.
    join!(future_one, future_two);
}

/// * Rust 中的自定义并发模型
// - Rust 不会强制你从线程模型和异步模型中间只选一个。你可以在同一个应用里同时使用两个模型，这在你混合了线程化的和异步的依赖时非常有用
// - 你甚至可以同时使用不同的并发模型，例如事件驱动编程，只要你能找到一个实现它的库

/// * 异步 Rust 编程目前状态
// 部分文档，还在不断地的开发中
// 特点：
// - 为典型的并发负载提供优秀的运行时性能
// - 更频繁地与先进的语言特性交互，例如生命周期（lifetime）和固定（pinning）
// - 一些兼容性保证，例如同步和异步代码之间，以及不同异步运行时之间的兼容性。
// - 更高的维护负担，因为异步运行时和语言支持都在持续演进。

/// * 语言和库支持
// 尽管 Rust 自身提供了异步编程支持，大部分异步应用基于社区库（community crates）提供的功能
// - 标准库提供了最基础的 traits、类型（types）和函数（functions）, 例如 Future trait
// - async/await 语法由 Rust 编译器直接支持。
// - futures 库提供很多实用类型、宏和函数。他们可以用在任何异步 Rust 应用
// - 异步代码的执行、IO 和任务生成均由 “异步运行时async runtimes” 提供支持，例如 Tokio 和 async-std. 多数异步应用，和一些异步库，都只依赖于一个特定的运行时，详情参见“异步生态”
//
// * 注意
// - Rust 不允许你在 trait 中声明异步函数

/// * 编译与调试
// * 编译错误
// 异步 Rust 中的编译错误遵循和同步 Rust 一样的高标准，但因为异步 Rust 通常依赖于更复杂的语言特性，例如生命周期和固定（pinning），你可能更频繁遇到这些错误。

// * 运行时错误
// 无论编译器在什么时候看到一个异步函数，它会在底层生成一个状态机。异步 Rust 中的堆栈追踪通常会包含这些状态机的细节，以及来自运行时的函数调用。因此对照堆栈追踪信息也会相比同步 Rust 更可能需要关注。

// * 新失败模式
// 好几种新的失败模式可以在异步 Rust 中使用。举例来说，如果你从异步上下文中调用一个阻塞函数，或者检查是否正确地实现了 Future trait。这些错误会静默传递到编译器，有时会甚至会传到单元测试中 (通过编译和单元测试)。

///* 兼容性考虑
// 异步的和同步的代码不总是能自由地结合在一起
// - 例如，你不能直接在同步函数里直接调用一个异步函数。
//
// 异步代码之间也不总是能自由地结合在一起。
// - 一些crate库依赖于特定async运行时来提供功能
//
// * 这些兼容性问题会限制你权衡，所以要尽早调查要使用哪个异步运行时和那些库

/// * 性能特征
// 异步Rust的性能取决于您所使用的异步运行时的实现。
// 尽管为异步Rust应用程序提供支持的运行时相对较新，但对于大多数实际工作负载而言，它们仍然表现出色。

/// async/await 入门
// async/.await是Rust内置语法，用于让异步函数编写得像同步代码。
// - async将代码块转化成 实现了Future trait 的状态机。
// - 使用同步方法调用阻塞函数会阻塞整个线程，但阻塞的 Future 将让出（yield）线程控制权，让其他Future继续执行。
//
// 使用`futures` crate
async fn do_something() {}
// * async fn函数返回实现了Future的类型。为了执行这个Future，我们需要执行器（executor）
use futures::executor::block_on;

async fn hello_world() {
    println!("hello, world!");
}

fn main1() {
    // 创建 future
    let future = hello_world();
    // block_on 阻塞
    block_on(future); // `future` is run and "hello, world!" is printed
}

// * 在async fn函数中， 你可以使用.await来等待其他实现了Future trait 的类型完成
// 和block_on不同，.await不会阻塞当前线程，而是异步地等待 future完成，在当前future无法进行下去时，允许其他任务运行
async fn learn_song() -> Song {}
async fn sing_song(song: Song) {}
async fn dance() {}
fn main2() {
    // block_on 阻塞执行
    let song = block_on(learn_song());
    block_on(sing_song(song));
    block_on(dance());
}
//
// 完全异步化
async fn learn_and_sing() {
    // Wait until the song has been learned before singing it.
    // We use `.await` here rather than `block_on` to prevent blocking the
    // thread, which makes it possible to `dance` at the same time.
    let song = learn_song().await;
    sing_song(song).await;
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();

    // `join!` is like `.await` but can wait for multiple futures concurrently.
    // If we're temporarily blocked in the `learn_and_sing` future, the `dance`
    // future will take over the current thread. If `dance` becomes blocked,
    // `learn_and_sing` can take back over. If both futures are blocked, then
    // `async_main` is blocked and will yield to the executor.
    futures::join!(f1, f2);
}

fn main3() {
    block_on(async_main());
}
