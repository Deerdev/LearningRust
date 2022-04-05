///* 高级函数与闭包
//
///* 函数指针
// 可以将函数传递给其他函数
// 函数在传递过程中会被强制转换成 fn 类型
// fn 类型就是 "函数指针" function pointer
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main1() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);
}

///* 函数指针与闭包的不同
// fn 是一个类型，不是一个 trait
// - 可以直接指定 fn 为参数类型，不用声明一个以 Fn trait 为约束的泛型参数
//
// 函数指针实现了全部 3 种闭包 trait（Fn、FnMut、FnOnce）：
// - 总是可以把函数指针用作参数传递给一个接收闭包的函数
// - 所以，倾向于搭配闭包 trait 的泛型类编写函数: 可以同时接收闭包和普通函数
//
// 某些情景，指向接收 fn 函数指针而不接收闭包：
// - 与外部不支持闭包的代码交互：比如 C 函数
fn main2() {
    let list_of_numbers = vec![1, 2, 3];
    // map使用闭包
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();
}

fn main3() {
    let list_of_numbers = vec![1, 2, 3];
    // map 使用函数
    // 注意这里必须使用 “高级 trait” 讲到的完全限定语法，因为存在多个叫做 to_string 的函数；
    // 这里使用了定义于 ToString trait 的 to_string 函数，标准库为所有实现了 Display 的类型实现了这个 trait。
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();
}
/*
* map定义中，F 要实现 FnMut trait，而这里闭包和函数指针都实现了这个 trait
#[inline]
#[stable(feature = "rust1", since = "1.0.0")]
fn map<B, F>(self, f: F) -> Map<Self, F>
where
    Self: Sized,
    F: FnMut(Self::Item) -> B,
{
    Map::new(self, f)
}
 */

fn main4() {
    enum Status {
        Value(u32),
        Stop,
    }
    // 枚举使用 () 作为初始化语法，这看起来就像函数调用
    // 实际上确实被实现为返回由参数构造的实例的函数: Status::Value(u32) -> Status
    // let value = Status::Value(4);

    // * 所以可以直接使用 Status::Value 作为函数指针，传递给 map
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
}

///* 返回闭包
// 闭包使用 trait 进行表达，无法在函数中返回一个闭包，可以将一个实现了改 trait 的具体类型作为返回值

// 无法计算`dyn Fn(i32) -> i32`的大小, Rust 并不知道需要多少空间来储存闭包
// `dyn Fn(i32) -> i32` 不是 Sized，它指向要调用函数片段的地址（无法知道大小），需要用 Sized 的类型来包裹，比如 Box，用 Box 包裹并指向它 （Box 存在栈上，有固定大小）
//! return type cannot have an unboxed trait object
fn returns_closure() -> dyn Fn(i32) -> i32 {
    |x| x + 1
}

fn returns_closure_box() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

