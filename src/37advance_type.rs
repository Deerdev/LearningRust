///* 高级类型
///* 使用 newtype 模式显示类型的安全和抽象
// newtype 模式可以：
// - 用来静态的宝座各种值之间不会混淆 并 表明值的单位
// - 为类型的某些细节提供抽象能力
// - 通过轻量级的封装来隐藏内部实现细节

///* 使用类型别名创建类型同义词
// Rust 通过了类型别的功能：
// - 为现有类型生产另外的名称（同义词）
// - 并不是一个独立的类型
// - 使用 type 关键字
//
// 主要用途：减少代码字符重复

fn main1() {
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);

    // let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));
    // 别名后
    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));

    fn takes_long_type(f: Thunk) {
        // --snip--
    }

    fn returns_long_type() -> Thunk {
        // --snip--
    }
}

// Result<T> 别名
use std::fmt;

// Result<T> 已经定义在 std::result::Result 中，精简 Result<T, E> 的写法
type Result<T> = std::result::Result<T, std::io::Error>;

pub trait Write {
    // fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;
    // 优化后
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
    // 使用标准库定义：std::io::Result
    fn flush(&mut self) -> std::io::Result<()>;

    fn write_all(&mut self, buf: &[u8]) -> Result<()>;
    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
}

/// Never 类型
// 有一个名为 ! 的特殊类型：
// - 它没有任何值，行话称为空类型（empty type）
// - 我们倾向于叫它 never 类型，因为它在不返回的函数中充当返回类型
// 不返回值的函数也被称为 发散函数 (diverging function)

// 报错：空函数的返回是 `() 空元组`，是有返回的，所以不是 !
// fn bar0() -> ! {}

fn bar() -> ! {
    // --snip--
    panic!(); // 恐慌，函数不会返回
}

// * 用于"辅助"判断值的返回类型
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main2() {
    loop {
        let mut guess = String::new();

        // --snip--

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // * continue 的值是 !。
        // 也就是说，当 Rust 要计算 guess 的类型时，它查看这两个分支。前者是 u32 值，而后者是 ! 值
        // 因为 ! 并没有一个值，Rust 决定 guess 的类型是 u32。
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        // 描述 ! 的行为的正式方式是 never type 可以强转为任何其他类型。
        // 允许 match 的分支以 continue 结束是因为 continue 并不真正返回一个值；
        // 相反它把控制权交回上层循环，所以在 Err 的情况，事实上「并未对 guess 赋值」。
    }
}

impl<T> Option<T> {
    pub fn unwrap(self) -> T {
        // Rust 知道 val 是 T 类型，panic! 是 ! 类型，所以整个 match 表达式的结果是 T 类型。
        // 这能工作是因为 panic! 并不产生一个值；它会终止程序
        match self {
            Some(val) => val,
            None => panic!("called `Option::unwrap()` on a `None` value"),
        }
    }
}
fn main3() {
    print!("forever ");

    // 循环永远也不结束，所以此表达式的值是 !
    loop {
        print!("and ever ");
    }
}

///* 动态大小类型和 Sized trait
// Rust 需要在编译时确定为一个特定类型的值分配多少空间
// 动态大小的类型（Dynamically Sized Types，DST）的概念：
// - 编写代码时 使用只有在运行时才能确定大小的值
//
// str 是动态大小的类型（这里不是 &str）：只有运行时才能确定字符串的长度
// - 只能使用 &str 类解决（字符串切片）
// - - &str 存储 str 的地址和 str 的长度，这在编译期是可以确定大小的
//
fn main4() {
    // 下面代码无法正常工作，str 大小在编译时不确定
    let s1: str = "Hello there!";
    let s2: str = "How's it going?";
}

///* Rust 使用动态大小类型通用方式
// 附带一些额外的元数据来存储动态信息的大小
// - 使用动态大小类型时，总会把它的值放在某种指针后边

///* 另外一种动态大小的类型：trait
// 每个 trait 都是一个动态大小的类型，可以通过名称对其进行引用
// 为了将 trait 用作 trait 对下，必须将它放置在某种指针之后
// - 例如 &dyn Trait 或 Box<dyn Trait> (Rc<dyn Trait>) 之后

///* Sized trait
// 为了处理动态大小的类型，Rust 提供了一个 Sized trait 来确定一个类型的大小在编译时是否已知
// - 编译时可计算出大小的类型会自动实现这一 trait
// - Rust 还会为每一个泛型函数隐式的添加 Sized 约束

fn generic<T>(t: T) {
    // --snip--
}

/*
* 实际被处理如下
fn generic<T: Sized>(t: T) {
    // --snip--
}
 */

//
/// 默认情况下，泛型函数只能被用于编译时已经知道大小的类型，可以通过特殊语法解出这一限制
///* ?Sized trait 约束
// T 可能是也不能不是 Sized
// 这个语法只能用在 Sized 上面，不能被用于其他 trait

// 参数中 T -> &T, 因为 T 可能不是 Sized
fn generic2<T: ?Sized>(t: &T) {
    // --snip--
}
