fn main() {
    // 变量与常量
    // let a = 1;
    // a = 2; // error

    // let mut a = 1;
    // a = '2'; // error

    // const MY_CONST: u32 = 1; // 必须声明类型

    // 变量覆盖域作用域
    // let x = 1;

    // println!("{}", x);

    // let x = 2; // 覆盖第一个变量

    // println!("{}", x);

    // {
    //     let x = 3;

    //     println!("{}", x) // 查找当前内部作用域x
    // }

    // println!("{}", x); // 访问不到内部作用域

    // 更多数据类型
    // 两类类型字集 (量标scalar 复合compound)
    // 量标 (整型 浮点型 布尔型 字符型) =============================
    // 整型分为 8,16,32,64,128,arch位, i代表有符号即可以是负数 u代表无符号即正数
    // 浮点型分为f23,f64, f32表示单精度浮点数, f64标识双精度浮点数
    // 布尔型分为true和false
    // 字符型char

    // 数值运算
    // println!("{}", 1 + 1);
    // println!("{}", 1.1 + 1.1);
    // println!("{}", 2 / 3); // 整数除法向下舍入最接近的整数

    // 字符型
    // let c = '1';
    // let z: char = '2'; // 显示声明类型 注意单引号'只能包涵一个字符的长度, 如果想包涵多个字符, 则须要使用双引号"
    // println!("{c}----{z}");

    // 复合型 (元祖, 数组) =============================
    // 元祖长度固定, 多个类型的复合类型, 一但声明不能修改, 使用圆括号创建
    // 数组中每个元素的类型是固定的,长度也是固定的

    // let tup: (char, i32) = ('a', 2);
    // let (x, y) = tup; // 可以使用解构
    // println!("{x}----{y}");
    // println!("{}----{}", tup.0, tup.1); // 也可以使用.加索引的方式取值

    // let a = [1, 2, 3];
    // let b: [u32; 2] = [1, 2]; // 显示的声明类型和长度
    // let c = [4; 4]; // 可以给定初始值加;长度来指定这个数组是定义的长度的相同的值
    //                 // 用索引取值
    // println!("{}", a[0]);
    // println!("{}", b[0]);
    // println!("{}", c[0]);

    // 函数
    // fn another_function(x: i32) {
    //     println!("x is {x}")
    // }

    // another_function(12)

    // 块表达式
    // let a = {
    //     let b = 1;
    //     b + 1 // 注意不要写分号, 没有分号是表达式, 有返回值, 有分号则是语句, 不会有返回值.
    // };
    // println!("a is {a}");

    // 函数的返回值
    // fn num(x: i32) -> i32 {
    //     5 + x // 直接指定值(结尾不可以带分号)
    // }
    // fn num(x: i32) -> i32 {
    //     return 5 + x; // 使用return关键字(结尾有分号)
    // }

    // let x = num(1);

    // println!("x is {x}")

    // 控制流(if ifelse loop while for)

    // let num = 3;
    // if num < 4 {
    //     println!("3 is less that 4!")
    // }

    // error 条件必须是bool值
    // if num {
    //     println!("true")
    // }

    // if num != 0 {
    //     println!("num is not equal to 0")
    // }

    // ifelse
    // if num % 4 == 0 {
    //     println!("num is divisible by 4")
    // } else if num % 3 == 0 {
    //     println!("num is divisible by 3")
    // }

    // let中使用if
    // let condition = true;
    // let is_hundred = true;
    // let number = if condition { 5 } else { 6 }; // 返回值必须是相同的类型
    //                                             // let中的ifelse
    // let number_2 = if !condition {
    //     5
    // } else if is_hundred {
    //     100
    // } else {
    //     6
    // };

    // println!("number is {number}");
    // println!("number_2 is {number_2}");

    // loop循环
    // loop {
    //     println!("again!")
    // }

    // continue 和 break
    // let mut count = 0;
    // loop {
    //     count += 1;

    //     if count == 5 {
    //         continue; // 跳出当前迭代
    //     }

    //     if count == 10 {
    //         break; // 跳出这个循环
    //     }

    //     println!("count is {count}"); // 不会打印出5和10
    // }

    // loop中的返回值
    // let mut count = 0;
    // let result = loop {
    //     // 通过变量接收返回值
    //     count += 1;

    //     if count == 5 {
    //         break count * 2; // break后加要返回的值
    //     }
    // };

    // println!("result is {result}")

    // 循环标签
    // let mut count = 0;

    // // 标记loop标签 注意'号
    // 'first_level_loop: loop {
    //     println!("count is {count}");
    //     let mut in_loop_count = 0;

    //     loop {
    //         println!("in_loop_count: {in_loop_count}");

    //         if in_loop_count == 2 {
    //             break;
    //         }

    //         if count == 2 {
    //             break 'first_level_loop; // 结束标记为'first_level_loop的循环
    //         }

    //         in_loop_count += 1;
    //     }

    //     count += 1;
    // }

    // while循环
    // let mut count = 1;

    // while count < 3 {
    //     println!("count is {count}");

    //     count += 1;
    // }

    // for循环
    // let list = [1, 2, 3, 4, 5];
    // for number in list {
    //     println!("the value is {number}");
    // }

    // 使用for遍历集合中的元素
    // for number in (1..4).rev() {
    //     println!("{number}")
    // }
}
