// use num::complex::Complex;
// use std::convert::TryInto;
// use std::prelude;

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
    //     assert_eq!(5, *b); // 解引用之后才能访问 b 所指向的值 然后进行比较. 不解引用的话 b 是个引用 不能与整型进行比较
    // }
    // 字符串拼接
    // {
    //     let s1 = String::from("Hello");
    //     let s2 = String::from("world");

    //     // String 必须和一个字符串切片 ($str) 进行拼接
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
        //     struct AB; // 定义了一个单元体 (啥也没有) 的结构体

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
        //     // 泛型 T
        //     enum Option<T> {
        //         Some(T), // 含有值
        //         None,    // 没有值
        //     }

        //     // Some 和 None 可以直接省略 不必要通过 Option::Some 这种使用方式
        //     let some_num = Some(5);
        //     let absent_number: Option<i32> = None; // 使用 None 时须要指定 Option 的 T 的类型

        //     // 如果想使用一个可能为空的值 那就必须明确处理值为空的情况
        // }
        // 数组
        // {
        //     // 长度固定的数组 (array) 存储在栈中
        //     {
        //         // {
        //         //     // 长度固定 类型统一
        //         //     let arr1 = [1, 2, 3, 4];
        //         //     let arr2 = [2; 5]; // 重复出现 5 次 2

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

        //     // 动态数组 (Vector) 存储在堆中
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
        // {
        //     // match
        //     // {
        //     //     // {
        //     //     //     enum Type {
        //     //     //         Input,
        //     //     //         Button,
        //     //     //         Checkbox,
        //     //     //         Radio,
        //     //     //     };

        //     //     //     let input = Type::Input;

        //     //     //     // 每条模式分支的返回值类型必须一致
        //     //     //     // match 本身也是表达式 所以他的返回值可以赋值给其他变量 这里返回了一个单元类型
        //     //     //     let res = match input {
        //     //     //         Type::Button => println!("Button"),
        //     //     //         // xxx 或 xxx
        //     //     //         Type::Input | Type::Radio => {
        //     //     //             println!("Input or Radio");
        //     //     //         }
        //     //     //         _ => println!("Checkbox"), // 其他情况
        //     //     //     };

        //     //     //     println!("{:?}", res); // ()
        //     //     // }
        //     //     // 模式绑定
        //     //     // {
        //     //     //     // 在模式中取出绑定的值
        //     //     //     #[derive(Debug)]
        //     //     //     enum Shape {
        //     //     //         Circle,
        //     //     //         Rect,
        //     //     //     }

        //     //     //     enum Type {
        //     //     //         Input,
        //     //     //         Button(Shape),
        //     //     //     }

        //     //     //     let button = Type::Button(Shape::Rect);

        //     //     //     match button {
        //     //     //         Type::Button(shape) => {
        //     //     //             println!("{:?}", shape)
        //     //     //         }
        //     //     //         _ => println!("Input"),
        //     //     //     }
        //     //     // }
        //     // }
        //     // if let 只匹配一个条件时使用
        //     // {
        //     //     let v = Some(3);

        //     //     if let Some(3) = v {
        //     //         println!("3");
        //     //     }

        //     //     // 这里 x 为变量 (?可以理解为形参吗)
        //     //     if let Some(x) = v {
        //     //         println!("3, {x}");
        //     //     }

        //     //     let y = 4;
        //     //     if let Some(y) = v {
        //     //         println!("3, {y}");
        //     //     }
        //     // }
        //     // matches!宏 直接返回 true 或 false
        //     // {
        //     //     let foo = 'A';
        //     //     let res1 = matches!(foo, 'a'..='z' | 'A'..='Z');

        //     //     println!("{res1}");

        //     //     let bar = Some(4);
        //     //     let res2 = matches!(bar, Some(x) if x > 2);

        //     //     println!("{res2}");
        //     // }
        //     // 无论是 match 还是 if let 都会发生变量遮蔽 语句中的变量会遮蔽上层变量
        //     // 结构 Option
        //     // {
        //     //     fn plus_one(x: Option<i32>) -> Option<i32> {
        //     //         match x {
        //     //             None => None,
        //     //             Some(x) => Some(x + 1),
        //     //         }
        //     //     }

        //     //     let v1 = Some(5);
        //     //     let v2 = plus_one(v1);
        //     //     let v3 = plus_one(None);

        //     //     println!("{:?},{:?},{:?}", v1, v2, v3);
        //     // }
        //     // while let
        //     // {
        //     //     // Vec 是动态数组
        //     //     let mut stack = Vec::new();

        //     //     // 向数组尾部插入元素
        //     //     stack.push(1);
        //     //     stack.push(2);
        //     //     stack.push(3);

        //     //     // stack.pop 从数组尾部弹出元素
        //     //     while let Some(top) = stack.pop() {
        //     //         println!("{}", top);
        //     //     }
        //     // }
        //     // 匹配范围
        //     // {
        //     //     let x1 = 5;

        //     //     match x1 {
        //     //         1..=5 => println!("matched"),
        //     //         _ => println!("not matched")
        //     //     }
        //     // }
        //     // construction
        //     // {
        //     //     struct Point {
        //     //         x: i32,
        //     //         y: i32,
        //     //     }

        //     //     let p = Point { x: 3, y: 4 };

        //     //     let Point { x, y } = p;
        //     //     println!("{x}, {y}");

        //     //     // variable alias
        //     //     let Point { x: x1, y: y1 } = p;
        //     //     println!("{x1}, {y1}");
        //     // }
        //     // 匹配结构体
        //     // {
        //     //     struct Point {
        //     //         x: i32,
        //     //         y: i32,
        //     //     }

        //     //     let p = Point { x: 1, y: 9 };

        //     //     match p {
        //     //         Point { x: 1, y } => println!("matched x = 1, y is {y}"), // 只匹配 x
        //     //         Point { x: _, y: 2 } => println!("matched y = 2"),        // 只匹配 y
        //     //         Point { x: _, y: __ } => println!("not matched anything"),
        //     //     }
        //     // }
        //     // construct nested enum
        //     // {
        //     //     enum ColorType {
        //     //         Rgb(i32, i32, i32),
        //     //         Hsv(i32, i32, i32),
        //     //     }

        //     //     enum Med {
        //     //         ChangeColor(ColorType),
        //     //         Move,
        //     //     }

        //     //     let ins = Med::ChangeColor(ColorType::Rgb(1, 2, 3));
        //     //     // let ins = Med::ChangeColor(ColorType::Hsv(1, 2, 3));

        //     //     match ins {
        //     //         Med::ChangeColor(ColorType::Rgb(_, __, ___)) => println!("change color"),
        //     //         _ => println!("not"),
        //     //     }
        //     // }
        //     // 解构嵌套的元祖和结构体
        //     // {
        //     //     #[derive(Debug)]
        //     //     struct Point {
        //     //         x: i32,
        //     //         y: i32,
        //     //         color: Color,
        //     //     }

        //     //     #[derive(Debug)]
        //     //     struct Color {
        //     //         rgb: (i32, i32, i32),
        //     //     }

        //     //     // let (a, b) = ((1, 3), Point { x: 4, y: 10 });
        //     //     let (
        //     //         (a, b),
        //     //         Point {
        //     //             x,
        //     //             y,
        //     //             color: Color { rgb: (aa, bb, cc) },
        //     //         },
        //     //     ) = (
        //     //         (1, 3),
        //     //         Point {
        //     //             x: 4,
        //     //             y: 10,
        //     //             color: Color { rgb: (11, 22, 33) },
        //     //         },
        //     //     );
        //     //     println!(
        //     //         "{:?}, {:?}, {:?}, {:?}, {:?},{:?},{:?}",
        //     //         a, b, x, y, aa, bb, cc
        //     //     );
        //     // }
        //     // 结构数组
        //     // {
        //     //     // 固定长度
        //     //     // {
        //     //     //     let arr1 = [1, 2];
        //     //     //     let [a, b] = arr1;

        //     //     //     println!("{}, {}", a, b);
        //     //     // }
        //     //     // 不固定长度
        //     //     {
        //     //         let arr1 = [1, 2];

        //     //         let [x, ..] = arr1;

        //     //         println!("{}", x);

        //     //         let arr2 = &[3, 4];
        //     //         let &[a, b] = arr2;
        //     //         let [c, d] = arr2;

        //     //         println!("{}, {}, {}, {}", a, b, c, d);

        //     //         // if let [x2, ..] = arr2 {
        //     //         //     println!("{}", x2)
        //     //         // }
        //     //     }
        //     // }
        //     // 在匹配中使用忽略模式
        //     // {
        //     //     let nums = (1, 2, 3);
        //     //     match nums {
        //     //         (3, 2, 1) => println!("3,2,1"),
        //     //         // (_, 2, _) => println!("x,2,x"), // 只匹配第二个为 2 的元祖
        //     //         (.., 3) => println!("x,x,3"), // 使用省略模式 省略忽略的值 值匹配最后一个为 3 的元祖
        //     //         _ => println!("not ..."),
        //     //     }
        //     // }
        //     // 使用下划线_加字母的变量 仍会被绑定值, 只使用下划线_则不会被绑定值 比如使用_绑定一个值的所有权 则原来的变量并不会丢失所有权.
        //     // 匹配中的额外条件
        //     // {
        //     //     let num = Some(5);

        //     //     match num {
        //     //         Some(x) if x > 4 => println!("x > 4"), // 模式中的额外 if 条件 更进一步匹配
        //     //         _ => println!("not ..."),
        //     //     }
        //     // }
        //     // 匹配多个
        //     // {
        //     //     let num1 = Some(1);
        //     //     let num2 = Some(2);

        //     //     match (num1, num2) {
        //     //         // (Some(_), Some(_)) => println!("Some"),
        //     //         // (Some(1), Some(2)) => println!("1,2"),
        //     //         (Some(1), Some(3)) => println!("1,3"),
        //     //         _ => println!("not ..."),
        //     //     }
        //     // }
        //     // 绑定 将变量绑定到一个新的变量
        //     // {
        //     //     // {
        //     //     //     struct Point {
        //     //     //         x: i32,
        //     //     //         y: i32,
        //     //     //     }

        //     //     //     let point = Point { x: 1, y: 2 };

        //     //     //     match point {
        //     //     //         Point { x: x_var @ _, .. } => println!("{}", x_var), // 将 x 绑定到新变量 x_var 上
        //     //     //     }
        //     //     // }
        //     //     // {
        //     //     //     #[derive(Debug)]
        //     //     //     struct Point {
        //     //     //         x: i32,
        //     //     //         y: i32,
        //     //     //     }

        //     //     //     // 绑定新变量的同时进行结构
        //     //     //     let p @ Point { x, y } = Point { x: 1, y: 2 };

        //     //     //     println!("{:?}, {},{}", p, x, y);
        //     //     // }
        //     // }
        // }
        // method
        // rust 中的方法往往跟 struct, enum, trait 一起使用
        // {
        //     // 简单的方法
        //     struct Point {
        //         x: i32,
        //         y: i32,
        //     }

        //     // impl 关键字定义实现方法 并与 Point 相关联
        //     impl Point {
        //         // self 是 Point 本身, 可以当作 js 中的 this 来理解
        //         // 这里的 self 只是对 Point 的所有权的不可变借用
        //         fn sum(&self) -> i32 {
        //             self.x * self.y
        //         }
        //     }

        //     // 使用
        //     let p = Point { x: 1, y: 2 };

        //     println!("sum {}", p.sum()); // 使用 sum 方法
        //     println!("x {}", p.x);
        // }
        // 私有属性
        // 默认在本文件内 所有属性都是公开的 如果在 mod 中对外导出 则所有属性就都变成私有的了 也就是说外部无法访问
        // 加了 pub 的作用是将成员变为公有的
        // {
        //     pub struct Point {
        //         x: i32,
        //         y: i32,
        //     }

        //     impl Point {
        //         // 这是一个关联函数 用作初始化当前结构体的实例 用 new 作为构造器名称是 rust 中约定俗成的规则
        //         // 大致可以用第一个参数是否是 self 来区分
        //         // 没有就是关联函数 有就是方法
        //         // 通过::来调用
        //         pub fn new(x: i32, y: i32) -> Self {
        //             Point { x, y }
        //         }

        //         pub fn test() {
        //             println!("test")
        //         }

        //         // 如果方法名和属性名同名 则一般用来实现 getter 访问器
        //         // 只将 getter 变为公有的 则外部就可以访问 并不可以进行修改 如果直接访问 Point.x 则会报错
        //         // 方法可以通过::来调用 也可以实例化过后通过.来调用
        //         pub fn x(&self) -> i32 {
        //             self.x
        //         }
        //     }

        //     let p1 = Point { x: 1, y: 2 };
        //     println!("{}", p1.x());

        //     Point::test();
        //     println!("{}", Point::x(&p1)); // 这样调用则必须传入必要的参数

        //     let p2 = Point::new(2, 4); // 使用关联函数 new 来构造 初始化数据等
        //     println!("{}", p2.x());
        // }
        // 所个 impl 块
        // {
        //     struct Point {
        //         x: i32,
        //         y: i32,
        //     }

        //     impl Point {
        //         fn test1(&self) {
        //             println!("test1")
        //         }
        //     }

        //     impl Point {
        //         fn test2(&self) {
        //             println!("test2")
        //         }
        //     }

        //     let p = Point { x: 1, y: 1 };
        //     p.test1();
        //     p.test2();
        // }
        // 为枚举实现方法
        // {
        //     #[derive(Debug)]
        //     enum Color {
        //         Rgb(i32, i32, i32),
        //         Hsv(i32, i32, i32),
        //     }

        //     impl Color {
        //         fn value(&self) {
        //             println!("{:?}", self);
        //         }
        //     }

        //     let c = Color::Rgb(1, 2, 3);
        //     c.value()
        // }
        // 泛型 类似 ts 中泛型, 特征 类似 interface
        {
            // {
            //     fn add<T: std::ops::Add<Output = T>>(x: T, y: T) -> T {
            //         x + y
            //     }

            //     enum Enum1<T, U> {
            //         A(T),
            //         B(U),
            //     }

            //     struct Struct1<T> {
            //         a: T,
            //     }

            //     // 方法中的泛型
            //     impl<T> Struct1<T> {
            //         fn get_a<K: std::fmt::Display>(&self, other: K) -> &T {
            //             println!("{}", other);
            //             &self.a
            //         }
            //     }

            //     // const 针对值的泛型
            //     // 这里的 N 是值的泛型 数来代替数组的长度 值的类型是 usize
            //     fn display_arr<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
            //         println!("{:?}", arr)
            //     }
            //     let arr: [i32; 3] = [1, 2, 3];
            //     let arr: [i32; 2] = [1, 2];

            //     // const 泛型表达式 where 在下面特征中有说明
            // }
            // trait 特征
            // 可以理解为定义特征就是 interface 接口 实现特征为继承接口
            // {
            //     // {
            //     //     trait Action {
            //     //         fn barking(&self) -> &String;
            //     //     }

            //     //     struct Dog {
            //     //         voice: String,
            //     //     }

            //     //     // 实现 特征 为 xxx
            //     //     impl Action for Dog {
            //     //         fn barking(&self) -> &String {
            //     //             &self.voice
            //     //         }
            //     //     }

            //     //     struct Cat {
            //     //         voice: String,
            //     //     }

            //     //     impl Action for Cat {
            //     //         fn barking(&self) -> &String {
            //     //             &self.voice
            //     //         }
            //     //     }

            //     //     let dog = Dog {
            //     //         voice: "wang".to_string(),
            //     //     };
            //     //     let cat = Cat {
            //     //         voice: "mao".to_string(),
            //     //     };

            //     //     println!("{:?}, {:?}", dog.barking(), cat.barking());
            //     // }
            //     // 为 Â 实现 B 特征, 则他俩至少有一个是在当前作用域当中
            //     // 你可以为标准库中的类型实现你自定义的特征 也可以为你自定义的类型标准库中的特征 但是不可以为标准库中的类型实现标准库中的特征 因为他们都没在当前的作用域内
            //     // 特征中的默认实现方法和重载特征中的方法
            //     // {
            //     //     trait Action {
            //     //         // 默认实现
            //     //         // 注意 想在实例化后通过.访问 就必须加上 &self 把函数作为方法
            //     //         fn barking(&self) {
            //     //             println!("HaHa");
            //     //         }
            //     //     }

            //     //     struct Dog {
            //     //         voice: String,
            //     //     }

            //     //     struct Cat {
            //     //         voice: String,
            //     //     }

            //     //     impl Action for Dog {}

            //     //     impl Action for Cat {
            //     //         fn barking(&self) {
            //     //             println!("mao");
            //     //         }
            //     //     }

            //     //     let dog = Dog {
            //     //         voice: "wang".to_string(),
            //     //     };
            //     //     let cat = Cat {
            //     //         voice: "mao".to_string(),
            //     //     };

            //     //     dog.barking();
            //     //     cat.barking();
            //     // }
            //     // 特征定默认实现里可以调用具有相同特征中的其他方法
            //     // {
            //     //     trait Action {
            //     //         fn action1(&self);
            //     //         fn action2(&self) {
            //     //             self.action1(); // 调用自身的 action1
            //     //         }
            //     //     }

            //     //     struct User;

            //     //     impl Action for User {
            //     //         fn action1(&self) {
            //     //             println!("action 1 is called");
            //     //         }
            //     //     }

            //     //     let user = User {};

            //     //     user.action2();
            //     // }
            //     // 把特征当作函数参数传递 + 特征约束
            //     // {
            //     //     trait Action {
            //     //         fn action1(&self);
            //     //     }

            //     //     struct User;

            //     //     impl Action for User {
            //     //         fn action1(&self) {
            //     //             println!("action 1 is called");
            //     //         }
            //     //     }

            //     //     // 此处定义的参数 action 的类型是必须实现了 Action 特征的
            //     //     // 这种书写形势只是一个语法糖
            //     //     // fn doit(action: &impl Action) {
            //     //     //     action.action1()
            //     //     // }
            //     //     // 这是完整的书写形式 称为特征约束 用来约束泛型 T 的类型是必须实现了 Action 特征的
            //     //     fn doit<T: Action>(action: &T) {
            //     //         action.action1();
            //     //     }

            //     //     let user = User;

            //     //     doit(&user);

            //     //     // 双重约束
            //     //     trait Fade {
            //     //         fn show(&self);
            //     //     }

            //     //     // 再次为 user 实现 Fade 特征
            //     //     impl Fade for User {
            //     //         fn show(&self) {
            //     //             println!("user has show fn");
            //     //         }
            //     //     }

            //     //     // 参数 p 必须是实现了 Action 和 Fade 特征的
            //     //     // fn multi(p: &(impl Action + Fade)) {
            //     //     //     p.action1();
            //     //     //     p.show();
            //     //     // }
            //     //     fn multi<T: Action + Fade>(p: &T) {
            //     //         p.action1();
            //     //         p.show();
            //     //     }

            //     //     multi(&user);

            //     //     // where 约束
            //     //     // 只是将泛型中的特征约束提取到 where 里
            //     //     {
            //     //         fn multi2<T, U>(p1: &T, p2: U)
            //     //         where
            //     //             T: Action + Fade,
            //     //             U: Fade,
            //     //         {
            //     //             p1.action1();
            //     //             p1.show();
            //     //         }
            //     //     }
            //     // }
            //     // 使用特征约束有条件的实现方法
            //     // {
            //     //     use std::fmt::Display;

            //     //     struct Pair<T> {
            //     //         x: T,
            //     //         y: T,
            //     //     }

            //     //     impl<T> Pair<T> {
            //     //         fn new(x: T, y: T) -> Self {
            //     //             Self { x, y }
            //     //         }
            //     //     }

            //     //     impl<T: Display + PartialOrd> Pair<T> {
            //     //         fn cmp_display(&self) { // 调用这个方法只有实现了 Display + PartialOrd 特征的值才可以
            //     //             if self.x >= self.y {
            //     //                 println!("The largest member is x = {}", self.x);
            //     //             } else {
            //     //                 println!("The largest member is y = {}", self.y);
            //     //             }
            //     //         }
            //     //     }

            //     //     enum A {
            //     //         A,
            //     //     }

            //     //     let pair = Pair {
            //     //         x: A::A, // 用了不被约束的特征的值 在调用相关方法会报错
            //     //         y: A::A,
            //     //         // x: String::from("value"),
            //     //         // y: String::from("value"),
            //     //     };

            //     //     pair.cmp_display();
            //     // }
            //     // 函数返回中的特征
            //     // {
            //     //     trait Action {
            //     //         fn action1(&self);
            //     //     }

            //     //     struct User;

            //     //     impl Action for User {
            //     //         fn action1(&self) {
            //     //             println!("action 1 is called");
            //     //         }
            //     //     }

            //     //     // 函数返回值是一个实现了 Action 特征的就可以
            //     //     // 只能返回一个具体的类型 例如为结构体 Animal 实现了 Action 特征 那么这个函数的返回就必须只存在这两个中的一个类型的可能
            //     //     fn return_trait() -> impl Action {
            //     //         User
            //     //     }
            //     // }
            //     // 可以通过 derive 为某个类型派生特定的特征, 这样这个类型就可以使用派生出来的方法等
            //     // 如果想使用特征中的方法 可以将这个特征引入到当前作用域中
            //     // {
            //     //     // use std::convert::TryInto; // 这个实际在最顶层引入 那样就不会报未使用的警告了

            //     //     let a: i32 = 10;
            //     //     let b: u16 = 100;

            //     //     let b_ = b.try_into().unwrap();

            //     //     if a < b_ {
            //     //         println!("Ten is less than one hundred.");
            //     //     }
            //     // }
            //     // 为自定义类型实现 Add
            //     // {
            //     //     // 引入 Add 特征
            //     //     use std::ops::Add;

            //     //     // 限制泛型 T 必须是具备 Add 特征的
            //     //     #[derive(Debug)]
            //     //     struct Point<T: Add<T, Output = T>> {
            //     //         x: T,
            //     //         y: T,
            //     //     }

            //     //     // 为 Point 结构体实现 Add 特征 这样就可以进行两个 Point 结构体相加了
            //     //     impl<T: Add<T, Output = T>> Add for Point<T> {
            //     //         type Output = Point<T>;

            //     //         // 这里是具体的相加实现 想通过 Point + Point 实现什么效果, 这里是把属性值 x 和 y 分别进行相加操作
            //     //         fn add(self, point: Point<T>) -> Point<T> {
            //     //             Point {
            //     //                 x: self.x + point.x,
            //     //                 y: self.y + point.y,
            //     //             }
            //     //         }
            //     //     }

            //     //     let p1 = Point { x: 1, y: 2 };
            //     //     let p2 = Point { x: 3, y: 4 };

            //     //     println!("{:?}", p1 + p2)
            //     // }
            //     // 为自定义类型实现格式化输出
            //     // {
            //     //     // 引入特征
            //     //     use std::fmt::Display;

            //     //     struct Point {
            //     //         x: i32,
            //     //         y: i32,
            //     //     }

            //     //     impl Display for Point {
            //     //         fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            //     //             write!(f, "x is {}, y is {}", self.x, self.y)
            //     //         }
            //     //     }

            //     //     let p1 = Point { x: 1, y: 2 };

            //     //     println!("{}", p1);
            //     // }
            //     // 特征对象
            //     // {
            //     //     trait Action {
            //     //         fn action1(&self);
            //     //     }

            //     //     struct User;

            //     //     impl Action for User {
            //     //         fn action1(&self) {
            //     //             println!("user");
            //     //         }
            //     //     }

            //     //     struct Animal;

            //     //     impl Action for Animal {
            //     //         fn action1(&self) {
            //     //             println!("animal")
            //     //         }
            //     //     }

            //     //     // 正如之前说过 返回值里只能返回一种特征对象 这里返回了两个 所以报错了
            //     //     // fn return_trait() -> impl Action {
            //     //     //     let is_user = false;

            //     //     //     if is_user {
            //     //     //         User
            //     //     //     } else {
            //     //     //         Animal
            //     //     //     }
            //     //     // }
            //     //     // 通过 & 引用或者 Box<T> 只能指针创建特征对象来解决这个问题
            //     //     // 一般像泛型等这类在编译期确定的属于静态分发
            //     //     // 这里的 dyn 的意思是动态分发 直到程序运行期才直到类型是什么
            //     //     fn return_trait() -> Box<dyn Action> {
            //     //         let is_user = true;

            //     //         if is_user {
            //     //             Box::new(User)
            //     //         } else {
            //     //             Box::new(Animal)
            //     //         }
            //     //     }

            //     //     let v = return_trait();

            //     //     v.action1();
            //     // }
            //     // self 和 Self
            //     // 不是所有的特征都可以有特征对象
            //     // {
            //     //     // 这个特征就不能有特征对象 原因是他的 action1 方法的放回类型是 Self
            //     //     // 还有一种是方法中含有泛型参数的也不可以作为特征对象
            //     //     trait Action {
            //     //         fn action1(&self) -> Self;
            //     //     }

            //     //     struct User;

            //     //     impl Action for User {
            //     //         // self 指 User 实例化后的对象
            //     //         // Self 指 User 类型
            //     //         fn action1(&self) -> Self {
            //     //             User
            //     //         }
            //     //     }

            //     //     // 报错
            //     //     fn return_trait() -> Box<dyn Action> {
            //     //         //
            //     //     }
            //     // }
            //     // 关联类型
            //     // 如果类型较多的话 比泛型更美观 更易懂
            //     // {
            //     //     trait Action {
            //     //         type Val; // 特征里定义一个关联类型

            //     //         fn action1(&self, val: Self::Val);
            //     //     }

            //     //     struct User;

            //     //     impl Action for User {
            //     //         // 这里定义真正的类型 类型为 i32
            //     //         type Val = i32;

            //     //         fn action1(&self, val: Self::Val) {
            //     //             println!("{:?}", val);
            //     //         }
            //     //     }

            //     //     let user1 = User;
            //     //     user1.action1(3);
            //     // }
            //     // 泛型默认类型
            //     // {
            //     //     // 这里给了泛型 T 默认的类型
            //     //     trait Action<T = Self> {
            //     //         fn action1(&self, val: T);
            //     //     }

            //     //     struct User;

            //     //     // 这里又重新指定了泛型 T 的类型为 i32
            //     //     impl Action<i32> for User {
            //     //         // val 的类型和新指定的泛型 T 的类型保持一致
            //     //         fn action1(&self, val: i32) {
            //     //             println!("{}", val);
            //     //         }
            //     //     }

            //     //     let user1 = User;
            //     //     user1.action1(3);
            //     // }
            //     // 调用同名方法
            //     // {
            //     //     trait Swim {
            //     //         fn action(&self);
            //     //         fn action2();
            //     //     }

            //     //     trait Jump {
            //     //         fn action(&self);
            //     //         fn action2();
            //     //     }

            //     //     struct User;

            //     //     impl Swim for User {
            //     //         fn action(&self) {
            //     //             println!("Swim");
            //     //         }
            //     //         fn action2() {
            //     //             println!("Swim");
            //     //         }
            //     //     }

            //     //     impl Jump for User {
            //     //         fn action(&self) {
            //     //             println!("Jump");
            //     //         }
            //     //         fn action2() {
            //     //             println!("Jump");
            //     //         }
            //     //     }

            //     //     impl User {
            //     //         fn action(&self) {
            //     //             println!("User");
            //     //         }
            //     //         fn action2() {
            //     //             println!("User");
            //     //         }
            //     //     }

            //     //     let user1 = User;
            //     //     // 调用方法
            //     //     user1.action(); // 默认调用的是该类型中定义的方法 也就是 User
            //     //     Swim::action(&user1); // 使用特征下的函数调用
            //     //     Jump::action(&user1);
            //     //     // 调用关联函数
            //     //     User::action2();
            //     //     <User as Swim>::action2(); // 使用 as 限定语法明确 User 是哪个特征
            //     //     <User as Jump>::action2();
            //     // }
            //     // 特征约束
            //     // {
            //     //     trait Target {
            //     //         fn must();
            //     //     }

            //     //     // 约束实现 Action 特征的类型要先具备 Target 的特征
            //     //     trait Action: Target {
            //     //         fn action1();
            //     //     }

            //     //     struct User;

            //     //     // 必须先为 User 实现 Target 特征
            //     //     impl Target for User {
            //     //         fn must() {
            //     //             println!("User must");
            //     //         }
            //     //     }

            //     //     impl Action for User {
            //     //         fn action1() {
            //     //             println!("User action");
            //     //         }
            //     //     }
            //     // }
            //     // 在外部特征上实现外部特征
            //     // 因为上面提到过 如果想要实现特征 那么特征和类型其中一个必须在当前作用域内 如果为存在于标准库中的类型实现标准库中的特征 那么上面的方法是不行的
            //     // {
            //     //     use std::fmt;

            //     //     // 使用元祖结构体 定义一个 new type
            //     //     struct Wrapper(Vec<String>);

            //     //     // 这样就可以为 Vec 类型实现 Display 特征了
            //     //     impl fmt::Display for Wrapper {
            //     //         fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            //     //             write!(f, "[{}]", self.0.join(", "))
            //     //         }
            //     //     }

            //     //     // 通过 Deref 特征可以做一层类型转换 可以不必使用元祖.操作去获取元素, 还可以重载实现特征的类型的方法
            //     // }
            // }
            // 合集
            {
                // 动态数组
                // {
                //     // 创建动态数组
                //     // let mut arr1 = Vec::new();
                //     // arr1.push(1); // 更新数组 如果 Vec 没有显示的指定类型 通过 push 编译期推断出 Vec 的类型是 i32

                //     // println!("{:?}", arr1);

                //     // 使用宏创建时既可初始化
                //     let arr2 = vec![1, 2, 3];

                //     println!("{:?}", arr2);
                //     println!("{}", arr2[1]); // 通过下标取值

                //     // 使用 .get 取值 得到的是一个 Option 枚举 须要通过匹配处理值
                //     // 与下标直接取值不同的是 如果发生取值越界 .get 不会报错 下标取值就会报错
                //     let val = match arr2.get(1) {
                //         Some(v) => v,
                //         None => &0,
                //     };

                //     println!("{}", val);

                //     // 第三种初始化
                //     let arr3 = vec![0; 3]; // 和静态数组一样 初始化 3 个 0
                //     println!("{:?}", arr3);

                //     // 第四种
                //     let arr4 = Vec::from([1, 2, 3]);
                //     println!("{:?}", arr4);
                // }
                // 借用多个数组元素
                // {
                //     let mut arr1 = vec![1, 2, 3];

                //     let val1 = &arr1[0];

                //     // 如果 val1 在 push 之后使用了 那么编译会报错 因为如果 push 了之后 数组的大小变了 这里是变大 当旧数组大小不够的时候 rust 会分配一块更大的内存 (2 倍大小) 那么原来的引用会指向一块无效的内存 应该不要发生这种事情 所以编译不通过
                //     arr1.push(4);

                //     println!("{val1}");
                // }
                // 迭代
                // {
                //     let mut arr1 = vec![1, 2, 3];

                //     // for num in arr1 {
                //     //     println!("{num}");
                //     // }
                //     // 可以修改元素
                //     for num in &mut arr1 {
                //         *num += 1;
                //         println!("{num}");
                //     }
                // }
                // 存储不同类型
                // 数组默认存储的都必须是相同类型的元素
                // 可以通过使用枚举和特征对象来实现存储不同类型
                // {
                //     // 通过枚举
                //     // {
                //     //     #[derive(Debug)]
                //     //     enum Store {
                //     //         String(String),
                //     //         Number(i32),
                //     //     }

                //     //     // 存储了两种不同的类型 都是属于 Store 枚举的成员
                //     //     let arr1 = vec![Store::String("Hello".to_string()), Store::Number(1)];

                //     //     println!("{:?}", arr1);

                //     //     for t in arr1 {
                //     //         println!("{:?}", t);
                //     //     }
                //     // }
                //     // 通过特征对象
                //     // {
                //     //     trait Action {
                //     //         fn say(&self);
                //     //     }

                //     //     struct User;
                //     //     impl Action for User {
                //     //         fn say(&self) {
                //     //             println!("ha");
                //     //         }
                //     //     }

                //     //     struct Animal;
                //     //     impl Action for Animal {
                //     //         fn say(&self) {
                //     //             println!("wa");
                //     //         }
                //     //     }

                //     //     // 给 arr 显示指定数组元素类型为实现了 Action 特征的对象
                //     //     let arr1: Vec<Box<dyn Action>> = vec![Box::new(User), Box::new(Animal)];

                //     //     for t in arr1 {
                //     //         t.say();
                //     //     }
                //     // }
                // }
                // 初始化数组时指定容量
                // {
                //     let mut v = Vec::with_capacity(5);
                //     v.extend([1, 2, 3]);

                //     println!("{}, {}", v.len(), v.capacity());

                //     v.reserve(1); // 调整容量 在之前的基础上增加至少 xx 的容量
                //     println!("{}, {}", v.len(), v.capacity());
                // }
                // 更多数组方法查阅文档
                // 数组排序分为稳定和非稳定 (对于相等的元素 稳定不会重排 非稳定不保证这一点)
                // 可以通过 derive 派生排序相关的特性给结构体 那么结构体也可以排序 前提是结构体的属性都必须实现了 Ord 特征
            }
        }
    }
}
