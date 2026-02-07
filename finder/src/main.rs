use finder::*;
use std::{env, process};

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("解析参数错误: {err}");
        process::exit(1);
    });

    // 也可以和处理 build 一样使用 unwrap_or_else 处理错误
    if let Err(e) = run(config) {
        // eprintln 用来重定向到 stderr, 即当我们把输入的内容作为日志输出到文件中, 如果它是 eprintln, 则只会在控制台输出, 并不会输出到文件里
        eprintln!("程序错误: {e}");
    }
}
