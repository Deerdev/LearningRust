/// 宏深入文档：https://veykril.github.io/tlborm/
///* 宏（Macro）指的是 Rust 中一组相关特性的集合称谓
/// 使用 macro_rules! 的 「声明（Declarative）宏」，和三种 「过程（Procedural）宏」：
// - 自定义 #[derive] 宏，在结构体和枚举上指定通过 derive 属性添加的代码
// - 类属性（Attribute-like）宏，定义可用于任意项的自定义属性
// - 类函数宏，看起来像函数, 对其指定为参数的 token 进行操作

///* 宏和函数的区别
// 宏是 用来编写可以生产其他代码的代码，即所谓的 元编程（metaprogramming）
// 一个函数标签必须声明函数参数个数和类型。相比，宏可处理可变的参数：用一个参数调用 println!("hello") 或用两个参数调用 println!("hello {}", name)
// 宏可以在编译器解释代码前展开
// 宏定义要比函数定义更复杂，难以阅读、理解、维护
// 在一个文件里调用宏 之前 必须定义它，或将其引入当前作用域
// 而函数则可以在任何地方定义和调用。

///* 使用 macro_rules! 的声明宏用于通用元编程
/// macro_rules! 即将弃用，macro_rules! 中有一些奇怪的地方。在将来，会有第二种采用 macro 关键字的声明宏，其工作方式类似但修复了这些极端情况。在此之后，macro_rules! 实际上就过时（deprecated）了。
// Rust 最常用的宏形式是 声明宏（declarative macros）
// 它们有时也被称为 “macros by example”、“macro_rules! 宏” 或者就是 “macros”。
//
// 定义：
// - 类似 match 的模式匹配
// - 需要使用 macro_rules!

//
// let v: Vec<u32> = vec![1, 2, 3];
// * vec! 宏的定义
#[macro_export] // 所处的包被引入作用域后，才可以使用（不加，就不能被引入作用域）
macro_rules! vec { // 使用 macro_rules! 声明宏，结尾没有 !
    // 类似 match 的分支，只有一个分支，一种匹配模式
    // match 匹配的是值，这里匹配的是 rust 的代码结构
    // 也可以定义多个匹配模式的宏，类似正则 匹配哪个执行哪个
    //
    // $x:expr 匹配任意的表达式，将其命名为表达式 $x
    // $( $x:expr ), 表达式后面可能出现','分隔符，匹配`1,2,3`
    // $( $x:expr ),* '*' 标识可以匹配零个或多个
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            // 根据'*'匹配的个数，生成对应数量的代码
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
/*
首先，一对括号包含了整个模式。接下来是美元符号（ $ ），后跟一对括号，捕获了符合括号内模式的值以用于替换后的代码。
$() 内则是 $x:expr ，其匹配 Rust 的任意表达式，并将该表达式记作 $x。

$() 之后的逗号说明一个可有可无的逗号分隔符可以出现在 $() 所匹配的代码之后。
紧随逗号之后的 * 说明该模式匹配零个或更多个 * 之前的任何模式。
 */

/*
* 实际代码类似于：
{
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
}
 */

///* 基于属性来生成代码的过程宏
// 第二种形式的宏被称为 过程宏（procedural macros），因为它们更像函数（一种过程类型）。
// 过程宏接收 Rust 代码作为输入，在这些代码上进行操作，然后产生另一些代码作为输出。
// - 而非像声明式宏那样匹配对应模式然后以另一部分代码替换当前代码。

//* 有三种类型的过程宏（自定义派生（derive），属性宏 和 函数宏）
//
// 创建过程宏时，其定义 必须单独放在它们自己的包中，并使用特殊的包/crate类型
//
use proc_macro; // 引入TokenStream

// 其中 some_attribute 是一个用来指定过程宏类型的占位符
#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {} // 接收 TokenStream ，产生另一个 TokenStream
///
/*
TokenStream 类型由包含在 Rust 中的 proc_macro crate 定义并表示令牌序列。 这是宏的核心：宏所操作的源代码构成了输入 TokenStream，宏产生的代码是输出 TokenStream。 该函数还附加了一个属性，用于指定我们正在创建的程序宏类型。
*/

///* 自定义 derive 宏 (有点像 Python 的装饰器)
// 需求：(类似 #[derive(Debug)] )
// - 创建一个 hello_macro crate，其包含名为 HelloMacro 的 trait 和关联函数 hello_macro
// - 提供一个过程式宏以便用户可以使用 #[derive(HelloMacro)] 注解他们的类型来得到 hello_macro 函数的默认实现
//
// --------
// main 依赖下面两个 crate
// hello_macro = { path = "../hello_macro" }
// hello_macro_derive = { path = "../hello_macro/hello_macro_derive" }
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)] // impl HelloMacro，获得 hello_macro 的默认实现
struct Pancakes;

fn main() {
    Pancakes::hello_macro();
}

// --------
// crate: proc_macro
pub trait HelloMacro {
    fn hello_macro();
}

// -------
// crate: hello_macro_derive 独立一个包
// 我们需要声明 hello_macro_derive crate 是过程宏(proc-macro) crate `proc-macro = true`
/*
[lib]
proc-macro = true

[dependencies]
syn = "1.0"
quote = "1.0"

 */
extern crate proc_macro;

use proc_macro::TokenStream; // 内置依赖，操作 rust 代码
use quote::quote; // 读取 syn 产生的数据结构(ast), 重新转化为 rust 代码
use syn; // 把 rust 代码从字符串转为可操作的结构（ast）

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    // 解析 TokenStream -> ast
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

// 过程宏具体要实现的效果
fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    // 获取 name 结构
    let name = &ast.ident;
    // 使用 name 构建 impl
    // quote! 这个宏也提供了一些非常酷的模板机制；我们可以写 #name ，然后 quote! 会以名为 name 的变量值来替换它
    // 语法：https://docs.rs/quote
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                // stringify! 编译时，将#name 真实的值转为字符串字面值，`1+2`表达式，会转为 `"1+2"`字符串，不会计算表达式
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    // 中间代码 转换为 TokenStream
    gen.into()
}

// 解析后的代码(以解析 Pancakes 为例)：
/*`````
DeriveInput {
    // --snip--

    ident: Ident {
        ident: "Pancakes",
        span: #0 bytes(95..103)
    },
    data: Struct(
        DataStruct {
            struct_token: Struct,
            fields: Unit,
            semi_token: Some(
                Semi
            )
        }
    )
}
 */
/*
此处所使用的 stringify! 为 Rust 内置宏。
其接收一个 Rust 表达式，如 1 + 2 ， 然后在编译时将表达式转换为一个字符串常量，如 "1 + 2" 。
这与 format! 或 println! 是不同的，它计算表达式并将结果转换为 String 。
有一种可能的情况是，所输入的 #name 可能是一个需要打印的表达式，因此我们用 stringify! 。
stringify! 编译时也保留了一份将 #name 转换为字符串之后的内存分配。
 */

///* 属性宏
// 类属性宏与自定义派生宏相似
// 不同于为 derive 属性生成代码，它们允许你创建新的属性
// derive 只能用于结构体和枚举；属性还可以用于其它的项，比如函数

#[route(GET, "/")]
fn index() {}

#[proc_macro_attribute]
// attr: 是所有入参 「GET, "/"」
// item: 是 index 函数
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {}

// * 类属性宏与自定义派生宏工作方式一致：创建 proc-macro crate 类型的 crate 并实现希望生成代码的函数！

///* 函数宏
// 类函数宏定义看起来像函数调用的宏，当比普通函数更加灵活
// 函数宏可以接受 TokenStream 作为参数
// 和另两种过程宏一样，在定义中使用 Rust 代码操纵 TokenStream
fn main1() {
    // 解析其中的 SQL 语句并检查其是否是句法正确的，这是比 macro_rules! 可以做到的更为复杂的处理
    let sql = sql!(SELECT * FROM posts WHERE id=1);
}

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {}
