/// 方法
// 方法（method）与函数类似：它们使用 fn 关键字和名称声明，可以拥有参数和返回值，同时包含在某处调用该方法时会执行的代码。
// 它们在结构体的上下文中被定义（或者是枚举或 trait 对象的上下文)
// 它们第一个参数总是 self，它代表调用该方法的结构体实例。

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

/// impl 可以拆分成多个
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    /// 关联函数（associated functions）
    // 类似 类方法、静态方法
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // 使用结构体名和 :: 语法来调用这个关联函数
    let sq = Rectangle::square(3);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}


/// self
// &self: 这里选择 &self 的理由跟在函数版本中使用 &Rectangle 是相同的：我们并不想获取所有权，只希望能够读取结构体中的数据，而不是写入。
// &mut self: 如果想要在方法中改变调用方法的实例，需要将第一个参数改为 &mut self。
// self: 通过仅仅使用 self 作为第一个参数来使方法获取实例的所有权是很少见的；这种技术通常用在当方法将 self 转换成别的实例的时候，这时我们想要防止调用者在转换之后使用原始的实例。

/// Rust 有一个叫 自动引用和解引用（automatic referencing and dereferencing）的功能
// 代码等价，不用使用 -> 操作符调用
p1.distance(&p2);
(&p1).distance(&p2);