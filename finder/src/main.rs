use finder::*;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("解析参数错误: {err}");
        process::exit(1);
    });

    // 也可以和处理 build 一样使用 unwrap_or_else 处理错误
    if let Err(e) = run(config) {
        println!("程序错误: {e}");
    }
}
