// 所有程序都必须管理其运行时使用计算机内存的方式。
// 一些语言中具有垃圾回收机制，在程序运行时不断地寻找不再使用的内存；在另一些语言中，程序员必须亲自分配和释放内存。
// Rust 则选择了第三种方式：通过所有权系统管理内存，编译器在编译时会根据一系列的规则进行检查。
// 如果违反了任何这些规则，程序都不能编译。在运行时，所有权系统的任何功能都不会减慢程序。

// 栈 堆
// 栈以放入值的顺序存储值并以相反顺序取出值。这也被称作 后进先出（last in, first out）栈中的所有数据都必须占用已知且固定的大小。
// 在编译时大小未知或大小可能变化的数据，要改为存储在堆上。 堆是缺乏组织的：当向堆放入数据时，你要请求一定大小的空间。内存分配器（memory allocator）在堆的某处找到一块足够大的空位，把它标记为已使用，并返回一个表示该位置地址的 指针（pointer）。这个过程称作 在堆上分配内存（allocating on the heap），有时简称为 “分配”（allocating）。将数据推入栈中并不被认为是分配。因为指针的大小是已知并且固定的，你可以将指针存储在栈上，不过当需要实际数据时，必须访问指针。
// 入栈比在堆上分配内存要快，因为（入栈时）分配器无需为存储新数据去搜索内存空间；其位置总是在栈顶。相比之下，在堆上分配内存则需要更多的工作，这是因为分配器必须首先找到一块足够存放数据的内存空间，并接着做一些记录为下一次分配做准备。
// 访问堆上的数据比访问栈上的数据慢，因为必须通过指针来访问。现代处理器在内存中跳转越少就越快（缓存）;处理器在处理的数据彼此较近的时候（比如在栈上）比较远的时候（比如可能在堆上）能更好的工作。在堆上分配大量的空间也可能消耗时间。


// 所有权规则
// 1 Rust 中的每一个值都有一个被称为其 所有者（owner）的变量。
// 2 值在任一时刻有且只有一个所有者。
// 3 当所有者（变量）离开作用域，这个值将被丢弃。

// heap 申请内存
// 对于 String 类型，为了支持一个可变，可增长的文本片段，需要在堆上分配一块在编译时未知大小的内存来存放内容。这意味着：
// 必须在运行时向内存分配器（memory allocator）请求内存。
// 需要一个当我们处理完 String 时将内存返回给分配器的方法。

// Rust 采取了一个不同的策略(相较于 垃圾回收和手动管理)：内存在拥有它的变量离开作用域后就被自动释放
{
    let s = String::from("hello"); // 从此处起，s 是有效的

    // 使用 s
}  // 此作用域已结束， s 不再有效
// drop函数：当 s 离开作用域的时候。当变量离开作用域，Rust 为我们调用一个特殊的函数。这个函数叫做 drop，在这里 String 的作者可以放置释放内存的代码。Rust 在结尾的 } 处自动调用 drop。


// 变量数据交互：Move 移动
// 堆变量在赋值之后（s1和s2 指向相同的内存），产生 Move，s1 无法访问
// 如果 s1 可以继续使用：当 s2 和 s1 离开作用域，他们都会尝试释放相同的内存。这是一个叫做 二次释放（double free）的错误，也是之前提到过的内存安全性 bug 之一。两次释放（相同）内存会导致内存污染，它可能会导致潜在的安全漏洞。
// 所以赋值之后，直接让 s1 失效，释放的时候只释放 s2
let s1 = String::from("hello");
let s2 = s1;

// String 的结构（变量，存在 stack 上）：
// ptr 指针，指向 heap
// len 当前字符的长度
// capacity 申请 heap 的总长度


// 变量数据交互：clone 克隆
// 使用 clone 会克隆堆数据，s1和 s2 指向不同的 heap，所以 s1 和 s2 都有效
// 当出现 clone 调用时，你知道一些特定的代码被执行而且这些代码可能相当消耗资源
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);


// 只在栈上的数据：拷贝
// Copy trait(类似接口)
// Rust 有一个叫做 Copy trait 的特殊注解，可以用在类似整型这样的存储在栈上的类型上
// 如果一个类型实现了 Copy trait，那么一个旧的变量在将其赋值给其他变量后仍然可用。Rust 不允许自身或其任何部分实现了 Drop trait 的类型使用 Copy trait。如果我们对其值离开作用域时需要特殊处理的类型使用 Copy 注解，将会出现一个编译时错误。要学习如何为你的类型添加 Copy 注解以实现该 trait，请阅读附录 C 中的 “可派生的 trait”。

// 任何一组简单标量值的组合都可以实现 Copy，任何不需要分配内存或某种形式资源的类型都可以实现 Copy 。如下是一些 Copy 的类型：

// 所有整数类型，比如 u32。
// 布尔类型，bool，它的值是 true 和 false。
// 所有浮点数类型，比如 f64。
// 字符类型，char。
// 元组，当且仅当其包含的类型也都实现 Copy 的时候。比如，(i32, i32) 实现了 Copy，但 (i32, String) 就没有。


// 所有权与函数
// 变量 s 传入函数，所有权转义，调用处就失效了
fn main1() {
    let s = String::from("hello");  // s 进入作用域

    takes_ownership(s);             // s 的值移动到函数里 ...
                                    // ... 所以到这里不再有效

    let x = 5;                      // x 进入作用域

    makes_copy(x);                  // x 应该移动函数里，
                                    // 但 i32 是 Copy 的，
                                    // 所以在后面可继续使用 x

} // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
  // 没有特殊之处

fn takes_ownership(some_string: String) { // some_string 进入作用域（s 赋值给 some_thing）
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。
  // 占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。没有特殊之处（非 heap 变量）


// 返回值与作用域
// 函数返回值将变量所有权转移给调用方
fn main2() {
    let s1 = gives_ownership();         // gives_ownership 将返回值转移给 s1

    let s2 = String::from("hello");     // s2 进入作用域

    let s3 = takes_and_gives_back(s2);  // s2 被移动到 takes_and_gives_back 中, 它也将返回值移给 s3
} // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
  // 所以什么也不会发生。s1 离开作用域并被丢弃

fn gives_ownership() -> String {             // gives_ownership 会将 返回值移动给 调用它的函数
    let some_string = String::from("yours"); // some_string 进入作用域.
    some_string                              // 返回 some_string 并移出给调用的函数 
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域
    a_string  // 返回 a_string 并移出给调用的函数
}

// 保留调用参数的所有权：将其再通过函数返回值返回
// 或 使用引用
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 返回字符串的长度

    (s, length)
}