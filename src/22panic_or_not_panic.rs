// 优先 Result
// 示例、代码原型和测试都非常适合 panic
// 在我们准备好决定如何处理错误之前，unwrap和expect方法在原型设计时非常方便 (占位 抛错)

fn main1() {
    use std::net::IpAddr;
    // 明确不会抛错，直接使用 unwrap，解出 OK
    let home: IpAddr = "127.0.0.1".parse().unwrap();
}

///* 错误处理指导原则
// 有害状态是非预期的行为，与偶尔会发生的行为相对，比如用户输入了错误格式的数据。
// 在此之后代码的运行依赖于不处于这种有害状态，而不是在每一步都检查是否有问题。
// 没有可行的手段来将有害状态信息编码进所使用的类型中的情况。

///* 创建自定义类型进行有效性验证
fn main2() {
    pub struct Guess {
        value: i32,
    }

    impl Guess {
        // 验证入参
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }

            Guess { value }
        }

        pub fn value(&self) -> i32 {
            self.value
        }
    }
}
