/// * 并发Web服务器
// 单线程版本
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main1() {
    // 监听本地端口 7878 ，等待 TCP 连接的建立
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // 阻塞等待请求的进入
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection1(stream);
    }
}

fn handle_connection1(mut stream: TcpStream) {
    // 从连接中顺序读取 1024 字节数据
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    // 处理HTTP协议头，若不符合则返回404和对应的`html`文件
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();

    // 将回复内容写入连接缓存中
    let response = format!("{status_line}{contents}");
    stream.write_all(response.as_bytes()).unwrap();
    // 使用flush将缓存中的内容发送到客户端
    stream.flush().unwrap();
}

/// * 选择一个异步库：使用 async-std 作为异步运行时
/*
[dependencies]
futures = "0.3"

[dependencies.async-std]
version = "1.6"
features = ["attributes"]
 */
#[async_std::main]
async fn main2() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // 警告，这里无法并发
        handle_connection2(stream).await;
    }
}

use async_std::task;

async fn handle_connection2(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        // 并没有使用 std::thread::sleep 进行睡眠，原因是该函数是阻塞的
        // 睡眠函数 async_std::task::sleep，它仅会让当前的任务陷入睡眠，然后该任务会让出线程的控制权，这样线程就可以继续运行其它任务。
        task::sleep(Duration::from_secs(5)).await;
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{status_line}{contents}");
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
// * 因此，光把函数变成 async 往往是不够的，还需要将它内部的代码也都变成异步兼容的，阻塞线程绝对是不可行的。

/// * 并发地处理连接
// 面代码最大的问题是 listener.incoming() 是阻塞的迭代器。
// 当 listener 在等待连接时，执行器是无法执行其它Future的，而且只有在我们处理完已有的连接后，才能接收新的连接
// * 解决方法是将 listener.incoming() 从一个阻塞的迭代器变成一个非阻塞的 Stream
use async_std::net::TcpListener;
use async_std::net::TcpStream;
use futures::stream::StreamExt;

#[async_std::main]
async fn main3() {
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();
    listener
        .incoming()
        .for_each_concurrent(/* limit */ None, |tcpstream| async move {
            let tcpstream = tcpstream.unwrap();
            handle_connection(tcpstream).await;
        })
        .await;
}
// 异步版本的 TcpListener 为 listener.incoming() 实现了 Stream 特征，以上修改有两个好处:
// - listener.incoming() 不再阻塞
// - 使用 for_each_concurrent 并发地处理从 Stream 获取的元素
use async_std::prelude::*;

async fn handle_connection3(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();

    //<-- snip -->
    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}

/// * 使用多线程并行处理请求
// 之前的例子有一个致命的缺陷：只能使用一个线程并发的处理用户请求
// * async 并发和多线程其实并不冲突，而 async-std 包也允许我们使用多个线程去处理，由于 handle_connection 实现了 Send 特征且不会阻塞，因此使用 async_std::task::spawn 是非常安全的:
use async_std::task::spawn;

#[async_std::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();
    listener
        .incoming()
        .for_each_concurrent(/* limit */ None, |stream| async move {
            let stream = stream.unwrap();
            spawn(handle_connection(stream));
        })
        .await;
}
// * 我们实现了同时使用并行(多线程)和并发( async )来同时处理多个请求！

//
/// * 单元测试：测试 handle_connection 函数
///
// 为了保证单元测试的隔离性和确定性，我们使用 MockTcpStream 来替代 TcpStream 。
// 首先，修改 handle_connection 的函数签名让测试更简单，之所以可以修改签名，原因在于 async_std::net::TcpStream 实际上并不是必须的，只要任何结构体实现了 async_std::io::Read, async_std::io::Write 和 marker::Unpin 就可以替代它：
use async_std::io::{Read, Write};
use std::marker::Unpin;

async fn handle_connection(mut stream: impl Read + Write + Unpin) {}
// 下面，来构建一个mock的 TcpStream 并实现了上面这些特征，它包含一些数据，这些数据将被拷贝到 read 缓存中, 然后返回 Poll::Ready 说明 read 已经结束：

use super::*;
use futures::io::Error;
use futures::task::{Context, Poll};

use std::cmp::min;
use std::pin::Pin;

struct MockTcpStream {
    read_data: Vec<u8>,
    write_data: Vec<u8>,
}

impl Read for MockTcpStream {
    fn poll_read(
        self: Pin<&mut Self>,
        _: &mut Context,
        buf: &mut [u8],
    ) -> Poll<Result<usize, Error>> {
        let size: usize = min(self.read_data.len(), buf.len());
        buf[..size].copy_from_slice(&self.read_data[..size]);
        Poll::Ready(Ok(size))
    }
}
// Write的实现也类似，需要实现三个方法 : poll_write, poll_flush, 与 poll_close。 poll_write 会拷贝输入数据到mock的 TcpStream 中，当完成后返回 Poll::Ready。由于 TcpStream 无需 flush 和 close，因此另两个方法直接返回 Poll::Ready 即可.

impl Write for MockTcpStream {
    fn poll_write(
        mut self: Pin<&mut Self>,
        _: &mut Context,
        buf: &[u8],
    ) -> Poll<Result<usize, Error>> {
        self.write_data = Vec::from(buf);

        Poll::Ready(Ok(buf.len()))
    }

    fn poll_flush(self: Pin<&mut Self>, _: &mut Context) -> Poll<Result<(), Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(self: Pin<&mut Self>, _: &mut Context) -> Poll<Result<(), Error>> {
        Poll::Ready(Ok(()))
    }
}
// 最后，我们的mock需要实现 Unpin 特征，表示它可以在内存中安全的移动，具体内容在前面章节有讲。

use std::marker::Unpin;
impl Unpin for MockTcpStream {}
// 现在可以准备开始测试了，在使用初始化数据设置好 MockTcpStream 后，我们可以使用 #[async_std::test] 来运行 handle_connection 函数，该函数跟 #[async_std::main] 的作用类似。为了确保 handle_connection 函数正确工作，需要根据初始化数据检查正确的数据被写入到 MockTcpStream 中。

use std::fs;

#[async_std::test]
async fn test_handle_connection() {
    let input_bytes = b"GET / HTTP/1.1\r\n";
    let mut contents = vec![0u8; 1024];
    contents[..input_bytes.len()].clone_from_slice(input_bytes);
    let mut stream = MockTcpStream {
        read_data: contents,
        write_data: Vec::new(),
    };

    handle_connection(&mut stream).await;
    let mut buf = [0u8; 1024];
    stream.read(&mut buf).await.unwrap();

    let expected_contents = fs::read_to_string("hello.html").unwrap();
    let expected_response = format!("HTTP/1.1 200 OK\r\n\r\n{}", expected_contents);
    assert!(stream.write_data.starts_with(expected_response.as_bytes()));
}
