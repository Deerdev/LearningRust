///* box 允许你将一个值放在堆上而不是栈上。留在栈上的则是指向堆数据的指针
// 除了数据被储存在堆上而不是栈上之外，box 没有性能损失

// *使用场景
// - 当有一个在编译时未知大小的类型，而又想要在需要确切大小的上下文中使用这个类型值的时候
// - 当有大量数据并希望在确保数据不被拷贝的情况下转移所有权的时候
// - 当希望拥有一个值并只关心它的类型是否实现了特定 trait 而不是其具体类型的时候

/*
第一种： “box 允许创建递归类型” 部分展示第一种场景
第二种：转移大量数据的所有权可能会花费很长的时间，因为数据在栈上进行了拷贝。为了改善这种情况下的性能，可以通过 box 将这些数据储存在堆上
第三种：第三种情况被称为 trait 对象（trait object）
*/

///* 使用 Box<T> 在堆上储存数据
//将一个单独的值存放在堆上并不是很有意义，所以这样单独使用 box 并不常见
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}

///* Box 允许创建递归类型
//Rust 需要在编译时知道类型占用多少空间。一种无法在编译时知道大小的类型是 递归类型（recursive type），其值的一部分可以是相同类型的另一个值。【rust 需要明确知道空间占用多少】
// 这种值的嵌套理论上可以无限的进行下去，所以 Rust 不知道递归类型需要多少空间。
// 不过 box 有一个已知的大小，所以通过在循环类型定义中插入 box，就可以创建递归类型了。

// * cons list
// * 比较像链表，指向下一个，最后一个指向 Nil
// cons list 是一个来源于 Lisp 编程语言及其方言的数据结构。在 Lisp 中，cons 函数（“construct function" 的缩写）利用两个参数来构造一个新的列表，他们通常是一个单独的值和另一个列表
// * cons list 的每一项都包含两个元素：当前项的值和下一项。其最后一项值包含一个叫做 Nil 的值且没有下一项

// List 的一个成员被定义为是递归的：它直接存放了另一个相同类型的值。这意味着 Rust 无法计算为了存放 List 值到底需要多少空间。
enum List {
    Cons(i32, List), // err: recursive type `List` has infinite size
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}

// * Rust 计算非递归类型的大小

// Message 值所需的空间等于储存其最大成员的空间大小。
enum Message {
    Quit,                    // 并不需要任何空间
    Move { x: i32, y: i32 }, // 需要足够储存两个 i32 值的空间
    Write(String),
    ChangeColor(i32, i32, i32),
}

// * 对于 List 类型，大小检查会一直循环下去

///* 使用 Box<T> 给递归类型一个已知的大小
// 明确：Cons 成员将会需要一个 i32 的大小加上储存 box 指针数据的空间
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
// box 只提供了间接存储和堆分配；他们并没有任何其他特殊的功能，比如我们将会见到的其他智能指针。它们也没有这些特殊功能带来的性能损失，所以他们可以用于像 cons list 这样间接存储是唯一所需功能的场景。

// * Box<T> 类型是一个智能指针，因为它实现了 Deref trait，它允许 Box<T> 值被当作引用对待。当 Box<T> 值离开作用域时，由于 Box<T> 类型 Drop trait 的实现，box 所指向的堆数据也会被清除。
