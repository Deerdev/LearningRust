///* crates.io[https://crates.io/] 用来分发包的源代码，所以它主要托管开源代码。

/// *编写有用的文档注释
/// *文档注释（documentation comments），会生成 HTML 文档。
// 这些 HTML 展示公有 API 文档注释的内容，他们意在让对库感兴趣的程序员理解如何 使用 这个 crate，而不是它是如何被 实现 的。
// 文档注释使用三斜杠 /// 而不是两斜杆以支持 Markdown 注解来格式化文本

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

/*
生成文档
$ cargo doc
$ cargo doc --open
*/

// * 常用（文档注释）部分/区域
// - # Examples Markdown 标题在 HTML 中创建了一个以 “Examples” 为标题的部分
// - Panics：这个函数可能会 panic! 的场景。并不希望程序崩溃的函数调用者应该确保他们不会在这些情况下调用此函数。
// - Errors：如果这个函数返回 Result，此部分描述可能会出现何种错误以及什么情况会造成这些错误，这有助于调用者编写代码来采用不同的方式处理不同的错误。
// - Safety：如果这个函数使用 unsafe 代码，这一部分应该会涉及到期望函数调用者支持的确保 unsafe 块中代码正常工作的不变条件（invariants）。

// * 文档注释作为测试
// 在文档注释中增加示例代码块 Example 是一个清楚的表明如何使用库的方法，这么做还有一个额外的好处：cargo test 也会像测试那样运行文档中的示例代码！

// * 注释包含项的结构 (给外层注释)
// `//!`


// 如在 src/lib.rs 内，注释是未 Crate 的这个模块做注释


//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given.
// --snip--
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}


///* 使用 pub use 导出合适的公有 API（对外重新导出一个 mod 描述）
// 问题：使用 mod 关键字来将代码组织进模块中，如何使用 pub 关键字将项变为公有，和如何使用 use 关键字将项引入作用域。然而你开发时候使用的文件架构可能并不方便用户。你的结构可能是一个包含多个层级的分层结构，不过这对于用户来说并不方便。这是因为想要使用被定义在很深层级中的类型的人可能很难发现这些类型的存在。
// 例如：他们也可能会厌烦要使用 use my_crate::some_module::another_module::UsefulType; 而不是 use my_crate::UsefulType; 来使用类型。

// 解决：无需重新安排内部组织：你可以选择使用 pub use 重导出（re-export）项来使公有结构不同于私有结构。重导出获取位于一个位置的公有项并将其公开到另一个位置，好像它就定义在这个新位置一样。

// 在 src/lib 中 pub use 导出深层级 mod，外部可以直接 use (类似 export)
pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;


///* 创建 Crates.io 账号
/*
在你可以发布任何 crate 之前，需要在 crates.io 上注册账号并获取一个 API token。为此，访问位于 crates.io 的首页并使用 GitHub 账号登陆。（目前 GitHub 账号是必须的，不过将来该网站可能会支持其他创建账号的方法）一旦登陆之后，查看位于 https://crates.io/me/ 的账户设置页面并获取 API token。接着使用该 API token 运行 cargo login 命令，像这样：

$ cargo login abcdefghijklmnopqrstuvwxyz012345

这个命令会通知 Cargo 你的 API token 并将其储存在本地的 ~/.cargo/credentials 文件中。注意这个 token 是一个 秘密（secret）且不应该与其他人共享。如果因为任何原因与他人共享了这个信息，应该立即到 crates.io 重新生成这个 token。
*/


///* 发布新 crate 之前
// 在 Cargo.toml 的[package]内添加元数据
// - name 唯一名称
// - description 描述，被搜索的内容
// - license license 标识符值（license identifier value） https://spdx.org/licenses/
// - version
// - author


///* 发布 cargo publish
// 发布是永久的，版本无法覆盖，代码无法 删除

///* 发布现存 crate 的新版本
// 更改 version 重新发布

///* 使用 cargo yank 从 Crates.io 撤回版本
// 不能删除之前版本的 crate, 但是可以阻止任何将来的项目将他们加入到依赖中
// 撤回某个版本会阻止新项目开始依赖此版本，不过所有现存此依赖的项目仍然能够下载和依赖这个版本。
// - 撤回意味着所有带有 Cargo.lock 的项目的依赖不会被破坏，
// - 同时任何新生成的 Cargo.lock 将不能使用被撤回的版本。
// * 命令
// 撤回 cargo yank --vers 1.0.1
// 撤销撤回操作 cargo yank --vers 1.0.1 --undo


