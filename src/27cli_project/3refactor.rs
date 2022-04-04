///* 二进制项目的关注分离
/*
main 函数负责多个任务的组织问题在许多二进制项目中很常见。所以 Rust 社区开发出一类在 main 函数开始变得庞大时进行二进制程序的关注分离的指导性过程。这些过程有如下步骤：

- 将程序拆分成 main.rs 和 lib.rs 并将程序的逻辑放入 lib.rs 中。
- 当命令行解析逻辑比较小时，可以保留在 main.rs 中。
- 当命令行解析开始变得复杂时，也同样将其从 main.rs 提取到 lib.rs 中。


经过这些过程之后保留在 main 函数中的责任应该被限制为：

- 使用参数值调用命令行解析逻辑
- 设置任何其他的配置
- 调用 lib.rs 中的 run 函数
- 如果 run 返回错误，则处理这个错误

这个模式的一切就是为了关注分离：main.rs 处理程序运行，而 lib.rs 处理所有的真正的任务逻辑。因为不能直接测试 main 函数，这个结构通过将所有的程序逻辑移动到 lib.rs 的函数中使得我们可以测试他们。仅仅保留在 main.rs 中的代码将足够小以便阅读就可以验证其正确性。让我们遵循这些步骤来重构程序。
*/

///* 1. 提取参数解析器
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, filename) = parse_config(&args);

    // --snip--

    println!("Searching for {}", query);
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}

///* 2.组合配置值
// 注意：一些同学将这种在复杂类型更为合适的场景下使用基本类型的反模式称为 基本类型偏执（primitive obsession）。
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    // --snip--

    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    //? main 中的 args 变量是参数值的所有者并只允许 parse_config 函数借用他们，这意味着如果 Config 尝试获取 args 中值的所有权将违反 Rust 的借用规则。
    // let query = args[1];
    // let filename = args[2];

    // * 简单但有些不太高效的方式是调用这些值的 clone 方法
    // 拷贝数据使得代码显得更加直白因为无需管理引用的生命周期，所以在这种情况下牺牲一小部分性能来换取简洁性的取舍是值得的
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}

/*
使用 clone 的权衡取舍
由于其运行时消耗，许多 Rustacean 之间有一个趋势是倾向于避免使用 clone 来解决所有权问题。在关于迭代器中，将会学习如何更有效率的处理这种情况，不过现在，复制一些字符串来取得进展是没有问题的，因为只会进行一次这样的拷贝，而且文件名和要搜索的字符串都比较短。在第一轮编写时拥有一个可以工作但有点低效的程序要比尝试过度优化代码更好一些。随着你对 Rust 更加熟练，将能更轻松的直奔合适的方法，不过现在调用 clone 是完全可以接受的。
*/

///* 3. 创建一个 Config 的构造函数
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    // --snip--
}

// --snip--
impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}

///* 4. 改善错误处理
use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    // * main 函数来处理现在 Config::new 返回的 Result。另外还需要手动实现原先由 panic!负责的工作，即以非零错误码退出命令行工具的工作
    // unwrap_or_else ok： 返回，err：触发闭包
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    // 从 new 中返回 Result 而不是调用 panic!
    // 出现错误时带有一个 &'static str
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

///* 5.从 main 提取逻辑
//
use std::env;
use std::fs;
use std::process;

fn main() {
    // --snip--
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // 这里 run 没有返回结果，所以不用 unwrap_or_else
    if let Err(e) = run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}

///* 将代码拆分到库 crate
/*
让我们将所有不是 main 函数的代码从 src/main.rs 移动到新文件 src/lib.rs 中：
- run 函数定义
- 相关的 use 语句
- Config 的定义
- Config::new 函数定义
*/
// 文件名: src/lib.rs
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        // --snip--
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // --snip--
}

// ---
// Filename: src/main.rs
use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // --snip--
    if let Err(e) = minigrep::run(config) {
        // --snip--
    }
}
