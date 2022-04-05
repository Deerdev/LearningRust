///* Rust 还隐藏有第二种语言，它不会强制执行这类内存安全保证：这被称为 不安全 Rust（unsafe Rust）
// 不安全 Rust 之所以存在，是因为静态分析本质上是保守的
// "代码 可能 是合法的，但如果 Rust 编译器没有足够的信息来确定，它将拒绝该代码"
// 在这种情况下，可以使用不安全代码告诉编译器，“相信我，我知道我在干什么。”
// 自己保证：这么做的缺点就是你只能靠自己了：如果不安全代码出错了，比如解引用空指针，可能会导致不安全的内存使用。

///* 不安全的超能力
// 通过 unsafe 关键字来切换到不安全 Rust, 开启一个 unsafe 代码块
// unsafe rust 可以执行四个动作：
// - 解引用裸指针
// - 调用不安全的函数或方法
// - 访问或修改可变静态变量
// - 实现不安全 trait
// - 访问 union 的字段
//
// * unsafe 并没有关闭借用检查或停用其他安全检查
// * 任何内存安全相关的错误必须留在 unsafe 块内
// * 尽可能隔离 unsafe 代码，最好将其封装在安全的抽象里，提供安全的 API (很多标准库都在内部使用 unsafe 代码，但是对外提供的标准 API)

///* 解引用原始指针
// 新类型：原始指针/裸指针（raw pointers）
// - 可变的: *mut T
// - 不可变的: *const T  -- 解引用后不能直接对齐进行赋值
// 这里的*不是解引用符号，它是类型名的一部分（类似 指针的定义）

//
//* 与引用不同，原始指针：
// - 允许通过同时具有不可变和可变指针 或 多个指向同一个位置的可变指针 （忽略借用规则，指针本身的使用）
// - 无法保证能指向合理的内存(可能是 野指针)
// - 允许为 null
// - 不实现任何自动清理
//
// * 放与弃保证的安全，换取更好的性能、其他语言和硬件接口的能力
fn main1() {
    let mut num = 5;
    // 可以在安全代码中 创建不安全指针
    // 但是不能使用-解引用（使用只能在 unsafe 中）
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    // 在 unsafe 中解引用
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // * 创建一个指针不会造成任何危险；只有当访问其指向的值时才有可能遇到无效的值。
    let address = 0x012345usize; // 不确定的地址，解引用会报错
    let r = address as *const i32;

    // * 因为 Rust 的所有权规则不允许在拥有任何不可变引用的同时再创建一个可变引用。
    // 通过裸指针，就能够同时创建同一地址的可变指针和不可变指针，若通过可变指针修改数据，则可能潜在造成数据竞争。
}
// * 既然存在这么多的危险，为何还要使用裸指针呢？
// - 一个主要的应用场景便是调用 C 代码接口。
// - 另一个场景是构建借用检查器无法理解的安全抽象。

///* 调用unsafe函数或方法
// unsafe 函数或方法：在定义前面加上 unsafe 关键字
// - 调用前需手动满足一些条件，入微 rust 无法对这些条件进行验证
// - 只能在 unsafe 块中调用 unsafe 函数、方法
fn main2() {
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }
}

///* 创建unsafe代码的安全抽象
// 函数包含 unsafe 代码并不意味着需要将整个函数标记为 unsafe
// 将 unsafe 代码包裹在一个安全函数中是一个常见的 抽象
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();

    assert!(mid <= len);
    //! 报错创建了两个可变引用, 但是实际上这两个切片不会交叉（互不影响）
    //! cannot borrow `*slice` as mutable more than once at a time
    // (&mut slice[..mid], &mut slice[mid..])

    //* 使用 unsafe 块
    unsafe {
        (
            // slice::from_raw_parts_mut 函数是不安全的因为它获取一个裸指针，并必须确信这个指针是有效的。
            slice::from_raw_parts_mut(ptr, mid),
            // 指针的操作，偏移量
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn main3() {
    let mut vector = vec![1, 2, 3, 4, 5, 6];
    // 获取切片的前半部分和后半部分, split_at_mut 已经做了安全抽象，所以不需要再使用 unsafe 了
    let (left, right) = split_at_mut(&mut vector, 3);
}

///* 使用 extern 函数调用外部代码
// extern 关键字：简化创建和使用外部函数接口（FFI）的过程
// 外部函数接口（Foreign Function Interface， FFI）它允许一种编程语言定义函数，并让其它编程语言能能调用这些函数
extern "C" {
    // extern 函数都是不安全的（不确定对应语言有没有实现）
    // 在调用时需要使用 unsafe
    fn abs(input: i32) -> i32;
}

fn main4() {
    unsafe {
        // abs 不安全
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

// * 在 extern "C" 块中，列出了希望能够调用的另一个语言中的外部函数的签名和名称。
// "C" 部分定义了外部函数所使用的 应用二进制接口（application binary interface，ABI） —— ABI 定义了如何在汇编语言层面调用此函数。
// "C" ABI 是最常见的，并遵循 C 编程语言的 ABI。

///* 从其它语言调用 Rust 函数
// - 可以使用 extern 来创建一个允许其他语言调用 Rust 函数的接口
// - 在 fn 前添加 extern 关键字，并指定 ABI
// - 还需增加 #[no_mangle] 注解，告诉 Rust 编译器不要 mangle 此函数的名称（改变函数名称）
// --- Mangling 发生于当编译器将我们指定的函数名修改为不同的名称时，这会增加用于其他编译过程的额外信息，不过会使其名称更难以阅读。每一个编程语言的编译器都会以稍微不同的方式 mangle 函数名，所以为了使 Rust 函数能在其他语言中指定，必须禁用 Rust 编译器的 name mangling。
//
// 一旦其编译为动态库并从 C 语言中链接，call_from_c 函数就能够在 C 代码中访问
fn main5() {
    #[no_mangle]
    pub extern "C" fn call_from_c() {   // extern 的使用无需 unsafe。
        println!("Just called a Rust function from C!");
    }
}


///* 访问或修改可变静态变量
// Rust 支持全局变量（global variables）， 但是因为所有权规则可能产生某些问题。比如有两个线程访问相同的可变全局变量，则可能会造成数据竞争。
// 在 Rust 里，全局变量叫做静态(static)变量
static HELLO_WORLD: &str = "Hello, world!";

fn main6() {
    println!("name is: {}", HELLO_WORLD);
}

// * 静态变量
// - 静态变量和常量类似
// - 命名采用 SCREAMING_SNAKE_CASE 写法
// - 必须标注类型
// - 静态变量只能存储 'static 生命周期的引用，无需显式标注
// - 访问不可变静态变量是安全的

// * 常量和不可变静态变量的区别
// - 静态变量：有固定的内存地址，使用它的值总会访问同样的数据
// - 常量：允许使用他们的时候对数据进行复制
// - 静态变量：可以是可变的，访问和修改静态可变变量是不安全unsafe的（多线程访问是不安全的）
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        // 修改可变静态变量是不安全的
        COUNTER += inc;
    }
}

fn main() {
    add_to_count(3);

    unsafe {
        // 访问可变静态变量是不安全的
        println!("COUNTER: {}", COUNTER);
    }
}

///* 实现unsafe trait
// unsafe trait: 当 trait 中至少有一个方法中包含编译器无法验证的不变式（invariant）时 trait 是不安全的
// 在 trait 之前增加 unsafe 关键字将 trait 声明为 unsafe
// 同时 trait 的实现也必须标记为 unsafe (只能在 unsafe 代码块中实现)
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}



///* 访问联合体中的字段
// union 和 struct 类似，但是在一个实例中同时只能使用一个声明的字段。
// 联合体主要用于和 C 代码中的联合体交互。
// 访问联合体的字段是不安全的，因为 Rust 无法保证当前存储在联合体实例中数据的类型

///* 何时使用不安全代码
// 使得 unsafe 代码正确也实属不易，因为编译器不能帮助保证内存安全
// 当有理由使用 unsafe 代码时，是可以这么做的(该用就用)
// 通过使用显式的 unsafe 标注可以更容易地在错误发生时追踪问题的源头