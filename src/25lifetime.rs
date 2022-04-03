///* 生命周期
// Rust 中的每一个引用都有其 生命周期（lifetime），也就是引用保持有效的作用域，大部分时候生命周期是隐含并可以推断的
// 类似于当因为有多种可能类型的时候必须注明类型，也会出现引用的生命周期以一些不同方式相关联的情况，所以 Rust 需要我们使用泛型生命周期参数来注明他们的关系，这样就能确保运行时实际使用的引用绝对是有效的。
// * 显式标注生命周期，针对参数和返回值

///* 生命周期避免了悬垂引用
// 生命周期的主要目标是避免悬垂引用，它会导致程序引用了非预期引用的数据
fn main() {
    {
        let r;
        {
            let x = 5;
            r = &x; // x 会脱离生命周期，报错:`x` does not live long enough
        }
        println!("r: {}", r);
    }
}

///* 借用检查器 （borrow checker）: 它比较作用域来确保所有的借用都是有效的
// 通过比较 x 和 r 生命周期的长度，判断生命周期是否匹配
/*
fn main() {
    {
        let r;                // ---------+-- 'a
                              //          |
        {                     //          |
            let x = 5;        // -+-- 'b  |
            r = &x;           //  |       |
        }                     // -+       |
                              //          |
        println!("r: {}", r); //          |
    }                         // ---------+
}
*/
// 这里将 r 的生命周期标记为 'a 并将 x 的生命周期标记为 'b。如你所见，内部的 'b 块要比外部的生命周期 'a 小得多。在编译时，Rust 比较这两个生命周期的大小，并发现 r 拥有生命周期 'a，不过它引用了一个拥有生命周期 'b 的对象。程序被拒绝编译，因为生命周期 'b 比生命周期 'a 要小：被引用的对象比它的引用者存在的时间更短。

///* 函数中的泛型生命周期
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

// *返回值需要一个泛型生命周期参数，因为 Rust 并不知道将要返回的引用是指向 x 或 y
// 如果 x 比 y 短，返回值是 x，那么返回值可能很快就销毁了（因为 x 没了）；但是对调用方来说，“看不出来”，认为返回值可以继续用
// err: [-> &str] expected named lifetime parameter
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

///* 生命周期注解语法
// 生命周期注解并不改变任何引用的生命周期的长短，只是为了函数签名，被编译器识别
// 生命周期注解描述了多个引用生命周期相互的关系，而不影响其生命周期。
// 单个的生命周期注解本身没有多少意义，因为生命周期注解告诉 Rust 多个引用的泛型生命周期参数如何相互联系的
/*
&i32        // 引用
&'a i32     // 带有显式生命周期的引用
&'a mut i32 // 带有显式生命周期的可变引用
*/
// * 例如如果函数有一个生命周期 'a 的 i32 的引用的参数 first。还有另一个同样是生命周期 'a 的 i32 的引用的参数 second。这两个生命周期注解意味着引用 first 和 second 必须与这泛型生命周期存在得一样久。

///* 函数签名中的生命周期注解
// 在这个签名中 x, y 和返回值：生命周期都是 'a, 一致(活的一样久)
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
// 「只是为了让编译器可以检查出调用时，生命周期是否满足」
// 我们并没有改变任何传入值或返回值的生命周期，而是指出任何不满足这个约束条件的值都将被借用检查器拒绝

// *当具体的引用被传递给 longest 时，被 'a 所替代的具体生命周期是 x 的作用域与 y 的作用域相重叠的那一部分 (交叉部分，等同于 x 和 y 的生命周期中较小的那一个)

// 以下编译报错，生命周期不匹配
// err: `string2` does not live long enough
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}

///* 深入理解生命周期
// *指定生命周期参数的正确方式依赖函数实现的具体功能
// 如果将 longest 函数的实现修改为总是返回第一个参数而不是最长的字符串 slice，就不需要为参数 y 指定一个生命周期
fn main() {
    let string1 = String::from("abcd");
    let string2 = "efghijklmnopqrstuvwxyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
// y 不需要指定生命周期
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

// 当从函数返回一个引用，返回值的生命周期参数需要与一个参数的生命周期参数相匹配。
// *如果返回的引用 没有 指向任何一个参数，那么唯一的可能就是它指向一个函数内部创建的值，它将会是一个悬垂引用，因为它将会在函数结束时离开作用域
fn longest<'a>(x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    // result 在 longest 函数的结尾将离开作用域并被清理，而我们尝试从函数返回一个 result 的引用
    result.as_str() // 报错 cannot return reference to local variable `result`
}

// 无法指定生命周期参数来改变悬垂引用，而且 Rust 也不允许我们创建一个悬垂引用
// 最好的解决方案是返回一个有所有权的数据类型而不是一个引用，这样函数调用者就需要负责清理这个值了。[转移所有权]
fn longest<'a>(x: &str, y: &str) -> String {
    let result = String::from("really long string");
    result // ok
}

///* 结构体定义中的生命周期注解
struct ImportantExcerpt<'a> {
    // 成员的生命周期必须和 结构体一致（或大于 结构体生命周期）
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

// ---
///* 生命周期省略（Lifetime Elision）
// 每一个引用都有一个生命周期，而且我们需要为那些使用了引用的函数或结构体指定生命周期
// 被编码进 Rust 引用分析的模式被称为 生命周期省略规则（lifetime elision rules）
// 代码符合规则，编译器会自动推断，不需要显式标注

// 省略规则并不提供完整的推断：如果 Rust 在明确遵守这些规则的前提下变量的生命周期仍然是模棱两可的话，它不会猜测剩余引用的生命周期应该是什么。
// 在这种情况，编译器会给出一个错误，这可以通过增加对应引用之间相联系的生命周期注解来解决。

///* 函数或方法的参数的生命周期被称为 输入生命周期（input lifetimes），
///* 而返回值的生命周期被称为 输出生命周期（output lifetimes）。

///* 生命周期省略的三个规则
// 编译器采用三条规则来判断引用何时不需要明确的注解。
// - 第一条规则适用于输入生命周期，
// - 后两条规则适用于输出生命周期。
// 如果编译器检查完这三条规则后仍然存在没有计算出生命周期的引用，编译器将会停止并生成错误。
// *这些规则适用于 fn函数、方法定义，以及 impl 块。

//* 规则 1：每一个是引用的参数都有它自己的生命周期参数
//* 规则 2：如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数
//* 规则 3：如果方法有多个输入生命周期参数并且其中一个参数是 &self 或 &mut self，说明是个对象的方法(method), 那么所有输出生命周期参数被赋予 self 的生命周期。

///* 假设我们自己就是编译器, 推断以下方法：
// ** 方法一
fn first_word(s: &str) -> &str {}

// 第一条规则，也就是每个引用参数都有其自己的生命周期
fn first_word<'a>(s: &'a str) -> &str {}

// 第二条规则，因为这里正好只有一个输入生命周期参数所以是适用的。第二条规则表明输入参数的生命周期将被赋予输出生命周期参数，所以现在签名看起来像这样：
fn first_word<'a>(s: &'a str) -> &'a str {}
// ** 现在这个函数签名中的所有引用都有了生命周期，如此编译器可以继续它的分析而无须程序员标记这个函数签名中的生命周期。

// ** 方法二
fn longest(x: &str, y: &str) -> &str {}

// 第一条规则：每个引用参数都有其自己的生命周期。这次有两个参数，所以就有两个（不同的）生命周期：
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {}

// 第二条规则，因为函数存在多个输入生命周期，它并不适用于这种情况
// 第三条规则，它同样也不适用，这是因为没有 self 参数

// ** 应用了三个规则之后编译器还没有计算出返回值类型的生命周期; 报错：编译器使用所有已知的生命周期省略规则，仍不能计算出签名中所有引用的生命周期。

// ---
///* 方法定义中的生命周期注解
// impl 块里的方法签名中，(生命周期)引用可能与结构体字段中的引用相关联，也可能是独立的
// * 生命周期省略规则也经常让我们无需在方法签名中使用生命周期注解
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        // 并不引用任何生命周期
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    // 第三条生命周期省略规则: 返回值类型被赋予了 &self 的生命周期
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

///* 静态生命周期 'static
// 其生命周期能够存活于整个程序期间。
// 所有的字符串字面值都拥有 'static 生命周期，我们也可以选择像下面这样标注出来：
fn main() {
    let s: &'static str = "I have a static lifetime.";
}
// 将引用指定为 'static 之前，思考一下这个引用是否真的在整个程序的生命周期里都有效
// * 大部分情况，代码中的问题是尝试创建一个悬垂引用或者可用的生命周期不匹配，请解决这些问题而不是指定一个 'static 的生命周期。

//
///* 结合泛型类型参数、trait bounds 和生命周期
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

///* 泛型类型参数意味着代码可以适用于不同的类型。
/// trait 和 trait bounds 保证了即使类型是泛型的，这些类型也会拥有所需要的行为。
/// 由生命周期注解所指定的引用生命周期之间的关系保证了这些灵活多变的代码不会出现悬垂引用。
/// 而所有的这一切发生在编译时所以不会影响运行时效率！
///* 更多内容 https://doc.rust-lang.org/reference/index.html