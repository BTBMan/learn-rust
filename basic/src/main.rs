// use num::complex::Complex;

fn main() {
    // 解构
    // {
    //     let (a, mut b) = (true, false);
    //     println!("{a}, {b}");

    //     b = true;
    //     println!("{b}");

    //     assert_eq!(a, b)
    // }
    // 解构式赋值
    // {
    //     struct Number {
    //         e: i32,
    //     }

    //     let (a, b, c, e, d);

    //     // 元祖
    //     (a, b) = (1, 2);

    //     // 数组
    //     [c, .., d, _] = [1, 2, 3, 4, 5];
    //     Number { e } = Number { e: 5 };

    //     println!("{a},{b},{c},{d},{e}");
    // }
    // 类型推导
    // {
    //     // let guess: i32 = "42".parse().expect("Not a number");
    //     let guess = "42".parse::<i32>().expect("Not a number");

    //     println!("{guess}")
    // }
    // 也存在浮点数丢精问题
    // {
    //     let sum = 0.1 + 0.2;

    //     println!("{sum}");
    // }
    // NaN
    // {
    //     let x = (-42.0_f32).sqrt();

    //     if x.is_nan() {
    //         println!("{x}")
    //     }
    // }
    // 序列
    // {
    //     let nums = 1..5;

    //     println!("{:?}", nums)
    // }
    // 数值库使用
    // {
    //     let a = Complex { re: 2.1, im: -1.2 };
    //     let b = Complex::new(11.1, 22.2);

    //     println!("{},{}", a, b)
    // }
    // 单元类型
    // {
    //     // () 零长度元祖就是一个单元类型, 比如函数不显示返回任何值/类型 那么它默认就返回单元类型
    // }
    // 用不返回的发散函数
    // {
    //     // 使用 ! 标识返回类型
    //     fn dead() -> ! {
    //         // 导致程序崩溃的代码
    //         loop {
    //             //
    //         }
    //     }
    // }
    // 解引用
    // {
    //     let a = 5;
    //     let b = &a;

    //     println!("{a},{b}");
    //     assert_eq!(5, *b); // 解引用之后才能访问b所指向的值 然后进行比较. 不解引用的话 b是个引用 不能与整型进行比较
    // }
    // 字符串拼接
    // {
    //     let s1 = String::from("Hello");
    //     let s2 = String::from("world");

    //     // String 必须和一个字符串切片($str)进行拼接
    //     let s3 = s1 + &s2;

    //     println!("{s3}");
    // }
    // 字符串转义
    // {
    //     let s1 = "I'm writing \x52\x75\x73\x74!";

    //     println!("{s1}");

    //     // 保持原样
    //     println!("{}", "hello \\x52\\x75\\x73\\x74");

    //     let raw_str = r"I'm writing \x52\x75\x73\x74!";

    //     println!("{raw_str}");

    //     let quotes = r#"And then I said: "There is no escape!""#;

    //     println!("{}", quotes);

    //     let longer_delimiter = r###"A string with "# in it. And even "##!"###;

    //     println!("{}", longer_delimiter);
    // }
}
