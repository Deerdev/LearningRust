/// * 执行器和系统IO
/*
pub struct SocketRead<'a> {
    socket: &'a Socket,
}

impl SimpleFuture for SocketRead<'_> {
    type Output = Vec<u8>;

    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        if self.socket.has_data_to_read() {
            // socket有数据，写入buffer中并返回
            Poll::Ready(self.socket.read_buf())
        } else {
            // socket中还没数据
            //
            // 注册一个`wake`函数，当数据可用时，该函数会被调用，
            // 然后当前Future的执行器会再次调用`poll`方法，此时就可以读取到数据
            self.socket.set_readable_callback(wake);
            Poll::Pending
        }
    }
}
*/
// * 该例子中还有一个疑问没有解决：
// `set_readable_callback` 方法到底是怎么工作的？怎么才能知道 socket 中的数据已经可以被读取了？
// - 如果自己实现：其中一个简单粗暴的方法就是使用一个新线程不停的检查 socket 中是否有了数据，当有了后，就调用 wake() 函数。该方法确实可以满足需求，但是性能着实太低了，需要为每个阻塞的 Future 都创建一个单独的线程！
//
// 在现实世界中，该问题往往是通过操作系统提供的 IO 多路复用机制来完成，例如 Linux 中的 epoll，FreeBSD 和 macOS 中的 kqueue ，Windows 中的 IOCP, Fuchisa中的 ports 等(可以通过 Rust 的跨平台包 mio 来使用它们)。
// 借助IO多路复用机制，可以实现一个线程同时阻塞地去等待多个异步IO事件，一旦某个事件完成就立即退出阻塞并返回数据。相关实现类似于以下代码：
struct IoBlocker {/* ... */}

struct Event {
    // Event的唯一ID，该事件发生后，就会被监听起来
    id: usize,

    // 一组需要等待或者已发生的信号
    signals: Signals,
}

impl IoBlocker {
    /// 创建需要阻塞等待的异步IO事件的集合
    fn new() -> Self { /* ... */
    }

    /// 对指定的IO事件表示兴趣
    fn add_io_event_interest(
        &self,

        /// 事件所绑定的socket
        io_object: &IoObject,

        event: Event,
    ) { /* ... */
    }

    /// 进入阻塞，直到某个事件出现
    fn block(&self) -> Event { /* ... */
    }
}
fn main2() {
    let mut io_blocker = IoBlocker::new();
    io_blocker.add_io_event_interest(
        &socket_1,
        Event {
            id: 1,
            signals: READABLE,
        },
    );
    io_blocker.add_io_event_interest(
        &socket_2,
        Event {
            id: 2,
            signals: READABLE | WRITABLE,
        },
    );
    let event = io_blocker.block();

    // 当socket的数据可以读取时，打印 "Socket 1 is now READABLE"
    println!("Socket {:?} is now {:?}", event.id, event.signals);
}

impl Socket {
    fn set_readable_callback(&self, waker: Waker) {
        // `local_executor` is a reference to the local executor.
        // this could be provided at creation of the socket, but in practice
        // many executor implementations pass it down through thread local
        // storage for convenience.
        let local_executor = self.local_executor;

        // Unique ID for this IO object.
        let id = self.id;

        // 通过 id 关联 waker
        // 通过 id 注册读取完成信号(readable)
        // Store the local waker in the executor's map so that it can be called
        // once the IO event arrives.
        local_executor.event_map.insert(id, waker);
        local_executor.add_io_event_interest(
            &self.socket_file_descriptor,
            Event {
                id,
                signals: READABLE,
            },
        );
    }
}
