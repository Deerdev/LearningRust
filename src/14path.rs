/// 路径
// 为了在 Rust 的模块中找到某个条目，需要使用路径
/*
路径有两种形式：
绝对路径（absolute path）从 crate 根开始，以 crate 名或者字面值 crate 开头。
相对路径（relative path）从当前模块开始，以 self、super 或当前模块的标识符开头。
*/
// 绝对路径和相对路径都后跟一个或多个由双冒号（::）分割的标识符。

/// 使用 pub 关键字暴露路径

// ---------------------------------------------------------
// 文件名: src/lib.rs
mod front_of_house {
    // 必须定义为 pub, 否则eat_at_restaurant无法调用
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// eat_at_restaurant可以访问front_of_house, 是因为他们在同级
pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}
// ---------------------------------------------------------

// 第一种方式，我们在 eat_at_restaurant 中调用 add_to_waitlist 函数，使用的是绝对路径。add_to_waitlist 函数与 eat_at_restaurant 被定义在同一 crate 中，这意味着我们可以使用 crate 关键字为起始的绝对路径。

// 第二种方式，我们在 eat_at_restaurant 中调用 add_to_waitlist，使用的是相对路径。这个路径以 front_of_house 为起始，这个模块在模块树中，与 eat_at_restaurant 定义在同一层级。与之等价的文件系统路径就是 front_of_house/hosting/add_to_waitlist。以名称为起始，意味着该路径是相对路径。

/// * 使用 super 起始的相对路径
// 可以使用 super 开头来构建从父模块开始的相对路径。这么做类似于文件系统中以 .. 开头的语法

// ---------------------------------------------------------
// 文件名: src/lib.rs
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // 调用上级, 包外的方法
        super::serve_order();
    }

    fn cook_order() {}
}
// ---------------------------------------------------------

/// * 使用 self 引用模块
// self 其实就是引用自身模块中的项，也就是说和我们之前章节的代码类似，都调用同一模块中的内容，区别在于之前章节中直接通过名称调用即可，而 self，你得多此一举：
fn serve_order() {
    self::back_of_house::cook_order()
}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        crate::serve_order();
    }

    pub fn cook_order() {}
}
// 是的，多此一举，因为完全可以直接调用 back_of_house，但是 self 还有一个大用处，在下一节中我们会讲。

/// * 创建pub公有的结构体和枚举 -- 代码可见性
// * pub 放在 struct 前：struct 是公共的，但是 字段默认还是私有的
// * struct 字段需要单独设置 pub 来变成公有
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // 在夏天订购一个黑麦土司作为早餐
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // 改变注意更换想要面包的类型
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // 如果取消下一行的注释代码不能编译；
    // 不允许查看或修改早餐附带的季节水果
    // meal.seasonal_fruit = String::from("blueberries");
}

// pub 关键字放在 enum 前
// 将枚举设为公有，则它的所有成员都将变为公有
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
