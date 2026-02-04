// use num::complex::Complex;
// use std::convert::TryInto;
// use std::prelude;

// use core::panic;
// use std::{
//     ffi::{c_long, os_str::Display},
//     fmt::{self, write, Debug, Formatter},
//     io::{self, Read},
//     str::FromStr,
// };

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
    //     // () 零长度元祖就是一个单元类型，比如函数不显示返回任何值 / 类型 那么它默认就返回单元类型
    // }
    // 用不返回的发散函数
    // {
    //     // 使用！标识返回类型
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
    //     assert_eq!(5, *b); // 解引用之后才能访问 b 所指向的值 然后进行比较。不解引用的话 b 是个引用 不能与整型进行比较
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
        //         Input (u8), // 类型也可以省略
        //         Checkbox { a: char },
        //         Radio(bool),
        //     }

        //     let input = Type::Input(1);
        //     let checkbox = Type::Checkbox { a: 'a' };
        //     let radio = Type::Radio(true);

        //     println!("{:?},{:?},{:?}", input, checkbox, radio);
        // }
        // Option 枚举，用来表示当前变量的值是缺失的，用来代替 null
        // {
        //     // 泛型 T
        //     enum Option<T> {
        //         Some (T), // 含有值
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
        //         //     //println!("{}", arr1 [4]); // 不可以跨界访问数组元素
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
        //     //     //     //match 本身也是表达式 所以他的返回值可以赋值给其他变量 这里返回了一个单元类型
        //     //     //     let res = match input {
        //     //     //         Type::Button => println!("Button"),
        //     //     //         //xxx 或 xxx
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
        //     //if let 只匹配一个条件时使用
        //     // {
        //     //     let v = Some(3);

        //     //     if let Some(3) = v {
        //     //         println!("3");
        //     //     }

        //     //     // 这里 x 为变量 (? 可以理解为形参吗)
        //     //     if let Some(x) = v {
        //     //         println!("3, {x}");
        //     //     }

        //     //     let y = 4;
        //     //     if let Some(y) = v {
        //     //         println!("3, {y}");
        //     //     }
        //     // }
        //     //matches! 宏 直接返回 true 或 false
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

        //     //     //stack.pop 从数组尾部弹出元素
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
        //     //         Point {x: 1, y} => println!("matched x = 1, y is {y}"), // 只匹配 x
        //     //         Point {x: _, y: 2} => println!("matched y = 2"),        // 只匹配 y
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
        //     // 使用下划线_加字母的变量 仍会被绑定值，只使用下划线_则不会被绑定值 比如使用_绑定一个值的所有权 则原来的变量并不会丢失所有权.
        //     // 匹配中的额外条件
        //     // {
        //     //     let num = Some(5);

        //     //     match num {
        //     //         Some (x) if x > 4 => println!("x > 4"), // 模式中的额外 if 条件 更进一步匹配
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
        //     //     //         Point {x: x_var @ _, ..} => println!("{}", x_var), // 将 x 绑定到新变量 x_var 上
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
        // // let-else
        // {
        //     use std::str::FromStr;

        //     fn get_count_item(s: &str) -> (u64, &str) {
        //         let mut it = s.split(' ');

        //         let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
        //             panic!("Can not segment str {s}");
        //         };

        //         let Ok(count) = u64::from_str(count_str) else {
        //             panic!("Can not parse count str {count_str}");
        //         };

        //         (count, item)
        //     }

        //     let (count, item) = get_count_item("123 chairs");
        //     println!("{count}, {item}");
        // }
        // method
        //rust 中的方法往往跟 struct, enum, trait 一起使用
        // {
        //     // 简单的方法
        //     struct Point {
        //         x: i32,
        //         y: i32,
        //     }

        //     //impl 关键字定义实现方法 并与 Point 相关联
        //     impl Point {
        //         //self 是 Point 本身，可以当作 js 中的 this 来理解
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
        //         // 通过：：来调用
        //         pub fn new(x: i32, y: i32) -> Self {
        //             Point { x, y }
        //         }

        //         pub fn test() {
        //             println!("test")
        //         }

        //         // 如果方法名和属性名同名 则一般用来实现 getter 访问器
        //         // 只将 getter 变为公有的 则外部就可以访问 并不可以进行修改 如果直接访问 Point.x 则会报错
        //         // 方法可以通过::来调用 也可以实例化过后通过。来调用
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
        // 多个 impl 块
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
        // 泛型 类似 ts 中泛型，特征 类似 interface
        {
            // {
            //     fn add<T: std::ops::Add<Output = T>>(x: T, y: T) -> T {
            //         x + y
            //     }

            //     println!("{}", add(1, 2));

            //     enum Enum1<T, U> {
            //         A(T),
            //         B(U),
            //     }

            //     struct Struct1<T> {
            //         a: T,
            //     }

            //     // 方法中的泛型 此时要把 Struct1<T> 看作是一个整体, 而不是仅仅只是 Struct1
            //     impl<T> Struct1<T> {
            //         fn get_a<K: std::fmt::Display>(&self, other: K) -> &T {
            //             println!("{}", other);
            //             &self.a
            //         }
            //     }

            //     let s = Struct1 { a: 1 };
            //     println!("{}", s.get_a(2));

            // 上面说的, 要把结构体加泛型看作一个整体, 这个例子就说明了, 只为 Struct1<i32> 的结构体实现了 x 方法
            // impl Struct1<i32> {
            //     fn x(&self) -> bool {
            //         true
            //     }
            // }

            // let user1 = Struct1 { a: "John" };
            // // user1.x(); // user1 上没有 x 方法
            // let user2 = Struct1 { a: 1 };
            // user2.x(); // user2 上有

            //     //const 针对值的泛型
            //     // 这里的 N 是值的泛型 用来代替数组的长度 值的类型是 usize
            //     fn display_arr<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
            //         println!("{:?}", arr)
            //     }
            //     let arr1: [i32; 3] = [1, 2, 3];
            //     let arr2: [i32; 2] = [1, 2];
            //     display_arr(arr1);
            //     display_arr(arr2);

            //     // const fn (编译期对函数进行求值)
            //     const fn const_add(x: i32, y: i32) -> i32 {
            //         x + y
            //     }
            //     println!("const_add {}", const_add(1, 2));

            //     //const 泛型表达式 where 在下面特征中有说明
            // }

            // trait 特征
            // 可以理解为定义特征就是 interface 接口 实现特征为继承接口
            {
                // {
                //     trait Action {
                //         fn barking(&self) -> &String;
                //     }

                //     struct Dog {
                //         voice: String,
                //     }

                //     // 实现 特征 为 xxx
                //     impl Action for Dog {
                //         fn barking(&self) -> &String {
                //             &self.voice
                //         }
                //     }

                //     struct Cat {
                //         voice: String,
                //     }

                //     impl Action for Cat {
                //         fn barking(&self) -> &String {
                //             &self.voice
                //         }
                //     }

                //     let dog = Dog {
                //         voice: "wang".to_string(),
                //     };
                //     let cat = Cat {
                //         voice: "miao".to_string(),
                //     };

                //     println!("{:?}, {:?}", dog.barking(), cat.barking());
                // }

                // // 为 A 实现 B 特征，则他俩至少有一个是在当前作用域当中
                // // 你可以为标准库中的类型实现你自定义的特征 也可以为你自定义的类型实现标准库中的特征 但是不可以为标准库中的类型实现标准库中的特征 因为他们都没在当前的作用域内
                // // 特征中的默认实现方法和重载特征中的方法
                // {
                //     trait Action {
                //         // 默认实现
                //         // 注意 想在实例化后通过 . 访问 就必须加上 &self 把函数作为方法
                //         fn barking(&self) {
                //             println!("HaHa");
                //         }
                //     }

                //     struct Dog {
                //         voice: String,
                //     }

                //     struct Cat {
                //         voice: String,
                //     }

                //     impl Action for Dog {}

                //     impl Action for Cat {
                //         fn barking(&self) {
                //             println!("mao");
                //         }
                //     }

                //     let dog = Dog {
                //         voice: "wang".to_string(),
                //     };
                //     let cat = Cat {
                //         voice: "mao".to_string(),
                //     };

                //     dog.barking();
                //     cat.barking();
                // }

                // // 特征定默认实现里可以调用具有相同特征中的其他方法
                // {
                //     trait Action {
                //         fn action1(&self);
                //         fn action2(&self) {
                //             self.action1(); // 调用自身的 action1
                //         }
                //     }

                //     struct User;

                //     impl Action for User {
                //         fn action1(&self) {
                //             println!("action 1 is called");
                //         }
                //     }

                //     let user = User {};

                //     user.action2();
                // }

                // // 把特征当作函数参数传递 + 特征约束
                // // 特征约束可以理解为 Typescript 中的泛型 + extends
                // {
                //     trait Action {
                //         fn action1(&self);
                //     }

                //     struct User;

                //     impl Action for User {
                //         fn action1(&self) {
                //             println!("action 1 is called");
                //         }
                //     }

                //     // 此处定义的参数 action 的类型是必须实现了 Action 特征的
                //     // 这种书写形势只是一个语法糖
                //     // fn doit(action: &impl Action) {
                //     //     action.action1()
                //     // }
                //     // 这是完整的书写形式 称为特征约束 用来约束泛型 T 的类型是必须实现了 Action 特征的
                //     fn doit<T: Action>(action: &T) {
                //         action.action1();
                //     }

                //     let user = User;

                //     doit(&user);

                //     // 双重约束
                //     trait Fade {
                //         fn show(&self);
                //     }

                //     // 再次为 user 实现 Fade 特征
                //     impl Fade for User {
                //         fn show(&self) {
                //             println!("user has show fn");
                //         }
                //     }

                //     // 参数 p 必须是实现了 Action 和 Fade 特征的
                //     // fn multi(p: &(impl Action + Fade)) {
                //     //     p.action1();
                //     //     p.show();
                //     // }
                //     fn multi<T: Action + Fade>(p: &T) {
                //         p.action1();
                //         p.show();
                //     }

                //     multi(&user);

                //     //where 约束
                //     // 只是将泛型中的特征约束提取到 where 里
                //     {
                //         fn multi2<T, U>(p1: &T, p2: U)
                //         where
                //             T: Action + Fade,
                //             U: Fade,
                //         {
                //             p1.action1();
                //             p1.show();
                //         }
                //     }
                // }

                // // 使用特征约束有条件的实现方法
                // {
                //     use std::fmt::Display;

                //     struct Pair<T> {
                //         x: T,
                //         y: T,
                //     }

                //     impl<T> Pair<T> {
                //         fn new(x: T, y: T) -> Self {
                //             Self { x, y }
                //         }
                //     }

                //     impl<T: Display + PartialOrd> Pair<T> {
                //         fn cmp_display(&self) {
                //             // 调用这个方法只有实现了 Display + PartialOrd 特征的值才可以
                //             if self.x >= self.y {
                //                 println!("The largest member is x = {}", self.x);
                //             } else {
                //                 println!("The largest member is y = {}", self.y);
                //             }
                //         }
                //     }

                //     enum A {
                //         A,
                //     }

                //     let pair = Pair {
                //         x: A::A, // 用了不被约束的特征的值 在调用相关方法会报错
                //         y: A::A,
                //         // x: String::from("value"),
                //         // y: String::from("value"),
                //     };

                //     pair.cmp_display();
                // }

                // // 函数返回中的特征
                // {
                //     trait Action {
                //         fn action1(&self);
                //     }

                //     struct User;

                //     impl Action for User {
                //         fn action1(&self) {
                //             println!("action 1 is called");
                //         }
                //     }

                //     // 函数返回值是一个实现了 Action 特征的就可以
                //     // 只能返回一个具体的类型 例如:如果也为结构体 Animal 实现了 Action 特征
                //     // 那么这个函数的返回就必须只存在这两个中 (User, Animal) 的一个类型的可能
                //     fn return_trait() -> impl Action {
                //         User
                //     }
                // }

                // // 可以通过 derive 为某个类型派生特定的特征，这样这个类型就可以使用派生出来的方法等
                // // 如果想使用特征中的方法 可以将这个特征引入到当前作用域中
                // {
                //     use std::convert::TryInto; // 这个实际在最顶层引入 那样就不会报未使用的警告了

                //     let a: i32 = 10;
                //     let b: u16 = 100;

                //     let b_ = b.try_into().unwrap();

                //     if a < b_ {
                //         println!("Ten is less than one hundred.");
                //     }
                // }

                // // 为自定义类型实现 Add
                // {
                //     // 引入 Add 特征
                //     use std::ops::Add;

                //     // 限制泛型 T 必须是具备 Add 特征的
                //     #[derive(Debug)]
                //     struct Point<T: Add<T, Output = T>> {
                //         x: T,
                //         y: T,
                //     }

                //     // 为 Point 结构体实现 Add 特征 这样就可以进行两个 Point 结构体相加了
                //     impl<T: Add<T, Output = T>> Add for Point<T> {
                //         type Output = Point<T>;

                //         // 这里是具体的相加实现 想通过 Point + Point 实现什么效果，这里是把属性值 x 和 y 分别进行相加操作
                //         fn add(self, point: Point<T>) -> Point<T> {
                //             Point {
                //                 x: self.x + point.x,
                //                 y: self.y + point.y,
                //             }
                //         }
                //     }

                //     let p1 = Point { x: 1, y: 2 };
                //     let p2 = Point { x: 3, y: 4 };

                //     println!("{:?}", p1 + p2)
                // }

                // // 为自定义类型实现格式化输出
                // {
                //     // 引入特征
                //     use std::fmt::Display;

                //     struct Point {
                //         x: i32,
                //         y: i32,
                //     }

                //     impl Display for Point {
                //         fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                //             write!(f, "x is {}, y is {}", self.x, self.y)
                //         }
                //     }

                //     let p1 = Point { x: 1, y: 2 };

                //     println!("{}", p1);
                // }

                // // 为自定义类型实现自定义格式化输入 2
                // {
                //     use std::fmt::{Display, Result};

                //     #[derive(Debug, PartialEq)]
                //     enum FileState {
                //         Open,
                //         Closed,
                //     }

                //     #[derive(Debug)]
                //     struct File {
                //         name: String,
                //         data: Vec<u8>,
                //         state: FileState,
                //     }

                //     impl Display for FileState {
                //         fn fmt(&self, f: &mut Formatter) -> Result {
                //             match *self {
                //                 FileState::Open => write!(f, "OPEN"),
                //                 FileState::Closed => write!(f, "CLOSED"),
                //             }
                //         }
                //     }

                //     impl Display for File {
                //         fn fmt(&self, f: &mut Formatter) -> Result {
                //             write!(f, "<{},({})>", self.name, self.state)
                //         }
                //     }

                //     impl File {
                //         fn new(name: &str) -> File {
                //             File {
                //                 name: String::from(name),
                //                 data: Vec::new(),
                //                 state: FileState::Closed,
                //             }
                //         }
                //     }

                //     let f = File::new("f.txt");
                //     println!("{:?}", f);
                //     println!("{}", f);
                //     print!("{}", FileState::Closed);
                //     print!("{}", FileState::Open);
                // }

                // // 特征对象
                // {
                //     trait Action {
                //         fn action1(&self);
                //     }

                //     struct User;

                //     impl Action for User {
                //         fn action1(&self) {
                //             println!("user");
                //         }
                //     }

                //     struct Animal;

                //     impl Action for Animal {
                //         fn action1(&self) {
                //             println!("animal")
                //         }
                //     }

                //     // 正如之前说过 返回值里只能返回一种特征对象 这里返回了两个 所以报错了
                //     // fn return_trait() -> impl Action {
                //     //     let is_user = false;

                //     //     if is_user {
                //     //         User
                //     //     } else {
                //     //         Animal
                //     //     }
                //     // }
                //     // 通过 & 引用或者 Box<T> 智能指针创建特征对象来解决这个问题
                //     // 一般像泛型等这类在编译期确定的属于静态分发
                //     // 这里的 dyn 的意思是动态分发 直到程序运行期才直到类型是什么
                //     fn return_trait() -> Box<dyn Action> {
                //         let is_user = true;

                //         if is_user {
                //             Box::new(User) // 这种 Box::new(Xxx) 调用方式对应的类型声明是 Box<dyn Xxx>
                //         } else {
                //             Box::new(Animal)
                //         }
                //     }

                //     // fn return_trait2() -> &dyn Action { // error, 与生命周期有关
                //     //     let is_user = true;

                //     //     if is_user {
                //     //         &User
                //     //     } else {
                //     //         &Animal
                //     //     }
                //     // }

                //     fn call_action1(x: &dyn Action) {
                //         x.action1();
                //     }

                //     let v = return_trait();
                //     v.action1();

                //     call_action1(&Animal); // 这种 &Xxx 调用方式对应的类型声明是 &dyn Xxx
                // }

                // // self 和 Self
                // // 不是所有的特征都可以有特征对象
                // {
                //     // 这个特征就不能有特征对象 原因是他的 action1 方法的返回类型是 Self
                //     // 还有一种是方法中含有泛型参数的也不可以作为特征对象
                //     trait Action {
                //         fn action1(&self) -> Self;
                //     }

                //     #[derive(Debug)]
                //     struct User;

                //     impl Action for User {
                //         //self 指 User 实例化后的对象
                //         // Self 指 User 类型
                //         fn action1(&self) -> Self {
                //             User
                //         }
                //     }

                //     let user = User;
                //     println!("{:?}", user.action1());

                //     // 报错
                //     // fn return_trait() -> Box<dyn Action> {
                //     //     //
                //     // }
                // }

                // // 关联类型
                // // 如果类型较多的话 比泛型更美观 更易懂
                // {
                //     trait Action {
                //         type Val; // 特征里定义一个关联类型

                //         fn action1(&self, val: Self::Val);
                //     }

                //     struct User;

                //     impl Action for User {
                //         // 这里定义真正的类型 类型为 i32
                //         type Val = i32;

                //         fn action1(&self, val: Self::Val) {
                //             println!("{:?}", val);
                //         }
                //     }

                //     let user1 = User;
                //     user1.action1(3);
                // }

                // // 泛型默认类型
                // {
                //     // 这里给了泛型 T 默认的类型
                //     trait Action<T = Self> {
                //         fn action1(&self, val: T);
                //     }

                //     struct User;

                //     // 这里又重新指定了泛型 T 的类型为 i32
                //     impl Action<i32> for User {
                //         // val 的类型和新指定的泛型 T 的类型保持一致
                //         fn action1(&self, val: i32) {
                //             println!("{}", val);
                //         }
                //     }

                //     let user1 = User;
                //     user1.action1(3);
                // }

                // // 调用同名方法
                // {
                //     trait Swim {
                //         fn action(&self);
                //         fn action2();
                //     }

                //     trait Jump {
                //         fn action(&self);
                //         fn action2();
                //     }

                //     struct User;

                //     impl Swim for User {
                //         fn action(&self) {
                //             println!("Swim");
                //         }
                //         fn action2() {
                //             println!("Swim");
                //         }
                //     }

                //     impl Jump for User {
                //         fn action(&self) {
                //             println!("Jump");
                //         }
                //         fn action2() {
                //             println!("Jump");
                //         }
                //     }

                //     impl User {
                //         fn action(&self) {
                //             println!("User");
                //         }
                //         fn action2() {
                //             println!("User");
                //         }
                //     }

                //     let user1 = User;
                //     // 调用方法
                //     user1.action(); // 默认调用的是该类型中定义的方法 也就是 User
                //     Swim::action(&user1); // 使用特征下的函数调用
                //     Jump::action(&user1);
                //     // 调用关联函数
                //     User::action2();
                //     <User as Swim>::action2(); // 使用 as 限定语法明确 User 是哪个特征
                //     <User as Jump>::action2();
                // }

                // // 特征约束
                // {
                //     trait Target {
                //         fn must();
                //     }

                //     // 约束实现 Action 特征的类型要先具备 Target 的特征
                //     trait Action: Target {
                //         fn action1();
                //     }

                //     struct User;

                //     // 必须先为 User 实现 Target 特征
                //     impl Target for User {
                //         fn must() {
                //             println!("User must");
                //         }
                //     }

                //     impl Action for User {
                //         fn action1() {
                //             println!("User action");
                //         }
                //     }
                // }

                // // 在外部特征上实现外部特征
                // // 因为上面提到过 如果想要实现特征 那么特征和类型其中一个必须在当前作用域内 如果为存在于标准库中的类型实现标准库中的特征 那么上面的方法是不行的
                // {
                //     use std::fmt;

                //     // 使用元祖结构体 定义一个 new type
                //     struct Wrapper(Vec<String>);

                //     // 这样就可以为 Vec 类型实现 Display 特征了
                //     impl fmt::Display for Wrapper {
                //         fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                //             write!(f, "[{}]", self.0.join(", "))
                //         }
                //     }

                //     // 通过 Deref 特征可以做一层类型转换 可以不必使用元祖.操作去获取元素，还可以重载实现特征的类型的方法
                // }
            }
        }
        // 集合
        {
            // 动态数组
            {
                // {
                //     // 创建动态数组
                //     // let mut arr1 = Vec::new();
                //     //arr1.push (1); // 更新数组 如果 Vec 没有显示的指定类型 通过 push 编译期推断出 Vec 的类型是 i32

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

                // // 借用多个数组元素
                // {
                //     let mut arr1 = vec![1, 2, 3];

                //     let val1 = &arr1[0];

                //     // 如果 val1 在 push 之后使用了(在 push 之前使用是没问题的) 那么编译会报错 因为如果 push 了之后 数组的大小变了 这里是变大 当旧数组大小不够的时候 rust 会分配一块更大的内存 (2 倍大小) 那么原来的引用会指向一块无效的内存 应该不要发生这种事情 所以编译不通过
                //     arr1.push(4);

                //     println!("{val1}");
                // }

                // // 迭代
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

                // // 存储不同类型
                // // 数组默认存储的都必须是相同类型的元素
                // // 可以通过使用枚举和特征对象来实现存储不同类型
                // {
                // // 通过枚举
                // {
                //     #[derive(Debug)]
                //     enum Store {
                //         String(String),
                //         Number(i32),
                //     }

                //     // 存储了两种不同的类型 都是属于 Store 枚举的成员
                //     let arr1 = vec![Store::String("Hello".to_string()), Store::Number(1)];

                //     println!("{:?}", arr1);

                //     for t in arr1 {
                //         println!("{:?}", t);
                //     }
                // }

                // // 通过特征对象
                // {
                //     trait Action {
                //         fn say(&self);
                //     }

                //     struct User;
                //     impl Action for User {
                //         fn say(&self) {
                //             println!("ha");
                //         }
                //     }

                //     struct Animal;
                //     impl Action for Animal {
                //         fn say(&self) {
                //             println!("wa");
                //         }
                //     }

                //     // 给 arr 显示指定数组元素类型为实现了 Action 特征的对象
                //     // let arr1: Vec<Box<dyn Action>> = vec![Box::new(User), Box::new(Animal)];
                //     // 也可以通过 &dyn Xxx 的方式定义
                //     let arr1: Vec<&dyn Action> = vec![&User, &Animal];

                //     for t in arr1 {
                //         t.say();
                //     }
                // }
            }

            // // 初始化数组时指定容量
            // {
            //     let mut v = Vec::with_capacity(5);
            //     v.extend([1, 2, 3]);

            //     println!("{}, {}", v.len(), v.capacity());

            //     v.reserve(1); // 调整容量 在之前的基础上增加至少 xx 的容量
            //     println!("{}, {}", v.len(), v.capacity());
            // }
            // 更多数组方法查阅文档

            // // 数组排序分为稳定和非稳定 (对于相等的元素 稳定不会重排 非稳定不保证这一点)
            // {
            //     let mut arr1 = vec![1, 5, 10, 2, 15];
            //     arr1.sort_unstable();
            //     println!("{:?}", arr1);

            //     let mut arr2 = vec![1.0, 5.6, 10.3, 2.0, 15f32];
            //     // arr2.sort_unstable(); // error, 浮点没有实现 Ord 特征, 只实现了 PartialOrd 特征
            //     arr2.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
            //     println!("{:?}", arr2);
            // }

            // // 对结构体数组进行排序 (根据结构体中的某个属性)
            // {
            //     #[derive(Debug)]
            //     struct User {
            //         name: String,
            //         age: u32,
            //     }

            //     impl User {
            //         fn new(name: String, age: u32) -> User {
            //             User { name, age }
            //         }
            //     }

            //     let mut users = vec![
            //         User::new("John".to_string(), 26),
            //         User::new("Bob".to_string(), 30),
            //         User::new("Alice".to_string(), 18),
            //     ];
            //     users.sort_unstable_by(|a, b| a.age.cmp(&b.age));
            //     println!("{:?}", users);

            // }

            // // 可以通过 derive 派生排序相关的特性给结构体 那么结构体也可以排序 前提是结构体的属性都必须实现了 Ord 特征
            // {
            //     // 此时要保证结构体中的所有属性都实现了 Ord 相关的特征, 不然会报错
            //     #[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
            //     struct User {
            //         name: String,
            //         age: u32,
            //     }

            //     impl User {
            //         fn new(name: String, age: u32) -> User {
            //             User { name, age }
            //         }
            //     }

            //     let mut users = vec![
            //         User::new("John".to_string(), 26),
            //         User::new("Bob".to_string(), 30),
            //         User::new("Alice".to_string(), 18),
            //     ];
            //     users.sort_unstable();
            //     println!("{:?}", users);
            // }

            // HashMap 哈希映射
            {
                // 没有在 rust 的 prelude 中 须要手动引入
                // use std::collections::HashMap;

                // // 创建
                // {
                //     let mut obj1 = HashMap::new(); // 通过 new 创建

                //     obj1.insert('a', 1); // 通过 insert 插件数据 编译器自动推断出类型

                //     for item in obj1 {
                //         println!("{:?}", item);
                //     }
                // }

                // // 将 vec 转为 hashmap
                // {
                //     let arr1 = vec![('a', 1), ('b', 2)];
                //     let obj1: HashMap<_, _> = arr1.into_iter().collect();

                //     for item in &obj1 {
                //         println!("{:?}", item);
                //     }

                //     println!("{:?}", obj1.get(&'a'));
                // }

                // // 所有权问题
                // {
                //     let name = String::from("Bob");
                //     let mut obj1 = HashMap::new();

                //     //obj1.insert (name, 1) // 这里把 name 的所有权转移
                //     //println!("{}", name); //name 的所有权丢失

                //     obj1.insert (&name, 1); // 这里获取了 name 的引用
                //     println!("{}", name); // 这里的 name 没有丢失

                //     // 必须确保 name 的生命周期和 hashmap 一样长
                //     // 如果手动移除 name 则再次使用 obj1 就会报错
                //     // std::mem::drop(name);

                //     println!("{:?}", obj1);
                // }

                // // 查询 更新
                // {
                //     let mut obj1 = HashMap::new();

                //     obj1.insert('a', 1);

                //     println!("{:?}", obj1.get(&'a')); // 通过 .get 取值 得到的是一个 Option 枚举

                //     // 循环取值
                //     println!("-- Get value in the loop -------------------");
                //     for (k, v) in &obj1 {
                //         println!("{}, {}", k, v);
                //     }

                //     // 直接取值
                //     println!("-- Get value use copied and unwrap -------------------");
                //     let v: i32 = obj1.get(&'a').copied().unwrap_or(0);
                //     println!("{}", v);

                //     println!("-- Update a ----------------------");
                //     // 更新 返回老值
                //     let old_val = obj1.insert('a', 2);

                //     println!("old a's value is {:?}", old_val); // Some(1)
                //     println!("{:?}", obj1);

                //     println!("-- Search and update a ----------------------");
                //     // 未查询到->更新 查询到->不做更新操作
                //     let v1 = obj1.entry('a').or_insert(3);

                //     println!("v1 is {:?}", v1);
                //     println!("{:?}", obj1);

                //     println!("-- Search and update b ----------------------");
                //     let v2 = obj1.entry('b').or_insert(4);

                //     println!("v2 is {:?}", v2);
                //     // println!("{:?}", obj1);

                //     println!("-- Modify v2's reference directly ----------------------");
                //     *v2 = 5; //.or_insert 返回的是一个可变的引用 所以可以通过直接修改它来达到修改 hashmap 中的此值

                //     println!("v2 is {:?}", v2); // 5

                //     println!("{:?}", obj1); // 此时 'b': 5
                // }
            }
        }

        // 生命周期
        {
            // // 悬垂指针
            // {
            //     {
            //         let a;

            //         {
            //             let b = 1;
            //             // a = b; // 这里只是 copy 没有问题
            //             a = &b; // 如果这里是个引用 则会出现问题
            //                     // b 在这个小的作用域离开后被释放 所以 a 引用了一个无效的引用
            //         }

            //         println!("{}", a);
            //     }
            //     {
            //         // 如果作用域一样大就没有问题
            //         let a;
            //         let b = 1;
            //         a = &b;

            //         println!("{}", a);
            //     }
            // }

            // // 函数中的生命周期
            // {
            //     // let str1 = String::from("string1");
            //     // let str2 = "string222";

            //     // let res1 = longest(str1.as_str(), str2);
            //     // println!("{}", res1);

            //     // 报错 因为函数不知道返回的是 a 还是 b 的引用
            //     // 无法得知 a 和 b 的生命周期
            //     // 无法知道返回值的生命周期和 a 和 b 的生命周期的关系
            //     // 当存在多个引用时 编译器有时候也无法推导出生命周期
            //     // fn longest(a: &str, b: &str) -> &str {
            //     //     if a.len() > b.len() {
            //     //         a
            //     //     } else {
            //     //         b
            //     //     }
            //     // }

            //     // 生命周期标注
            //     // 'a 标注 a 可以是任何字符 一般用 a
            //     // 生命周期标注在引用符号后面 可变关键字前面 &'a mut &'a
            //     // 只是告诉编译器多个引用之间的关系 并不会改变引种真实的生命周期
            //     // 这里标注了参数 a 和 b 和返回值都和函数 longest 活的一样久 这样编译器就不会报错了
            //     // 实际上返回值的生命周期与两个参数生命周期较小的那个一致
            //     fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
            //         if a.len() > b.len() {
            //             a
            //         } else {
            //             b
            //         }
            //     }

            //     // {
            //     //     let str3 = String::from("string3");
            //     //     {
            //     //         let str4 = String::from("string444");
            //     //         let res2 = longest(str3.as_str(), str4.as_str()); // 返回值的生命周期和 str4 保持一致

            //     //         println!("{}", res2);
            //     //     }
            //     // }

            //     // {
            //     //     let str5 = String::from("string5");
            //     //     let res;
            //     //     {
            //     //         let str6 = String::from("string666");
            //     //         res = longest(str5.as_str(), str6.as_str()); // 编译器报错 str6 的生命周期比 str5 短 (str6 活的不够长)

            //     //         println!("{}", res); // 这里是有效的
            //     //                              // 当离开 str6 的作用域 res 也被释放
            //     //     }

            //     //     // 报错 无法访问被释放的引用
            //     //     println!("{}", res);
            //     // }
            // }

            // // 结构体中的生命周期
            // {
            //     #[derive(Debug)]
            //     struct User<'a> {
            //         name: &'a str,
            //         // name: &str,
            //     }

            //     #[derive(Debug)]
            //     struct User2 {
            //         name: i32,
            //     }

            //     let str1 = String::from("string1");
            //     // 没问题 str1 的生命周期和 user1 的生命周期相等
            //     let user1 = User {
            //         name: str1.as_str(),
            //     };

            //     println!("{:?}", user1);

            //     let user2;
            //     let user3;
            //     {
            //         let str2 = String::from("string2");
            //         user2 = User {
            //             name: str2.as_str(),
            //         };
            //         user3 = User2 { name: 1 };

            //         println!("{:?}", user2); // 同样这里是有效的
            //     }

            //     println!("{:?}", user2); // 这里报错 name 也就是 str2 在其内部作用域完成后被释放 那么 user2 内部存在一个无效引用
            //     println!("{:?}", user3);
            // }

            // 生命周期消除满足三个条件
            // 1. 为每个引用参数都获得独自的标注
            // 2. 当只有一个输入生命周期 (一个引用参数) 的时候 那么返回值的生命周期都等于该输入生命周期
            // 3. 当有多个输入生命周期 且有一个是 self, 则返回值的生命周期等于 self 生命周期

            // // 方法中的生命周期
            // {
            //     struct User<'a> {
            //         name: &'a str,
            //     }

            //     //impl 必须使用结构体完整名称 包括生命周期标注
            //     // 这里根据生命周期消除三个条件 编译器会自动推断方法中的生命周期
            //     // impl<'a> User<'a> {
            //     //     fn return_name(&self, other: &str) -> &str {
            //     //         println!("{}", other);
            //     //         self.name
            //     //     }
            //     // }
            //     // 完整的写法
            //     // 满足生命周期消除条件一和条件三
            //     // impl<'a> User<'a> {
            //     //     fn return_name<'b>(&'a self, other: &'b str) -> &'a str {
            //     //         println!("{}", other);
            //     //         self.name
            //     //     }
            //     // }
            //     // 手动指定不同的返回类型生命周期 'a -> 'b
            //     // 如果返回的声明周期改为 'b, 那么一定要确定 'a 和 'b 的关系
            //     // 因为这时候 User 的 生命周期已经被定义为 'a 了, 那么他的方法的参数的生命周期一定要小于等于 'a
            //     // 否则如果 User 销毁了, 但方法的参数还存在一个活着的引用, 这会形成悬垂引用
            //     // 那么就要约束 'b 的生命周期一定比 'a 短 否则就会出现悬垂引用 通过 'a: 'b 或通过 where 约束 和泛型一样
            //     // impl<'a: 'b, 'b> User<'a> {
            //     //     fn return_name(&'a self, other: &'b str) -> &'b str {
            //     //         println!("{}", other);
            //     //         self.name
            //     //     }
            //     // }

            //     // 测试用的
            //     // impl<'a, 'b, 'c: 'b> User<'c> {
            //     //     fn return_name(&'a self, other: &'b str) -> &'b str {
            //     //         println!("{}", other);
            //     //         self.name
            //     //     }
            //     // }

            //     // 通过 where 约束
            //     impl<'a> User<'a> {
            //         fn return_name<'b>(&'a self, other: &'b str) -> &'b str
            //         where
            //             'a: 'b,
            //         {
            //             println!("{}", other);
            //             self.name
            //         }
            //     }

            //     let user = User { name: "John" };

            //     println!("{}", user.return_name("Hello"))
            // }

            // // 静态生命周期
            // {
            //     // 通过'static 标注
            //     // 表示这个生命周期活的和程序一样久 例如字符串字面量 (硬编码到二进制文件里)
            //     let name: &'static str = "John";
            //     println!("{}", name);
            // }

            // {
            //     use std::fmt::Display;

            //     fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
            //     where
            //         T: Display,
            //     {
            //         println!("Announcement! {}", ann);
            //         if x.len() > y.len() {
            //             x
            //         } else {
            //             y
            //         }
            //     }

            //     let str1 = String::from("value");
            //     let str2 = String::from("value11111");
            //     let ret = longest_with_an_announcement(&str1, &str2, "HaHaHaHaHaHa");
            //     println!("{}", ret);
            // }
        }

        // 错误处理
        {
            // 可恢复错误 Result<T, E> 用户访问，操作等错误，会影响某个用户自身的操作进程 不会影响全局或系统的错误
            // 不可恢复错误 panic
            {

                // {
                //     use std::fs::File;
                //     let f = File::open("hello.txt");

                //     match f {
                //         Ok(file) => file,
                //         Err(err) => {
                //             panic!("打开文件发生错误 {}", err)
                //         }
                //     };
                // }

                // // 处理不同类型的错误
                // {
                //     use std::io::ErrorKind;
                //     use std::fs::File;

                //     let file_name = "hello.txt";
                //     let f = File::open(file_name);

                //     let f = match f {
                //         Ok(file) => file,
                //         Err(err) => match err.kind() {
                //             ErrorKind::NotFound => match File::create(file_name) {
                //                 Ok(fc) => fc,
                //                 Err(e) => panic!("创建文件发生错误 {}", e),
                //             },
                //             _ => panic!("错误"),
                //         },
                //     };

                //     println!("{:?}", f);
                // }

                // // unwrap 处理
                // {
                //     use std::net::IpAddr;
                //     // 试图将字符床解析成 ip 地址, 返回 Result
                //     // 使用 unwrap 处理, 成功则返回解析后的地址, 失败则 panic
                //     let ip: IpAddr = "127.0.0.1".parse().unwrap();
                //     println!("{}", ip);
                // }

                // // expect 处理错误
                // {
                //     use std::fs::File;

                //     // 与 unwrap 不同的是 expect 可以自定义错误信息
                //     let file = File::open("hello2.txt").expect("打开文件错误");

                //     println!("{:?}", file);
                // }
            }

            // 不可恢复错误 panic! 全局或系统及的错误 程序崩溃 (数组越界等)
            {
                // 被动触发
                // {
                //     let arr = vec![1, 2, 3];
                //     println!("{}", arr[99]);
                // }

                // 主动触发
                // {
                //     panic!("错误");
                // }

                // 可以通过设置环境变量 RUST_BACKTRACE=1 用来回溯错误
                // panic! 有两种终止模式 默认是展开栈 用来回溯错误信息等 另一种是直接终止 通过修改 Cargo.toml 文件
                // Cargo.toml
                // [profile.release]
                // panic = 'abort'
                // 如果 panic! 发生在主线程 那么程序会终止 发生在子线程 那么该线程会终止
                // Result 枚举出错后调用 unwrap 也是报错后直接 panic!
            }

            // 错误传播
            {
                // // 函数方法等返回错误 在调用方处理
                // {
                //     use std::fs::File;
                //     use std::io::{Error, Read};

                //     // let file = File::open("hello2.txt");
                //     // let file = match file {
                //     //     Ok(file) => file,
                //     //     Err(err) => return Err(err),
                //     // };

                //     fn read_username_from_file() -> Result<String, Error> {
                //         // 打开文件，f 是`Result<文件句柄,io::Error>`
                //         let f = File::open("hello.txt");

                //         let mut f = match f {
                //             // 打开文件成功，将 file 句柄赋值给 f
                //             Ok(file) => file,
                //             // 打开文件失败，将错误返回 (向上传播)
                //             Err(e) => return Err(e),
                //         };

                //         // 创建动态字符串 s
                //         let mut s = String::new();

                //         // 从 f 文件句柄读取数据并写入 s 中
                //         match f.read_to_string(&mut s) {
                //             // 读取成功，返回 Ok 封装的字符串
                //             Ok(_) => Ok(s),
                //             // 将错误向上传播
                //             Err(e) => Err(e),
                //         }
                //     }

                //     let ret: String = read_username_from_file().unwrap();
                //     println!("{}", ret);
                // }

                // // 使用 ? 宏来传播错误
                // // 省去写过多的 match
                // {
                //     use std::fs::File;
                //     use std::io::Error;

                //     fn read_username_from_file() -> Result<String, Error> {
                //         let mut f = File::open("hello.txt")?;
                //         let mut s = String::new();
                //         f.read_to_string(&mut s)?;
                //         Ok(s)
                //     }
                //     let ret: String = read_username_from_file().unwrap();
                //     println!("{}", ret);
                // }

                // // 链式调用
                // {
                //     use std::fs::File;
                //     use std::io::Error;

                //     fn read_username_from_file() -> Result<String, Error> {
                //         let mut s = String::new();
                //         File::open("hello.txt")?.read_to_string(&mut s)?;

                //         let r = if s.is_empty() {
                //             String::from("Is empty!!")
                //         } else {
                //             s
                //         };

                //         Ok(r)
                //     }

                //     let ret: String = read_username_from_file().unwrap();
                //     print!("{}", ret);
                // }

                // // 更少的代码量
                // {
                //     use std::fs;
                //     use std::io::Error;

                //     fn read_username_from_file() -> Result<String, Error> {
                //         fs::read_to_string("hello.txt")
                //     }

                //     let ret: String = read_username_from_file().unwrap();
                //     println!("{}", ret);
                // }
            }
        }
    }
}
