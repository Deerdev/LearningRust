/// * Future trait
// Future trait 是 Rust 异步编程中心内容。
// 它是一种异步计算，可以产生值（尽管这个值可以为空， 如 ()）。
// 实现了 Future 的类型标识 目前可能还不可用的值（将来会有值）
//
// 简化版 future trait看起来可能像这样：
trait SimpleFuture {
    // 最终产生的类型
    type Output;
    // 推进 产出结果，结果是 Poll 枚举 - 两种状态
    // wake 是函数指针
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

enum Poll<T> {
    Ready(T),
    Pending,
}
// Future 可以表示：
// - 下一次网络数据包的到来
// - 下一次鼠标的移动
// - 或者仅仅是经过一段时间的时间点
//
// * Future 代表这一种你可以 检验其是否完成 的操作
// * Future 可以通过调用 poll 函数来取得进展
/// * poll 方法
// Future 能通过调用 poll 的方式推进，这会尽可能地推进 future 到完成状态。
// - 如果 future 完成了，那就会返回 poll::Ready(result)。
// - 如果 future 尚未完成，则返回 poll::Pending，并当 Future 准备好取得更多进展（通常是一个事件，比如 IO 完成）时调用一个 Waker（唤醒者） 的 wake() 函数（相当于是这个Future 事件的回调函数）。
// * 针对 Future，你唯一能做的就是使用 poll 来敲它，直到一个值掉出来（或发送错误）

/// * wake()
// - 当 wake() 调用时，执行器将驱动 Future 再次调用 poll，以便 Future 能取得更多的进展
// - 没有 wake() 函数的话，执行器将无从获知一个future 是否能有所进展（就得不断的 poll）
// 通过 wake() 函数，执行器就能知道哪些 future 已经准备好进行 poll()的调用。

pub struct SocketRead<'a> {
    socket: &'a Socket,
}

impl SimpleFuture for SocketRead<'_> {
    type Output = Vec<u8>;

    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        if self.socket.has_data_to_read() {
            // The socket has data -- read it into a buffer and return it.
            Poll::Ready(self.socket.read_buf())
        } else {
            // The socket does not yet have data.
            //
            // Arrange for `wake` to be called once data is available.
            // When data becomes available, `wake` will be called, and the
            // user of this `Future` will know to call `poll` again and
            // receive data.
            // 注册 wake，当 socket 有数据可读时，通知执行器，执行 Future 的 poll 函数
            self.socket.set_readable_callback(wake);
            Poll::Pending
        }
    }
}

// * 组合多个异步操作，而无需中间分配
// 可以通过无分配的状态机来实现多个 Future 同时运行或串联运行
/// A SimpleFuture that runs two other futures to completion concurrently.
///
/// Concurrency is achieved via the fact that calls to `poll` each future
/// may be interleaved, allowing each future to advance itself at its own pace.
pub struct Join<FutureA, FutureB> {
    // Each field may contain a future that should be run to completion.
    // If the future has already completed, the field is set to `None`.
    // This prevents us from polling a future after it has completed, which
    // would violate the contract of the `Future` trait.
    a: Option<FutureA>, // 当 Future 执行完成时，置空
    b: Option<FutureB>,
}

impl<FutureA, FutureB> SimpleFuture for Join<FutureA, FutureB>
where
    FutureA: SimpleFuture<Output = ()>,
    FutureB: SimpleFuture<Output = ()>,
{
    type Output = ();
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        // Attempt to complete future `a`.
        if let Some(a) = &mut self.a {
            if let Poll::Ready(()) = a.poll(wake) {
                self.a.take(); // 使用 take 置空
            }
        }

        // Attempt to complete future `b`.
        if let Some(b) = &mut self.b {
            if let Poll::Ready(()) = b.poll(wake) {
                self.b.take(); // 使用 take 置空
            }
        }

        // none，全部完成
        if self.a.is_none() && self.b.is_none() {
            // Both futures have completed -- we can return successfully
            Poll::Ready(())
        } else {
            // One or both futures returned `Poll::Pending` and still have
            // work to do. They will call `wake()` when progress can be made.
            Poll::Pending
        }
    }
}
//
// *  多个连续的 Future 可以一个接一个的运行
/// A SimpleFuture that runs two futures to completion, one after another.
//
// Note: for the purposes of this simple example, `AndThenFut` assumes both
// the first and second futures are available at creation-time. The real
// `AndThen` combinator allows creating the second future based on the output
// of the first future, like `get_breakfast.and_then(|food| eat(food))`.
pub struct AndThenFut<FutureA, FutureB> {
    first: Option<FutureA>,
    second: FutureB,
}

impl<FutureA, FutureB> SimpleFuture for AndThenFut<FutureA, FutureB>
where
    FutureA: SimpleFuture<Output = ()>,
    FutureB: SimpleFuture<Output = ()>,
{
    type Output = ();
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        if let Some(first) = &mut self.first {
            match first.poll(wake) {
                // We've completed the first future -- remove it and start on
                // the second!
                Poll::Ready(()) => self.first.take(),
                // We couldn't yet complete the first future.
                // 第一个未执行完，返回
                Poll::Pending => return Poll::Pending,
            };
        }
        // Now that the first future is done, attempt to complete the second.
        self.second.poll(wake)
    }
}

/// * 真正的 Future trait
trait Future {
    type Output;
    fn poll(
        // Note the change from `&mut self` to `Pin<&mut Self>`:
        self: Pin<&mut Self>,
        // and the change from `wake: fn()` to `cx: &mut Context<'_>`:
        cx: &mut Context<'_>,
    ) -> Poll<Self::Output>;
}
// self 参数类型不再是 mut self 而是 Pin<&mut Self>
// - - 使用它可以创建一个无法被移动的 Future ，因为无法被移动，因此它将具有固定的内存地址，意味着我们可以存储它的指针(如果内存地址可能会变动，那存储指针地址将毫无意义！)
// - - 不可移动对象可以在他们的字段间存储指针，例如实现一个自引用数据结构：struct MyFut { a: i32, ptr_to_a: *const i32 }。
// - 需要启用 async/await, pin 就是必须的
//
// wake: fn() 变成了 &mut Context<'_>
// - 在 SimpleFuture 里:
// - - 我们调用函数指针（fn()）来告诉执行器有future需要轮询。
// - - 然而，因为 fn() 是仅仅是个函数指针，它不能储存任何信息说明哪个 Future 调用了 wake。
// - 在正式场景要进行 wake ，就必须携带上数据。 而 Context 类型通过提供一个 Waker 类型的值，就可以用来唤醒特定的的任务
// - - 意味着 wake 函数可以携带数据了，为何要携带数据？考虑一个真实世界的场景，一个复杂应用例如web服务器可能有数千连接同时在线，那么同时就有数千 Future 在被同时管理着，如果不能携带数据，当一个 Future 调用 wake 后，执行器该如何知道是哪个 Future 调用了 wake ,然后进一步去 poll 对应的 Future ？没有办法！那之前的例子为啥就可以使用没有携带数据的 wake ？ 因为足够简单，不存在歧义性。

/// * 使用 Waker 来唤醒任务
// - 对于 Future 来说，第一次被 poll 时无法完成任务是很正常的
// - 但它需要确保在未来一旦准备好时，可以通知执行器再次对其进行 poll 进而继续往下执行，该通知就是通过 Waker 类型完成的。
// - Waker 提供了一个 wake() 方法可以用于告诉执行器：相关的任务可以被唤醒了，此时执行器就可以对相应的 Future 再次进行 poll 操作。
//
//* 在Future和等待的线程间共享状态
use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};
pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

struct SharedState {
    // 定时(睡眠)是否结束
    completed: bool,
    // 当睡眠结束后，线程可以用`waker`通知`TimerFuture`来唤醒任务
    waker: Option<Waker>,
}
impl Future for TimerFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // 通过检查共享状态，来确定定时器是否已经完成
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            Poll::Ready(())
        } else {
            // 设置`waker`，这样新线程在睡眠(计时)结束后可以唤醒当前的任务，接着再次对`Future`进行`poll`操作,
            //
            // 下面的`clone`每次被`poll`时都会发生一次，实际上，应该是只`clone`一次更加合理。
            // 选择每次都`clone`的原因是： `TimerFuture`可以在执行器的不同任务间移动，如果只克隆一次，
            // 那么获取到的`waker`可能已经被篡改并指向了其它任务，最终导致执行器运行了错误的任务
            // N.B. it's possible to check for this using the `Waker::will_wake` function, but we omit that here to keep things simple.
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}
impl TimerFuture {
    /// 创建一个新的`TimerFuture`，在指定的时间结束后，该`Future`可以完成
    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));

        // 创建新线程
        // clone Arc 类型，指针指向还是 shared_state
        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            // 睡眠指定时间实现计时功能
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();
            // 通知执行器定时器已经完成，可以继续`poll`对应的`Future`了
            shared_state.completed = true;
            if let Some(waker) = shared_state.waker.take() {
                waker.wake()
            }
        });

        TimerFuture { shared_state }
    }
}
