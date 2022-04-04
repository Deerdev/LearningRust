///* The Gang of Four: 面向对象的程序是由对象组成的。一个 对象 包含数据和操作这些数据的过程。这些过程通常被称为 方法 或 操作。
// 在这个定义下，Rust 是面向对象的：结构体和枚举包含数据而 impl 块提供了在结构体和枚举之上的方法。虽然带有方法的结构体和枚举并不被 称为 对象，但是他们提供了与对象相同的功能

///* 封装（encapsulation）的思想：对象的实现细节不能被使用对象的代码获取到。所以唯一与对象交互的方式是通过对象提供的公有 API；
// Rust: 使用 pub 关键字来决定模块、类型、函数和方法是公有的，而默认情况下其他一切都是私有的
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}
// 公有方法 add、remove 和 average 是修改 AveragedCollection 实例的唯一方法
impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}


///* 继承（Inheritance）是一个很多编程语言都提供的机制，一个对象可以定义为继承另一个对象的定义，这使其可以获得父对象的数据和行为，而无需重新定义。
// Rust 没有继承 (但是继承带来的优点都可以支持)
// 使用继承的原因：
// - 代码复用：Rust 默认 trait 方法来进行代码共享
// - 多态：Rust 泛型和 trait 约束（限定参数化多态 bounded parametric）