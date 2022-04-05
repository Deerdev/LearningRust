use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        // 链接到达
        let stream = stream.unwrap();

        // 处理消息
        handle_connection(stream);
    }
}

// --snip--

fn handle_connection(mut stream: TcpStream) {
    // --snip--

    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    // 转为 u8 的字符，为了和 buffer 直接比对
    let get = b"GET / HTTP/1.1\r\n"; // 匹配根路径

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html") // 非根路径报错
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    // flush 会等待并阻塞程序执行直到所有字节都被写入连接中；TcpStream 包含一个内部缓冲区来最小化对底层操作系统的调用。
    stream.flush().unwrap();
}
