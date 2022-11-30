// 引入标准库里的io库
use std::io;

// 入口
fn main() {
    // 输入内容到终端
    println!("Guess the number!");

    println!("Please input the guess!");

    // 创建一个可变的变量
    // ::语法后面的new是String类型的关联函数(静态方法)
    let mut guess = String::new(); // String::new() 返回一个String的新实例 是标准库里提供的字符串类型

    // 默认定义的变量是不可以改变的
    // let count = 1;
    // count = 1; // error
    // 定义一个可变的变量 只需要在变量名前添加 mut
    // let mut count = 1;
    // count = 2;

    // 调用io库中的stdin函数
    io::stdin()
        // 把用户输入的内容传进read_line方法中
        // 因参数是可变的 则须要加上mut
        // &的作用是引用 而不需要每次都拷贝一份
        .read_line(&mut guess)
        // Result结果用来处理潜在的错误
        .expect("Failed to read line!");

    // 这里的{}用来表示变量的占位符
    println!("You guessed: {guess}");
    // 也可以写成这样 第一个占位符接收第一个参数guess的值 以此类推
    println!("You guessed: {}", guess);
}
