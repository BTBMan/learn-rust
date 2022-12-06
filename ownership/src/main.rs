fn main() {
    // {
    //     // s无效, 因为s还为声明
    //     let s = "sss"; // 此处是有效的
    //     println!("{s}");
    // } // 作用域结束, s就无效了

    // {
    // 简单了解复杂数据类型String 存储在堆上
    // let mut s = String::from("hello"); // 此处有效

    // s.push_str(", world");

    // println!("{s}");
    // } // 作用域结束, s无效, 会调用String的drop函数

    // move移动
    {
        // let x = 5;
        // let y = x; // 将x的拷贝绑定到y
        // println!("{}----{}", x, y); // x y 被放入栈中

        // let s1 = String::from("hello");
        // let s2 = s1; // 此时rust认为s1不在有效, 这个操作在rust中被称为移动(move)

        // println!("{s1}"); // error 会禁止使用无效的引用
        // String由三部分组成, 1.存放内容的内存的指针, 长度(多少字节的内存), 容量(分配器取得了多少字节的内存).
        // println!("{s2}");

        // 如需克隆 则使用String的克隆方法
        // let s1 = String::from("hello");
        // let s2 = s1.clone();

        // println!("{}----{}", s1, s2);

        // 函数与所有权
        // let s = String::from("hello");
        // println!("{s}"); // s有效
        // takes_ownership(s); // s被移动至函数内
        // println!("{s}"); // s无效

        // let x = 5;
        // println!("{x}"); // 有效
        // makes_copy(x); // x被copy到函数内部
        // println!("{x}"); //  有效

        // fn takes_ownership(p: String) {
        //     println!("inner scope: {p}");
        // }
        // fn makes_copy(p: i32) {
        //     println!("inner scope: {p}");
        // }

        // 返回值
        // {
        //     let s1 = gives_ownership();
        //     println!("{s1}"); // 有效
        //     let s2 = String::from("world");
        //     println!("{s1}"); // 有效
        //     println!("{s2}"); // 有效
        //     let s3 = takes_and_gives_back(s2); // s3有效 s2被移动到函数内部 并且返回了
        //     println!("{s2}"); // s2无效, 因为已经被移动
        //     println!("{s3}"); // s3有效

        //     fn gives_ownership() -> String {
        //         let some_string = String::from("hello");
        //         some_string
        //     }
        //     fn takes_and_gives_back(s: String) -> String {
        //         s // 这里无论返回的是否是移动进来的s 外部的都将失效
        //     }
        // }

        // {
        //     // 如何在函数里返回其他的值呢, 可以在函数中返回元祖, 这是一种方法
        //     let s1 = String::from("hello");
        //     println!("{s1}");
        //     let (s2, len) = test(s1);
        //     // print!("{s1}"); // 无效
        //     print!("{len}");
        //     println!("{s2}");

        //     fn test(s: String) -> (String, usize) {
        //         let length = s.len();

        //         (s, length)
        //     }
        // }

        // {
        //     // 如何不获得所有权并且想使用一个值
        //     let s1 = String::from("hello");
        //     println!("{s1}");
        //     let s2 = test(&s1); // 引用s1(创建一个指向值s1的引用)
        //     println!("{s1}"); // 有效
        //     println!("{s2}");
        //     s2.push_str("abc"); // error 借用的s2不可变

        //     fn test(x: &String) -> &String {
        //         x
        //     }
        // }

        // {
        //     // 可变引用
        //     let mut s1 = String::from("hello");
        //     println!("{s1}");
        //     let s2 = test(&mut s1); // 引用s1(创建一个指向值s1的引用) 并且是可修改的
        //     println!("{s1}"); // error
        //     println!("{s2}");
        //     s2.push_str(", world"); // 可以修改可变引用
        //     println!("{s2}");
        //     println!("{s1}"); // s1已经被修改

        //     fn test(x: &mut String) -> &mut String {
        //         x
        //     }
        // }

        // {
        //     let mut s = String::from("hello");
        //     let r1 = &mut s;
        //     let r2 = &mut s; // error 不能创建多个可变引用

        //     println!("{},{}", r1, r2);
        // }

        // {
        //     let mut s = String::from("hello");

        //     {
        //         let r2 = &mut s; // ok
        //         println!("{}", r2);
        //     }

        //     let r1 = &mut s; // ok

        //     println!("{}", r1,);
        // }

        // {
        //     let mut s = String::from("hello");
        //     let r1 = &s;
        //     let r2 = &s;
        //     let r3 = &mut s; // error

        //     println!("{},{},{}", r1, r2, r3);
        // }

        // {
        //     let mut s = String::from("hello");
        //     let r1 = &s;
        //     let r2 = &s;
        //     println!("{},{}", r1, r2);
        //     let r3 = &mut s; // ok, 因为再次之后r1 r2不再使用

        //     println!("{}", r3);
        // }

        // {
        //     // 悬垂引用
        //     fn dangle() -> &String {
        //         // error
        //         let s = String::from("hello");

        //         &s // 返回s的引用
        //     } // s离开作用域, 内存被释放, 这时候会形成悬垂引用
        // }
    }
}
