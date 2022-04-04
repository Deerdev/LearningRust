/// 单元测试倾向于更小而更集中，在隔离的环境中一次测试一个模块，或者是测试私有接口
// 单元测试的目的是在与其他部分隔离的环境中测试每一个单元的代码，以便于快速而准确的某个单元的代码功能是否符合预期。
// 单元测试与他们要测试的代码共同存放在位于 src 目录下相同的文件中
// 规范是在每个文件中创建包含测试函数的 tests 模块，并使用 #[cfg(test)] 标注模块。

// * cfg 属性代表 configuration ，它告诉 Rust 其之后的项只应该被包含进特定配置选项中
// 在这里，配置选项是 test，即 Rust 所提供的用于编译和运行测试的配置选项
// 通过使用 cfg 属性，Cargo 只会在我们主动使用 cargo test 运行测试时才编译测试代码。这包括测试模块中可能存在的 helper 函数，以及标注为 #[test] 的函数。

// * 测试私有函数
// 子模块的项可以使用其上级模块的项。在测试中，我们通过 use super::* 将 test 模块的父模块的所有项引入了作用域
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
