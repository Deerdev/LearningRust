///* 大部分终端都提供了两种输出：
/// 标准输出（standard output，stdout）对应一般信息 [println! ]
/// 标准错误（standard error，stderr）则用于错误信息。[eprintln!]

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        // 错误：将错误打印到标准错误
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        // 错误：将错误打印到标准错误
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}
