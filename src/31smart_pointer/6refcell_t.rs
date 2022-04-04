///* 内部可变性（Interior mutability）是 Rust 中的一个设计模式，它允许你即使在有不可变引用时也可以改变数据，这通常是借用规则所不允许的
//* 为了改变数据，该模式在数据结构中使用 unsafe 代码来模糊 Rust 通常的可变性和借用规则

///* 通过 RefCell<T> 在运行时检查借用规则
// 不同于 Rc<T>，RefCell<T> 代表其数据的唯一的所有权。

/*
* 借用规则：

- 在任意给定时刻，只能拥有一个可变引用或任意数量的不可变引用 之一（而不是两者）。
- 引用必须总是有效的。
*/
// * Box<T> 在「编译阶段」强制代码准守借用规则，否则出现错误
// * RefCell<T> 只会在「运行时」检查借用规则，否则触发 panic

///* 借用规则 在不同阶段进行检查的比较
/// 编译阶段：这些错误将在开发过程的早期被捕获，同时对运行时没有性能影响，因为所有的分析都提前完成了。为此，在编译时检查借用规则是大部分情况的最佳选择，这也正是其为何是 Rust 的默认行为。

// 运行阶段：在运行时检查借用规则的好处则是允许出现特定内存安全的场景，而它们在编译时检查中是不允许的。静态分析，正如 Rust 编译器，是天生保守的。【实现某些特定的内存安全场景-不可变环境中修改自身数据】
// - 问题暴露延后，甚至到生成环境；因借用技术产生些许性能损失

///* 使用 RefCell<T> 是基于开发者来确认/保证借用规则没有问题（而编译器不能理解和确定的时候）
///* RefCell<T> 只能用于单线程场景

//
///* 如下为选择 Box<T>，Rc<T> 或 RefCell<T> 的理由：
// - Rc<T> 允许相同数据有多个所有者；Box<T> 和 RefCell<T> 有单一所有者。
// - Box<T> 允许在编译时执行 「不可变或可变」 借用检查；Rc<T>仅允许在编译时执行「不可变」借用检查；RefCell<T> 允许在运行时执行「不可变或可变」借用检查。
// - 因为 RefCell<T> 允许在运行时执行可变借用检查，所以我们可以在即便 RefCell<T> 自身是不可变的情况下修改其内部的值。

// * 内部可变性：可变的借用一个不可变的值

// 举例：内部可变性的用例：mock 对象
// 测试替身（test double）是一个通用编程概念，它代表一个在测试中替代某个类型的类型。mock 对象 是特定类型的测试替身，它们记录测试过程中发生了什么以便可以断言操作是正确的。

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

// mock 方法测试
#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        // 声明为 RefCell 可变类型
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                // 创建 RefCell 可变类型
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        // 因为 trait 声明了  fn send(&self, msg: &str); 中 self 是不可变的，所以内部不可以修改 sent_messages
        // 只能通过 RefCell 不可变引用来绕过
        fn send(&self, message: &str) {
            // borrow_mut 可变借用，获取 sent_messages 的值，并修改
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        // --snip--
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        // borrow 不可变借用，获取 sent_messages 的值，读取
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

///* RefCell<T> 在运行时记录借用
// 当创建不可变和可变引用时，我们分别使用 & 和 &mut 语法。对于 RefCell<T> 来说，则是 borrow 和 borrow_mut 方法
// * borrow 方法返回 Ref<T> 类型的智能指针
// * borrow_mut 方法返回 RefMut 类型的智能指针。
// 这两个类型都实现了 Deref，所以可以当作常规引用对待。

//
// * RefCell<T> 记录当前有多少个活动的 Ref<T> 和 RefMut<T> 智能指针
// - 每次调用 borrow，RefCell<T> 将活动的不可变借用计数加一
// - 当 Ref<T> 值离开作用域时，不可变借用计数减一
// - 每次调用 borrow_mut: 可变借用计数加 1
// - 任何一个 RefMut<T> 的值离开作用域被释放时：可变借用计数减 1

// 如果我们尝试违反这些规则，相比引用时的编译时错误，RefCell<T> 的实现会在「运行时」出现 panic

// * 以此技术来维护借用检查规则（违反，也会运行时报错）
// - 任何一个给定时间里，只允许拥有多个不可变借用 或 一个可变借用
impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
        // 编译报错：相同作用域中创建两个可变引用，这是不允许的
        let mut one_borrow = self.sent_messages.borrow_mut();
        let mut two_borrow = self.sent_messages.borrow_mut();

        one_borrow.push(String::from(message));
        two_borrow.push(String::from(message));
    }
}

///* 结合 Rc<T> 和 RefCell<T> 来实现一个拥有多重所有权的可变数据
/// 如果有一个储存了 RefCell<T> 的 Rc<T> 的话，就可以得到有多个所有者 并且 可以修改的值了！
#[derive(Debug)]
enum List {
    // 不可变<可变<i32>>, 不可变<List>
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    // 不可变(可变(5))
    let value = Rc::new(RefCell::new(5));
    // 不可变(Cons(引用不可变 value，不可变 Nil))
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil))); // 让 a 拥有 value，但是不转义所有权

    // Cons(不可变(可变(3))，引用不可变 a)
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    // Cons(不可变(可变(4))，引用不可变 a)
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    // deref value -> 不可变(可变(5)) -> deref -> 可变(5)
    // 可变(5).borrow_mut -> RefMut<5> 智能指针 -> deref 解引用运算符并修改其内部值(+10)
    *value.borrow_mut() += 10; // 修改 Rc 内部的值

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

//* 标准库中也有其他提供内部可变性的类型，比如 Cell<T>，它类似 RefCell<T> 但有一点除外：它并非提供内部值的引用，而是把值拷贝进和拷贝出 Cell<T>。还有 Mutex<T>，其提供线程间安全的内部可变性
