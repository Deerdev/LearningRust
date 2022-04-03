enum IpAddrKind {
    V4,
    V6,
}

fn main1() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}

fn route(ip_kind: IpAddrKind) {}

/// 将数据附加到枚举的变体中
fn main2() {
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
}

/// 数据类型可以是任意，struct 也可以
// Quit 没有关联任何数据。
// Move 类似结构体包含命名字段。
// Write 包含单独一个 String。
// ChangeColor 包含三个 i32。
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// 如果使用 struct 定义的话：
struct QuitMessage; // 类单元结构体
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // 元组结构体
struct ChangeColorMessage(i32, i32, i32); // 元组结构体

/// 使用 impl 为枚举定义方法
impl Message {
    fn call(&self) {
        // 在这里定义方法体
    }
}
fn main3() {
    let m = Message::Write(String::from("hello"));
    m.call();
}

///
/// Option 枚举
// Rust 没有 NUll，然而，空值尝试表达的概念仍然是有意义的：空值是一个因为某种原因目前无效或缺失的值。
// Rust 并没有空值，不过它确实拥有一个可以编码存在或不存在概念的枚举。这个枚举是 Option<T>，而且它定义于标准库中，如下:
enum Option<T> {
    None,
    Some(T),
}

// Option<T> 枚举是如此有用以至于它甚至被包含在了 prelude 之中，你不需要将其显式引入作用域。
// 另外，它的成员也是如此，可以不需要 Option:: 前缀来直接使用 Some 和 None。
// 即便如此 Option<T> 也仍是常规的枚举，Some(T) 和 None 仍是 Option<T> 的成员。
fn main4() {
    // Option<i32>
    let some_number = Some(5);
    // Option<&str>
    let some_string = Some("a string");

    // 当为 None 时，必须显式指定 T 的类型，系统无法推断
    let absent_number: Option<i32> = None;

    /// Option<T> 和 T（这里 T 可以是任何类型）是不同的类型，编译器不允许像一个肯定有效的值那样使用 Option<T>
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // 无法直接相加，类型不同
    let sum = x + y; // err: cannot add `Option<i8>` to `i8`
}
