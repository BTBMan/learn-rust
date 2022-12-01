use std::{cmp::Ordering, io};
// 使用了外部的rand crate
// Rng是一个trait(特征) 如果想使用随机生成器的方法 则必须在trait的作用域中
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // 生成一个随机数
    // 调用rand::thread_rng函数提供实际使用的生成器
    // 调用生成器的gen_range方法
    // 传入范围表达式
    // start..=end的形式 这里是1-100之间的数
    // 可以通过运行cargo doc --open来获取crate的文档
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    // loop循环比较 只有猜对或转换guess出错会退出循环
    loop {
        println!("Please input the guess!");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        // 将guess转为数字类型
        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        println!("You guessed: {guess}")
    }
}
