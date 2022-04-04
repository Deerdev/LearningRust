///* Rc<T> 引用计数智能指针 【针对不可变引用】
// 引用计数意味着记录一个值引用的数量来知晓这个值是否仍在被使用。如果某个值有零个引用，就代表没有任何有效引用并可以被清理。
// Rc<T> 用于当我们希望在堆上分配一些内存供程序的多个部分读取，而且无法在编译时确定程序的哪一部分会最后结束使用它的时候。如果确实知道哪部分是最后一个结束使用的话，就可以令其成为数据的所有者，正常的所有权规则就可以在编译时生效。
// 注意 Rc<T> 只能用于单线程场景；
/*
- 一个值会有多个所有者
- reference counting 引用计数
- 追踪到所有值的引用
- 0 个引用：该值可以被清理掉
*/


///* 使用 Rc<T> 共享数据
// Rc<T>不在预导入模块中 prelude
// Rc::clone(&a) 函数：增加引用计数
// Rc::strong_count(&a): 获得应用计数
// 还有 Rc::weak_count

enum List {
    // Cons(i32, &Rc<List>) 这么声明需要指定生命周期，而且这里Rc::new(Nil) 生命周期会立即销毁，所以无法声明成引用
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // 也可以使用 a.clone(), 但是性能底，深拷贝

    // b 和 c 共享 a 的 list
    let b = Cons(3, Rc::clone(&a));
    let c = Cons(4, Rc::clone(&a));
}

// 引用计数变化
fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
// count after creating a = 1
// count after creating b = 2
// count after creating c = 3
// count after c goes out of scope = 2

///* Rc::clone() vs 类型的 clone()方法
/// Rc::clone()：增加引用，不会执行数据的深度拷贝操作
/// 类型的 clone(): 很多会执行数据的深度拷贝操作

///* Rc<T> 通过不可变引用，使你可以在程序不同部分之间共享只读数据
// 如果 Rc<T> 也允许多个可变引用，则会违反 借用规则之一：相同位置的多个可变借用可能造成数据竞争和不一致