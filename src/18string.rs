/// 字符串
// Rust 的核心语言中只有一种字符串类型：str，字符串 slice，它通常以被借用的形式出现，&str
// 字符串 slice：它们是一些储存在别处的 UTF-8 编码字符串数据的引用
// 称作 String 的类型是由标准库提供的，而没有写进核心语言部分，它是可增长的、可变的、有所有权的、UTF-8 编码的字符串类型

// “字符串” 通常指的是 String 和字符串 slice &str 类型
// String 和字符串 slice 都是 UTF-8 编码的。

// Rust 标准库中还包含一系列其他字符串类型，比如 OsString、OsStr、CString 和 CStr

/// 新建字符串
fn main1() {
    // 空字符串，mut
    let mut s1 = String::new();

    // 使用 to_string 方法，它能用于任何实现了 Display trait 的类型，字符串字面值也实现了它
    let s2 = "initial contents".to_string();

    let s3 = String::from("initial contents");

    // 记住字符串是 UTF-8 编码的，所以可以包含任何可以正确编码的数据
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
}

/// 更新字符串

/// 使用 push_str 和 push 附加字符串
fn main2() {
    let mut s = String::from("foo");
    s.push_str("bar"); // foobar

    // 添加
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    // push_str 方法采用字符串 slice, 调用之后，依然可以使用 s2
    println!("s2 is {}", s2); // 不报错

    // 单个字符: push 方法被定义为获取一个单独的字符作为参数
    let mut s3 = String::from("lo");
    s3.push('l'); // lol
}

/// 使用 + 运算符或 format! 宏拼接字符串
fn main3() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // + 会调用 add 方法 `fn add(self, s: &str) -> String`, 第二个参数是引用
    let s3 = s1 + &s2;
    // 注意 s1 被移动了，不能继续使用；s2 可以正常使用

    /// 上述：&s2 的类型是 &String 而不是 &str。那么为什么还能编译呢？
    // 因为 &String 可以被 强转（coerced）成 &str
    // 当add函数被调用时，Rust 使用了一个被称为 Deref 强制转换（deref coercion）的技术，你可以将其理解为它把 &s2 变成了 &s2[..]

    /// add 中 self 不是引用，所以 s1 所有权发生了转移
    // 实际上这个语句会获取 s1 的所有权，附加上从 s2 中拷贝的内容，并返回结果的所有权。【比使用 s1 拷贝高效】
}

/// 拼接
fn main4() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    let ss = format!("{}-{}-{}", s1, s2, s3);
}

/// 索引字符串
// Rust 的字符串不支持索引
// String 是一个 Vec<u8> 的封装，而字符可能是 Unicode 的，一个字符占据 2+ 个 byte，没办法通过索引准确的定位是哪个字符
// 因此一个字符串字节值的索引并不总是对应一个有效的 Unicode 标量值
// Rust 不允许使用索引获取 String 字符的原因是，索引操作预期总是需要常数时间 (O(1))。但是对于 String 不可能保证这样的性能，因为 Rust 必须从开头到索引位置遍历来确定有多少有效的字符。
fn main5() {
    let hello = String::from("Hola"); // 长度 4
    let hello = String::from("Здравствуйте"); // 西里尔字母, 长度 24，一个字母两个 byte
}

/// 字节、标量值 和 字形簇(最接近人们眼中 字母 的概念)
// 比如这个用梵文书写的印度语单词 “नमस्ते”，最终它储存在 vector 中的 u8 值看起来像这样：
// [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
// 这里有 18 个字节，也就是计算机最终会储存的数据。

// 如果从 Unicode 标量值的角度理解它们，也就像 Rust 的 char 类型那样，这些字节看起来像这样：
// ['न', 'म', 'स', '्', 'त', 'े']
// 这里有六个 char，不过第四个和第六个都不是字母，它们是发音符号本身并没有任何意义。

// 最后，如果以字形簇的角度理解，就会得到人们所说的构成这个单词的四个字母：
// ["न", "म", "स्", "ते"]

/// 使用字符串 slice提取（不建议）
// 返回的类型是不明确的：字节值、字符、字形簇或者字符串 slice
// slice 必须明确范围在 Unicode 的标量边界，不能在标量中间截断

fn main6() {
    let hello = "Здравствуйте";

    let s = &hello[0..4]; // s 会是一个 &str，它包含字符串的头四个字节, s 将会是 “Зд”。

    /// 如果获取 &hello[0..1] 会发生什么呢？答案是：Rust 在运行时会 panic
    // panicked at 'byte index 1 is not a char boundary; it is inside 'З' (bytes 0..2) of `Здравствуйте`'
}

/// 遍历字符串的方法
/// 操作字符串每一部分的最好的方法是明确表示需要字符还是字节。
// 对于单独的 Unicode 标量值使用 chars 方法。对 “नमस्ते” 调用 chars 方法会将其分开并返回六个 char 类型的值
fn main6() {
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    // न
    // म
    // स
    // ्
    // त
    // े
}

// bytes 方法返回每一个原始字节
fn main7() {
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
    // 224
    // 164
    // --snip--
    // 165
    // 135
}

// 字形簇遍历：从字符串中获取字形簇是很复杂的，所以标准库并没有提供这个功能。crates.io 上有些提供这样功能的 crate。

/// 总而言之，字符串还是很复杂的。不同的语言选择了不同的向程序员展示其复杂性的方式。Rust 选择了以准确的方式处理 String 数据作为所有 Rust 程序的默认行为，这意味着程序员们必须更多的思考如何预先处理 UTF-8 数据。这种权衡取舍相比其他语言更多的暴露出了字符串的复杂性，不过也使你在开发生命周期后期免于处理涉及非 ASCII 字符的错误。
