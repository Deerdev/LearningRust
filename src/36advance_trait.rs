///* 高级 trait

///* 关联类型在 trait 定义中指定占位符类型
// 关联类型（associated types）是一个将类型占位符与 trait 相关联的方式，这样 trait 的方法签名中就可以使用这些占位符类型
// 可以定义出包含某些类型的 trait，直到实现此 trait 时都无需知道这些类型具体是什么。
pub trait Iterator {
    type Item; // 关联类型

    fn next(&mut self) -> Option<Self::Item>;
}
// 关联类型看起来像一个类似泛型的概念，因为它允许定义一个函数而不指定其可以处理的类型
// ** 泛型 **
// - 每次实现 trait 时要标注类型
// - 可以为一个类型多次实现某个 trait（使用不同的泛型参数）
// ** 关联类型 **
// - 无需标注类型
// - 无法为单个类型多次实现某个 trait 「只能实现一次」

// 泛型方式：每次 impl 时，需要指定 T 的类型，一个类型可以 impl 多次（比如 sting、i32）
pub trait IteratorT<T> {
    fn next(&mut self) -> Option<T>;
}

///* 默认泛型类型参数和运算符重载
// 当使用泛型类型参数时，可以为泛型指定一个默认的具体类型
// 如果默认类型就足够的话，这消除了为具体类型实现 trait 的需要(实现 trait 时，不用写类型)
// 为泛型类型指定默认类型的语法是在声明泛型类型时使用 <PlaceholderType=ConcreteType>
// 常用语运算符重载, 运算符重载（Operator overloading）是指在特定情况下自定义运算符（比如 +）行为的操作。
// Rust 并不允许创建自定义运算符或重载任意运算符
// 不过 std::ops 中所列出的运算符和相应的 trait 可以通过实现运算符相关 trait 来重载
//
//* 默认参数类型主要用于如下两个方面：
//
// 扩展类型而不破坏现有代码。
// 在大部分用户都不需要的特定情况进行自定义。
//
//
use std::ops::Add;
/*
* <Rhs=Self> 指定默认泛型参数为 自己
trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}
*/

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;
    // 重载 + 运算符
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main1() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}

use std::ops::Add;

// 这种将现有类型简单封装进另一个结构体的方式被称为 newtype 模式（newtype pattern)
struct Millimeters(u32);
struct Meters(u32);

// Millimeters 和 Meters 相加
// * Add<Meters> 显示指定了 Rhs 为 Meters
impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

///* 完全限定语法(Fully Qualified Syntax)与消歧义：调用相同名称的方法
/// 完全限定语法：`<Type as Trait>::function(receiver_if_method, next_arg, ...);`
// - 可以在任何调用函数或方法的地方使用
// - 允许忽略那些从其它上下文能推导出来的部分
// - 当 Rust 无法区分你期望调用哪个具体实现的时候，才需要使用这种语法
//
// ---
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}
// ----
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}
// ---
fn main2() {
    let person = Human;
    // * 调用同名 trait 方法
    // 容易辨别的方式
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    // Animal::baby_name()
    //! err: type annotations needed
    // println!("A baby dog is called a {}", Animal::baby_name());

    // 完全限定语法
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name()); // A baby dog is called a puppy
}


///* 使用 supertrait 来要求 trait 附带其它 trait 的功能（类似接口继承）
// 需要在一个 trait 中使用其他 trait 的功能：
// - 需要被依赖的 trait 也被实现
// - 那个被间接依赖的 trait 就是当前 trait 的 supertrait
use std::fmt;

//* impl OutlinePrint，就必须 impl fmt::Display
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct PointD {
    x: i32,
    y: i32,
}

impl OutlinePrint for PointD {}

use std::fmt;

impl fmt::Display for PointD {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main3() {
    let p = PointD { x: 1, y: 3 };
    p.outline_print();
}


/// newtype 模式用以在外部类型上实现外部 trait
// 孤儿规则：只有当 trait 或类型定义在本地包时，才能为该类型实现这个 trait
// 可以通过 newtype 模式来绕过这一规则
// - 利用 tuple struct（元组结构体）创建一个新的类型

use std::fmt;

// 把 Vec 定义成新的类型 Wrapper，让 Wrapper 可以实现某些 trait
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main4() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}



