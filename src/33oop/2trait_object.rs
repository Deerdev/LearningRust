///* 遍历 GUI 元素，调用它们的 draw 方法

// * 使用泛型
pub trait Draw {
    fn draw(&self);
}

// 泛型 限制了 Screen 实例必须拥有一个全是 Button 类型或者全是 TextField 类型的组件列表。
// 如果只需要同质（相同类型）集合，则倾向于使用泛型和 trait bound，因为其定义会在编译时采用具体类型进行单态化。
pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// * 使用 trait
// 通过使用 trait 对象的方法，一个 Screen 实例可以存放一个既能包含 Box<Button>，也能包含 Box<TextField> 的 Vec<T>。让我们看看它是如何工作的，接着会讲到其运行时性能影响。
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // dyn Draw 实现 Draw 的所有实例
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}

use gui::Draw;

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}

use gui::{Button, Screen};

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}

///* trait 对象执行动态分发
// 当对泛型使用 trait bound 时编译器所进行单态化处理：编译器为每一个被泛型类型参数代替的具体类型生成了非泛型的函数和方法实现。
// 单态化所产生的代码进行 静态分发（static dispatch）。
// 静态分发 发生于编译器在编译时就知晓调用了什么方法的时候。
// 这与 动态分发 （dynamic dispatch）相对，这时编译器在编译时无法知晓调用了什么方法。
// 在动态分发的情况下，编译器会产生额外的代码以便在运行时找出希望调用的方法

// 当使用 trait 对象时，Rust 必须使用动态分发：
// - 产生运行时开销：编译器无法知晓所有可能用于 trait 对象代码的类型，所以它也不知道应该调用哪个类型的哪个方法实现。为此，Rust 在运行时使用 trait 对象中的指针来知晓需要调用哪个方法。
// - 阻止内联方法： 动态分发也阻止编译器有选择的内联方法代码，这会相应的禁用一些优化。

// 尽管在编写代码的过程中确实获得了额外的灵活性，但仍然需要权衡取舍。




///* trait对象需要类型安全
// 只有对象安全（object-safe）的trait可以转化为 trait 对象。
// Rust有一些复杂的规则来实现 trait 的对象安全，但在实践中，只有两个相关的规则。
// 如果一个 trait 中定义的所有方法都符合以下规则，则该 trait 是对象安全的：
// - 方法的返回类型不是 Self
// - 党发中没有泛型类型的参数

pub trait Clone {
    fn clone(&self) -> Self;
}
pub struct Screen {
    //! Clone 不是对象安全的，因为它的方法返回了 Self
    // the trait `Clone` cannot be made into an object
    pub components: Vec<Box<dyn Clone>>,
}


