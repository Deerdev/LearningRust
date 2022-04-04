/// 测试函数体通常执行如下三种操作(3A 操作)：
// 设置任何所需的数据或状态
// 运行需要测试的代码
// 断言其结果是我们所期望的

///* 测试函数剖析
// 将一个函数变成测试函数，需要在 fn 行之前加上 #[test] (test 属性attribute注解)
// cargo test 命令会运行项目中所有的测试

// `cargo new adder --lib`
// 使用 Cargo 新建一个库项目时，它会自动为我们生成一个测试模块和一个测试函数
// 也可以额外增加任意多的测试函数以及测试模块
// src/lib.rs
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    // 直接 panic! 标识测试失败
    #[test]
    fn another() {
        panic!("Make this test fail");
    }
}

///* 使用 assert! 宏来检查结果

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// lib
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        // true 成功
        assert!(larger.can_hold(&smaller));
    }
}

///* assert_eq! 相等
///* assert_ne! 不等
// 当断言失败时他们也会打印出这两个值具体是什么，以便于观察测试 为什么 失败，而 assert! 只会打印出它从 == 表达式中得到了 false 值，而不是导致 false 的两个值
// assert_eq! 和 assert_ne! 宏在底层分别使用了 == 和 !=。
// 当断言失败时，这些宏会使用调试格式打印出其参数，这意味着被比较的值必需实现了 PartialEq 和 Debug trait。
// 所有的基本类型和大部分标准库类型都实现了这些 trait。对于自定义的结构体和枚举，需要实现 PartialEq 才能断言他们的值是否相等。需要实现 Debug 才能在断言失败时打印他们的值。
// 因为这两个 trait 都是派生 trait，通常可以直接在结构体或枚举上添加 #[derive(PartialEq, Debug)] 注解。
pub fn add_two(a: i32) -> i32 {
    a + 2
}

// lib
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
}

///* 自定义失败信息
// 可以向 assert!、assert_eq! 和 assert_ne! 宏传递一个可选的失败信息参数
// 任何在 assert! 的一个必需参数和 assert_eq! 和 assert_ne! 的两个必需参数之后指定的参数(额外的自定义参数)都会传递给 format! 宏，所以可以传递一个包含 {} 占位符的格式字符串和需要放入占位符的值
pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }
}

///* 使用 should_panic 检查 panic
// 检查代码是否按照期望处理错误(抛出 panic)
// 这个属性在函数中的代码发生 panic 时会通过，而在其中的代码没有 panic 时失败。
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}
// * 让 should_panic 更精确
// 给 should_panic 属性增加一个可选的 expected 参数。
// 测试工具会确保错误信息中包含其提供的文本
pub struct Guess {
    value: i32,
}

// --snip--
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // 大于 100 时，才会通过，匹配报错文案
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}

///* 将 Result<T, E> 用于测试
// 测试通过时返回 Ok(())，在测试失败时返回带有 String 的 Err。
// 不能对这些使用 Result<T, E> 的测试使用 #[should_panic] 注解。
// 为了断言一个操作返回 Err 成员，不要使用对 Result<T, E> 值使用问号表达式（?）。而是使用 assert!(value.is_err())。
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
