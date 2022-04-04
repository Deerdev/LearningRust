///* Refutability（可反驳性）: 模式是否会匹配失效
// 模式有两种形式：refutable（可反驳的）和 irrefutable（不可反驳的）
// - 可失败的，不可失败的

// 能匹配任何传递的可能值的模式被称为是 不可反驳的（irrefutable）
// - let x = 5

// 对某些可能的值进行匹配会失败的模式被称为是 可反驳的（refutable）
// - if let Some(x) = a_value 等于 None 无法匹配

//* 函数参数、 let 语句和 for 循环只能接受不可反驳的模式，因为通过不匹配的值程序无法进行有意义的工作。

//* if let 和 while let 表达式被限制为只能接受可反驳的模式，因为根据定义他们意在处理可能的失败：条件表达式的功能就是根据成功或失败执行不同的操作。

fn main() {
    let a: Option<i32> = Some(3);
    //! 如果 a 的值是 None，其不会成功匹配模式 Some(x)
    // refutable pattern in local binding: `None` not covered
    let Some(x) = a;

    // ok
    if let Some(x) = a {}
}
