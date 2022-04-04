///* 闭包：可以捕获环境的匿名函
// Rust 的 闭包（closures）是可以保存进变量或作为参数传递给其他函数的匿名函数。
// 可以在一个地方创建闭包，然后在不同的上下文中执行闭包运算。
// 不同于函数，闭包允许捕获调用者作用域中的值。

///* 闭包类型推断和注解
// 闭包不要求像 fn 函数那样在参数和返回值上注明类型。
// 闭包通常很短，并只关联于小范围的上下文而非任意情境。在这些有限制的上下文中，编译器能可靠的推断参数和返回值的类型
use std::thread;
use std::time::Duration;

fn generate_workout(intensity: u32, random_number: u32) {
    // 明确标注类型: 为闭包的参数和返回值增加可选的类型注解
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure(intensity));
        }
    }
}

// * 闭包语法 vs 函数语法
/*
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
*/
fn main() {
    let example_closure = |x| x;
    // 第一次调用，闭包已经被绑定为 string 类型（编译器推到）
    let s = example_closure(String::from("hello"));
    // 闭包已经被推导确定，这里报错 err: 类型不同
    let n = example_closure(5);
}

///* 使用带有泛型和 Fn trait 的闭包
// 可以创建一个存放闭包和调用闭包结果的结构体。该结构体只会在需要结果时执行闭包，并会缓存结果值，这样余下的代码就不必再负责保存结果并可以复用该值。
// 你可能见过这种模式被称 memoization 或 lazy evaluation （惰性求值）。

// * 让结构体存放闭包：
// 我们需要指定闭包的类型，因为结构体定义需要知道其每一个字段的类型。
// 每一个闭包实例有其自己独有的匿名类型：也就是说，即便两个闭包有着相同的签名，他们的类型仍然可以被认为是不同。为了定义使用闭包的结构体、枚举或函数参数，需要使用泛型和 trait bound。
// Fn 系列 trait 由标准库提供。
// 所有的闭包都实现了 trait Fn、FnMut 或 FnOnce 中的一个。
// *函数也都实现了这三个 Fn trait。如果不需要捕获环境中的值，则可以使用实现了 Fn trait 的函数而不是闭包。

// 为了满足 Fn trait bound 我们增加了代表闭包所必须的参数和返回值类型的类型。在这个例子中，闭包有一个 u32 的参数并返回一个 u32，这样所指定的 trait bound 就是 Fn(u32) -> u32。

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>, // 运行闭包之前是 None，运行闭包之后有值
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }
    // 缓存值，没有就执行一次闭包
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn main() {
    // 只定义，并未执行闭包
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    // 通过 value 访问，执行闭包
    println!("Next, do {} situps!", expensive_result.value(intensity));
}

///* Cacher 实现的限制
// 一个问题是 Cacher 实例假设对于 value 方法的任何 arg 参数值总是会返回相同的值
// * 可以使用 HashMap 代替单个值：key-arg 参数，value-执行闭包的结果
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let v1 = c.value(1);
        let v2 = c.value(2);
        // 失败
        assert_eq!(v2, 2);
    }
}
// 第二个问题是它的应用被限制为只接受获取一个 u32 值并返回一个 u32 值的闭包
// * 尝试引入更多泛型参数来增加 Cacher 功能的灵活性

//
///* 闭包会捕获其环境
// 闭包可以捕获其环境并访问其被定义的作用域的变量。（函数则不能做到同样的事）
// 当闭包从环境中捕获一个值，闭包会在闭包体中储存这个值以供使用。这会使用内存并产生额外的开销，

/*
*闭包可以通过三种方式捕获其环境，他们直接对应函数的三种获取参数的方式：获取所有权，可变借用和不可变借用。这三种捕获值的方式被编码为如下三个 Fn trait：

- FnOnce 消费从周围作用域捕获的变量，闭包周围的作用域被称为其 环境，environment。为了消费捕获到的变量，闭包必须获取其所有权并在定义闭包时将其移动进闭包。其名称的 Once 部分代表了闭包不能多次获取相同变量的所有权的事实，所以它只能被调用一次。
- FnMut 获取可变的借用值（可修改借用值），所以可以改变其环境
- Fn 从其环境获取不可变的借用值
*/

// * Rust 根据闭包如何使用环境中变量 来推断具体使用哪个 trait(层层递进，实现了 Fn 肯定实现了 Mut 和 Once，实现了 Mut 肯定实现了 Once):
// 由于所有闭包都可以被调用至少一次，所以所有闭包都实现了 FnOnce 。
// 那些并没有移动被捕获变量的所有权到闭包内的闭包也实现了 FnMut
// 而不需要对被捕获的变量进行可变访问的闭包则也实现了 Fn

// * 如果你希望强制闭包获取其使用的环境值的所有权，可以在参数列表前使用 move 关键字。这个技巧在将闭包传递给新线程以便将数据移动到新线程中时最为实用。
// 即使其捕获的值已经被移动了，move 闭包仍需要实现 Fn 或 FnMut。这是因为闭包所实现的 trait 是由闭包所捕获了什么值而不是如何捕获所决定的。而 move 关键字仅代表了后者。

fn main() {
    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;

    // 报错，x 已经移动
    println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}

/// 最佳实践：当指定 Fn trait bound 之一时，首先用 Fn，基于闭包体里的情况，如果需要 FnOnce 或 FnMut，编译器会告诉你。