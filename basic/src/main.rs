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
    // 结构体
    {
        // {
        //     // 创建结构体
        //     #[derive(Debug)]
        //     struct User {
        //         name: String,
        //         age: u64,
        //         active: bool,
        //     }

        //     // 创建结构体实例
        //     // If you wanna change the value, you must to change the variable to be mutable.
        //     let mut user1 = User {
        //         active: true,
        //         age: 18,
        //         name: String::from("Bob"),
        //     };

        //     // Just like javascript (es6) ... operation.
        //     let user2 = User {
        //         name: String::from("Sun"),
        //         active: false,
        //         ..user1 // 结构更新语法在最后面
        //     };

        //     // let user3 = User {
        //     //     name: user2.name, // 这么做 user2 的 name 会失去所有权
        //     //     ..user2
        //     // };

        //     change_name(&mut user1, &"Kevin");

        //     fn change_name(user: &mut User, name: &str) {
        //         user.name = name.to_string();
        //     }

        //     println!("{:?}", user1);
        //     println!("{:?}", user2);
        //     // println!("{:?}", user3);
        // }

        // 元祖结构体
        // {
        //     #[derive(Debug)]
        //     struct Color(i32, i32, char);

        //     let c = Color(1, 2, 'a');

        //     println!("{:?}", c);
        //     dbg!(&c);
        // }
        // 单元结构体
        // {
        //     #[derive(Debug)]
        //     struct AB; // 定义了一个单元体(啥也没有)的结构体

        //     let a = AB;

        //     println!("{:?}", a)
        // }
        // {
        //     #[derive(Debug)]
        //     struct Shape {
        //         width: i32,
        //         height: i32,
        //     }

        //     let rect = Shape {
        //         width: 3,
        //         height: 5,
        //     };

        //     dbg!(&rect);
        // }
        // 结构体内避免使用引用类型标注 编译器会报错 如果使用引用 则必加上用生命周期
        // 枚举
        // {
        //     #[derive(Debug)]
        //     enum Type {
        //         Input(u8), // 类型也可以省略
        //         Checkbox { a: char },
        //         Radio(bool),
        //     }

        //     let input = Type::Input(1);
        //     let checkbox = Type::Checkbox { a: 'a' };
        //     let radio = Type::Radio(true);

        //     println!("{:?},{:?},{:?}", input, checkbox, radio);
        // }
        // Option 枚举, 用来表示当前变量的值是缺失的, 用来代替 null
        // {
        //     // 泛型T
        //     enum Option<T> {
        //         Some(T), // 含有值
        //         None,    // 没有值
        //     }

        //     // Some 和 None 可以直接省略 不必要通过 Option::Some 这种使用方式
        //     let some_num = Some(5);
        //     let absent_number: Option<i32> = None; // 使用None时须要指定Option的T的类型

        //     // 如果想使用一个可能为空的值 那就必须明确处理值为空的情况
        // }
        // 数组
        // {
        //     // 长度固定的数组(array)存储在栈中
        //     {
        //         // {
        //         //     // 长度固定 类型统一
        //         //     let arr1 = [1, 2, 3, 4];
        //         //     let arr2 = [2; 5]; // 重复出现5次2

        //         //     println!("{}", arr1[0]);
        //         //     // println!("{}", arr1[4]); // 不可以跨界访问数组元素
        //         //     println!("{:?}, {:?}", arr1, arr2);
        //         // }
        //         // 快速生成多个重复的复杂数据类型的元素
        //         // {
        //         //     let arr1: [String; 8] = std::array::from_fn(|_i| String::from("Hello"));

        //         //     println!("{:?}", arr1);
        //         // }
        //     }

        //     // 动态数组(Vector)存储在堆中
        //     {
        //         //
        //     }
        // }
        // flow control
        // 如果是复杂数据类型 一般都须要使用他的引用 除非后续不再使用这个数据了
        // 如何循环的是数组 尽量直接使用数组循环 不要生成下标循环在通过循环后的每个下标去访问数组 不安全 + 性能损耗
        // {
        //     // {
        //     //     // for i in 1..5 {
        //     //     //     println!("{i}")
        //     //     // }

        //     //     // for i in 1..=5 {
        //     //     //     println!("{i}")
        //     //     // }
        //     // }
        //     // {
        //     //     let arr1 = [1, 2, 3, 4];

        //     //     for i in arr1 {
        //     //         println!("{i}");
        //     //     }

        //     //     // 变为迭代器后进行循环
        //     //     for i in arr1.iter().enumerate() {
        //     //         println!("{:?}", i);
        //     //     }
        //     // }
        // }
        // match
        {
            // match
            // {
            //     // {
            //     //     enum Type {
            //     //         Input,
            //     //         Button,
            //     //         Checkbox,
            //     //         Radio,
            //     //     };

            //     //     let input = Type::Input;

            //     //     // 每条模式分支的返回值类型必须一致
            //     //     // match 本身也是表达式 所以他的返回值可以赋值给其他变量 这里返回了一个单元类型
            //     //     let res = match input {
            //     //         Type::Button => println!("Button"),
            //     //         // xxx 或 xxx
            //     //         Type::Input | Type::Radio => {
            //     //             println!("Input or Radio");
            //     //         }
            //     //         _ => println!("Checkbox"), // 其他情况
            //     //     };

            //     //     println!("{:?}", res); // ()
            //     // }
            //     // 模式绑定
            //     // {
            //     //     // 在模式中取出绑定的值
            //     //     #[derive(Debug)]
            //     //     enum Shape {
            //     //         Circle,
            //     //         Rect,
            //     //     }

            //     //     enum Type {
            //     //         Input,
            //     //         Button(Shape),
            //     //     }

            //     //     let button = Type::Button(Shape::Rect);

            //     //     match button {
            //     //         Type::Button(shape) => {
            //     //             println!("{:?}", shape)
            //     //         }
            //     //         _ => println!("Input"),
            //     //     }
            //     // }
            // }
            // if let 只匹配一个条件时使用
            // {
            //     let v = Some(3);

            //     if let Some(3) = v {
            //         println!("3");
            //     }

            //     // 这里 x 为变量 (?可以理解为形参吗)
            //     if let Some(x) = v {
            //         println!("3, {x}");
            //     }

            //     let y = 4;
            //     if let Some(y) = v {
            //         println!("3, {y}");
            //     }
            // }
            // matches!宏 直接返回 true 或 false
            // {
            //     let foo = 'A';
            //     let res1 = matches!(foo, 'a'..='z' | 'A'..='Z');

            //     println!("{res1}");

            //     let bar = Some(4);
            //     let res2 = matches!(bar, Some(x) if x > 2);

            //     println!("{res2}");
            // }
            // 无论是match还是if let都会发生变量遮蔽 语句中的变量会遮蔽上层变量
            // 结构 Option
            // {
            //     fn plus_one(x: Option<i32>) -> Option<i32> {
            //         match x {
            //             None => None,
            //             Some(x) => Some(x + 1),
            //         }
            //     }

            //     let v1 = Some(5);
            //     let v2 = plus_one(v1);
            //     let v3 = plus_one(None);

            //     println!("{:?},{:?},{:?}", v1, v2, v3);
            // }
            // while let
            // {
            //     // Vec是动态数组
            //     let mut stack = Vec::new();

            //     // 向数组尾部插入元素
            //     stack.push(1);
            //     stack.push(2);
            //     stack.push(3);

            //     // stack.pop从数组尾部弹出元素
            //     while let Some(top) = stack.pop() {
            //         println!("{}", top);
            //     }
            // }
            // 匹配范围
            // {
            //     let x1 = 5;

            //     match x1 {
            //         1..=5 => println!("matched"),
            //         _ => println!("not matched")
            //     }
            // }
            // construction
            // {
            //     struct Point {
            //         x: i32,
            //         y: i32,
            //     }

            //     let p = Point { x: 3, y: 4 };

            //     let Point { x, y } = p;
            //     println!("{x}, {y}");

            //     // variable alias
            //     let Point { x: x1, y: y1 } = p;
            //     println!("{x1}, {y1}");
            // }
            // 匹配结构体
            // {
            //     struct Point {
            //         x: i32,
            //         y: i32,
            //     }

            //     let p = Point { x: 1, y: 9 };

            //     match p {
            //         Point { x: 1, y } => println!("matched x = 1, y is {y}"), // 只匹配 x
            //         Point { x: _, y: 2 } => println!("matched y = 2"),        // 只匹配 y
            //         Point { x: _, y: __ } => println!("not matched anything"),
            //     }
            // }
            // construct nested enum
            // {
            //     enum ColorType {
            //         Rgb(i32, i32, i32),
            //         Hsv(i32, i32, i32),
            //     }

            //     enum Med {
            //         ChangeColor(ColorType),
            //         Move,
            //     }

            //     let ins = Med::ChangeColor(ColorType::Rgb(1, 2, 3));
            //     // let ins = Med::ChangeColor(ColorType::Hsv(1, 2, 3));

            //     match ins {
            //         Med::ChangeColor(ColorType::Rgb(_, __, ___)) => println!("change color"),
            //         _ => println!("not"),
            //     }
            // }
            // 解构嵌套的元祖和结构体
            // {
            //     #[derive(Debug)]
            //     struct Point {
            //         x: i32,
            //         y: i32,
            //         color: Color,
            //     }

            //     #[derive(Debug)]
            //     struct Color {
            //         rgb: (i32, i32, i32),
            //     }

            //     // let (a, b) = ((1, 3), Point { x: 4, y: 10 });
            //     let (
            //         (a, b),
            //         Point {
            //             x,
            //             y,
            //             color: Color { rgb: (aa, bb, cc) },
            //         },
            //     ) = (
            //         (1, 3),
            //         Point {
            //             x: 4,
            //             y: 10,
            //             color: Color { rgb: (11, 22, 33) },
            //         },
            //     );
            //     println!(
            //         "{:?}, {:?}, {:?}, {:?}, {:?},{:?},{:?}",
            //         a, b, x, y, aa, bb, cc
            //     );
            // }
        }
    }
}
