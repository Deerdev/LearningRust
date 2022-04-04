///* 使用 cargo install 从 Crates.io 安装二进制文件
// cargo install 命令用于在本地安装和使用二进制 crate。
// 它并不打算替换系统中的包；它意在作为一个方便 Rust 开发者们安装其他人已经在 crates.io 上共享的工具的手段。
// *只有拥有二进制目标文件的包能够被安装。
// 二进制目标 文件是在 crate 有 src/main.rs 或者其他指定为二进制文件时所创建的可执行程序，这不同于自身不能执行但适合包含在其他程序中的库目标文件。通常 crate 的 README 文件中有该 crate 是库、二进制目标还是两者都是的信息。

// 所有来自 cargo install 的二进制文件都安装到 Rust 安装根目录的 bin 文件夹中。
// 如果你使用 rustup.rs 安装的 Rust 且没有自定义任何配置，这将是 $HOME/.cargo/bin。确保将这个目录添加到 $PATH 环境变量中就能够运行通过 cargo install 安装的程序了。

/*
$ cargo install ripgrep
    Updating crates.io index
  Downloaded ripgrep v11.0.2
  Downloaded 1 crate (243.3 KB) in 0.88s
  Installing ripgrep v11.0.2
--snip--
   Compiling ripgrep v11.0.2
    Finished release [optimized + debuginfo] target(s) in 3m 10s
  Installing ~/.cargo/bin/rg
   Installed package `ripgrep v11.0.2` (executable `rg`)
*/
// * 最后一行输出展示了安装的二进制文件的位置和名称，在这里 ripgrep 被命名为 rg。只要你像上面提到的那样将安装目录加入 $PATH，就可以运行 rg --help 并开始使用一个更快更 Rust 的工具来搜索文件了！


///* Cargo 自定义扩展命令
// Cargo 的设计使得开发者可以通过新的子命令来对 Cargo 进行扩展，而无需修改 Cargo 本身。
// *如果 $PATH 中有类似 cargo-something 的二进制文件，就可以通过 `cargo something` 来像 Cargo 子命令一样运行它。
// *像这样的自定义命令也可以运行 cargo --list 来展示出来。
// 能够通过 cargo install 向 Cargo 安装扩展并可以如内建 Cargo 工具那样运行他们是 Cargo 设计上的一个非常方便的优点！

