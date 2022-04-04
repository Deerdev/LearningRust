///* 使用 Sync 和 Send trait 的可扩展并发
// Rust 语言的并发特效较少，目前的并发特效都来自标准库（而不是语言本身）
// 并发方案不受标准库或语言所限：我们可以编写自己的或使用别人编写的并发功能。
// * 但是有 两个并发概念是内嵌于语言中的：std::marker 中的 Sync 和 Send trait。

///* 通过 Send 允许在线程间转移所有权
// 实现 Send trait 的类型可以在线程间转移所有权
// 几乎所有的 Rust 类型都实现了 Send
// - 但 Rc<T> 没有实现，只用于单线程
// 任何完全由 Send 的类型组成的类型也会自动被标记为 Send。（结构体的成员实现了 Send，自身就实现了 Send）
// 除了原始指针（裸指针（raw pointer））之外，几乎所有基本类型都是 Send 的，


///* Sync 允许多线程访问
// 实现 Sync 的类型可以安全的被多个线程引用
// 也就是说：如果 T 是 Sync，那么&T 就是 Sync
// - 引用可以被安全的送往另一个线程
// 基础类型都是 Sync
// 完全有 Sync 类型组件的类型也是 Sync
// - 但，Rc<T> 不是 Sync 的
// - RefCell<T> 和 Cell<T> 家族也不是 Sync 的
// - 而 Mutex<T>是 Sync 的

///* 手动实现 Send 和 Sync 是不安全的
// 通常并不需要手动实现 Send 和 Sync trait，因为由 Send 和 Sync 的类型组成的类型，自动就是 Send 和 Sync 的。因为他们是标记 trait，甚至都不需要实现任何方法。他们只是用来加强并发相关的不可变性的。

// 手动实现这些标记 trait 涉及到编写不安全的 Rust 代码