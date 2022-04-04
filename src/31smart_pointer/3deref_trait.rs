///* 实现 Deref trait 允许我们重载 解引用运算符（dereference operator）*（与乘法运算符或通配符相区别）
/// 通过这种方式实现 Deref trait 的智能指针可以被当作常规引用来对待

///* 通过实现 Deref trait 将某类型像引用一样处理
// Deref trait，由标准库提供，要求实现名为 deref 的方法，其借用 self 并返回一个内部数据的引用
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;
    // 实现 deref trait
    fn deref(&self) -> &Self::Target {
        // 返回第一个元素
        &self.0
    }
}

// 元组结构体，就一个元组，可多个(T, U)
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // 支持解引用
                       // 实际调用为：*(y.deref())
}


///* 函数和方法的隐式 Deref 强制转换(隐式解引用转换)
// Deref 强制转换（deref coercions）是 Rust 在函数或方法传参上的一种便利。
// Deref 强制转换只能作用于实现了 Deref trait 的类型。
// Deref 强制转换将这样一个类型的引用转换为另一个类型的引用。例如，Deref 强制转换 可以将 &String 转换为 &str，因为 String 实现了 Deref trait 因此可以返回 &str。当这种特定类型的引用作为实参传递给和形参类型不同的函数或方法时，Deref 强制转换将自动发生。这时会有一系列的 deref 方法被调用，把我们提供的类型转换成了参数所需的类型。

// Deref 强制转换的加入使得 Rust 程序员编写函数和方法调用时无需增加过多显式使用 & 和 * 的引用和解引用。这个功能也使得我们可以编写更多同时作用于引用或智能指针的代码。

// * 假设 T 实现了 Deref trait：Deref Coercion 可以把 T 的引用转化为 T 经过 Deref 操作后生成的引用
// * 当把某类型的引用传递给函数或方法时，但它的类型与定义的参数类型不匹配：
// - Deref Coercion 就会自动发生
// - 编译器会对 deref 进行一系列调用，来把它转为所需的参数类型（编译时完成，没有额外性能开销）
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let m = MyBox::new(String::from("Rust"));
    // 层层 Deref，解出 &str 类型
    // 当所涉及到的类型定义了 Deref trait，Rust 会分析这些类型并使用任意多次 Deref::deref 调用以获得匹配参数的类型
    // &m &MyBox<String>
    // deref &MyBox<String> -> &String
    // deref &String -> &str
    hello(&m);
    // * 没有 Deref
    // (*m) 将 MyBox<String> 解引用为 String。接着 & 和 [..] 获取了整个 String 的字符串 slice 来匹配 hello 的签名。没有 Deref 强制转换所有这些符号混在一起将更难以读写和理解。Deref 强制转换使得 Rust 自动的帮我们处理这些转换。

    hello(&(*m)[..]);
}




///* 解引用与可变性
// 可使用 DerefMut trait 重载可变引用的 * 运算符
// 在类型和 trait 在下列三种情况发生时，Rust 会执行 deref coercion：
// - 当 T: Deref<Target=U> 时, 允许从 &T 到 &U。
// - 当 T: DerefMut<Target=U> 时, 允许从 &mut T 到 &mut U。
// - 当 T: Deref<Target=U> 时, 允许从 &mut T 到 &U。（反之不允许）

// 第三个情况有些微妙：Rust 也会将可变引用强转为不可变引用。但是反之是 不可能 的：不可变引用永远也不能强转为可变引用。
// 因为根据借用规则，如果有一个可变引用，其必须是这些数据的唯一引用（否则程序将无法编译）。将一个可变引用转换为不可变引用永远也不会打破借用规则。将不可变引用转换为可变引用则需要初始的不可变引用是数据唯一的不可变引用，而借用规则无法保证这一点。因此，Rust 无法假设将不可变引用转换为可变引用是可能的。