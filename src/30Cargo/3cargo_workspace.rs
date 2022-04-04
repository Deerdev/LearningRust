///* Workspace
// 随着项目开发的深入，库 crate 持续增大，而你希望将其进一步拆分成多个库 crate。对于这种情况，Cargo 提供了一个叫 工作空间（workspaces）的功能，它可以帮助我们管理多个相关的协同开发的包。


///* 创建工作空间 (类似 monorepo)
// 工作空间 是一系列共享同样的 Cargo.lock 和输出目录的包

// 文件名: Cargo.toml
[workspace]
members = [
    "adder",
    "add_one",
]

/*
├── Cargo.lock // 全局 lock 锁，所有 crate 共享
├── Cargo.toml
├── add_one
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
*/

// * 互相依赖，指明 path
// adder/Cargo.toml
[dependencies]
add_one = { path = "../add_one" }

// * 为了在顶层 add 目录运行二进制 crate，可以通过 -p 参数和包名称来运行 cargo run 指定工作空间中我们希望使用的包：
$ cargo run -p adder

///* 在工作空间中依赖外部包
// 所有 crate 的依赖必须显式声明在自己的 Cargo.toml 中
// 只在根目录有一个 Cargo.lock，而不是在每一个 crate 目录都有 Cargo.lock。这确保了所有的 crate 都使用完全相同版本的依赖。如果在 Cargo.toml 和 add-one/Cargo.toml 中都增加 rand crate，则 Cargo 会将其都解析为同一版本并记录到唯一的 Cargo.lock 中。使得工作空间中的所有 crate 都使用相同的依赖意味着其中的 crate 都是相互兼容的


///* 为工作空间增加测试
// 根目录 cargo test 会执行所有 crate 的 test，通过 -p 指定运行哪个 crate
cargo test -p add_one
