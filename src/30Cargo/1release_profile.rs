///* 发布配置（release profiles）是预定义的、可定制的带有不同选项的配置，他们允许程序员更灵活地控制代码编译的多种选项。每一个配置都彼此相互独立。

// Cargo 有两个主要的配置：
// *运行 cargo build 时采用的 dev 配置
// *运行 cargo build --release 的 release 配置。
// dev 配置被定义为开发时的好的默认配置，release 配置则有着良好的发布构建的默认配置。




///* 自定义 release profiles
// 当项目的 Cargo.toml 文件中没有任何 [profile.*] 部分的时候，Cargo 会对每一个配置都采用默认设置。通过增加任何希望定制的配置对应的 [profile.*] 部分，我们可以选择覆盖任意默认设置的子集。

// 文件名: Cargo.toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3

// *配置查看：https://doc.rust-lang.org/cargo/reference/profiles.html