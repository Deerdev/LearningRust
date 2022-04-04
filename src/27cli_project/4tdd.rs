///* 测试驱动开发（Test Driven Development, TDD）
/*
这是一个软件开发技术，它遵循如下步骤：

- 编写一个失败的测试，并运行它以确保它失败的原因是你所期望的。
- 编写或修改足够的代码来使新的测试通过。
- 重构刚刚增加或修改的代码，并确保测试仍然能通过。
- 从步骤 1 开始重复！
*/

///* 1 编写失败测试
use std::error::Error;
use std::fs;

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}

///* 2 不断编写使测试通过的代码
// 搜索文本内容中包含 query 的行
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}

///* 3 在 run 函数中使用 search 函数
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    // grep 搜索
    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}
