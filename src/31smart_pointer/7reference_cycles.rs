///* 引用循环与内存泄漏
// Rust 的内存安全性保证使其难以意外地制造永远也不会被清理的内存（被称为 内存泄漏（memory leak）），但并不是不可能。

/// 制造引用循环
use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    // 定义RefCell 是为了下一个节点可以修改
    // 定义 RC，为了 List 可以被多次引用/共享访问
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        // 把 a 的结尾赋值成 b，造成循环引用：a -> a.tail -> b -> b.tail -> a
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // 循环引用，一直循环打印，直到栈溢出 stack overflow
    // println!("a next item = {:?}", a.tail());
}

// a initial rc count = 1
// a next item = Some(RefCell { value: Nil })
// a rc count after b creation = 2
// b initial rc count = 1
// b next item = Some(RefCell { value: Cons(5, RefCell { value: Nil }) })
// b rc count after changing a = 2
// a rc count after changing a = 2
// * 可以看到将列表 a 修改为指向 b 之后， a 和 b 中的 Rc<List> 实例的引用计数都是 2
// * - 在 main 的结尾，Rust 丢弃 b，这会 b Rc<List> 实例的引用计数从 2 减为 1。然而，b Rc<List> 不能被回收，因为其引用计数是 1 而不是 0。
// * - 接下来 Rust 会丢弃 a 将 a Rc<List> 实例的引用计数从 2 减为 1。这个实例也不能被回收，因为 b Rc<List> 实例依然引用它，所以其引用计数是 1

// * 防止内存泄漏的解决版本
// 依赖开发者保证
// 重新组织数据结构，使得一部分引用拥有所有权而另一部分没有
// - 循环将由一些拥有所有权的关系 和 一些无所有权的关系组成
// - 而只有所有权关系才能影响值是否可以被丢弃（清理）

///* 避免引用循环：将 Rc<T> 变为 Weak<T>
// 调用 Rc::clone 会增加 Rc<T> 实例的 strong_count，和只在其 strong_count 为 0 时才会被清理
// 通过调用 Rc::downgrade 并传递 Rc<T> 实例的引用来创建其值的 弱引用（weak reference）
// - 返回 Weak<T>
// - 调用 Rc::downgrade 会将 weak_count 加 1 (不影响strong_count)
// * c<T> 类型使用 weak_count 来记录其存在多少个 Weak<T> 引用,
// * 区别在于 weak_count 无需计数为 0 就能使 Rc<T> 实例被清理。

///* Strong vs Weak
// 强引用代表如何共享 Rc<T> 实例的所有权
// 弱引用并不属于所有权关系
// - 弱引用 不会造成引用循环，因为任何弱引用的循环会在其相关的强引用计数为 0 时被打断。
// 使用 Weak<T> 所指向的值前，必须确保其值仍然存在
// - 调用 Weak<T> 实例的 upgrade 方法，这会返回 Option<Rc<T>> 「可选值」

// * 创建树形数据结构：带有子节点的 Node
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    // RefCell 希望能修改其他节点的子节点
    // 希望 Node 拥有其子节点，同时也希望通过变量来共享所有权，以便可以直接访问树中的每一个 Node，为此 Vec<T> 的项的类型被定义为 Rc<Node>
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        // 叶子的父节点 weak 指向 branch
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    // 尝试使用 upgrade 方法获取 leaf 的父节点引用时，会得到一个 None 值
    // branch 已经不存在
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
// leaf strong = 1, weak = 0
// branch strong = 1, weak = 1
// leaf strong = 2, weak = 0
// leaf parent = None
// leaf strong = 1, weak = 0
