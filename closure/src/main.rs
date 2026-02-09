fn main() {
    // 闭包是一个匿名函数, 可以赋值给变量, 也可以作为参数传递给函数
    // 闭包可以捕获环境中的变量, 这个是函数做不到的, 函数只跟输入的参数相关, 而闭包是可以和上下文相关的
    // {
    //     let a = 1;
    //     // fn fn1() {
    //     //     println!("{a}"); // error, 不能捕获环境中的变量
    //     // }
    //     let c1 = || println!("{a}");
    //     c1();
    // }

    // 闭包在第一次调用后对进行类型推导, 如果第二次调用后的类型和第一次的不一样, 则报错
    // {
    //     let c1 = |x| x;
    //     c1(1); // 已经推导出类型是 i32
    //     c1("a"); // 类型错误
    // }

    // 结构体中的闭包
    // {
    //     struct Cacher<T, P>
    //     where
    //         T: Fn(P) -> P,
    //         P: Copy,
    //     {
    //         query: T,
    //         value: Option<P>,
    //     }

    //     impl<T, P> Cacher<T, P>
    //     where
    //         T: Fn(P) -> P,
    //         P: Copy,
    //     {
    //         fn new(query: T) -> Cacher<T, P> {
    //             Cacher { query, value: None }
    //         }

    //         fn value(&mut self, arg: P) -> P {
    //             match self.value {
    //                 Some(v) => v,
    //                 None => {
    //                     let v = (self.query)(arg);
    //                     self.value = Some(v);
    //                     v
    //                 }
    //             }
    //         }
    //     }

    //     let mut c1 = Cacher::new(|arg| {
    //         println!("c1 arg is: {arg}");
    //         return arg;
    //     });
    //     println!("value is {:?}", c1.value);
    //     println!("get value is: {:?}", c1.value(1));
    //     println!("value is {:?}", c1.value);

    //     let mut c2 = Cacher::new(|arg| {
    //         println!("c2 arg is: {arg}");
    //         return arg;
    //     });
    //     println!("value is {:?}", c2.value);
    //     println!("get value is: {:?}", c2.value(true));
    //     println!("value is {:?}", c2.value);
    // }

    // FnOnce: 该特征表示闭包会拿走捕获的变量的所有权, 并且只执行一次该闭包
    // {
    //     fn fn_once<F>(func: F)
    //     where
    //         // F: FnOnce(usize) -> bool,
    //         F: FnOnce(usize) -> bool + Copy,
    //     {
    //         println!("{}", func(3));
    //         println!("{}", func(4)); // 如何约束不是 FnOnce + Copy 则多次调用会报错
    //     }

    //     let arr = vec![1, 2, 3];
    //     fn_once(|x| {
    //         // arr; // 如果约束是 FnOnce + Copy, 则不能捕获未实现 Copy 特征的变量, 这里会报错
    //         return x == arr.len();
    //     });
    //     println!("{arr:?}");
    // }

    // 使用 move 关键字强制获取闭包捕获变量的所有权
    // {
    //     use std::thread;

    //     let arr = vec![1, 2, 3];
    //     let handle = thread::spawn(move || {
    //         println!("{arr:?}");
    //     });
    //     handle.join().unwrap();
    // }

    // FnMut: 该特征表示闭包会以可变借用的方式捕获变量, 因次它可以对所捕获的变量进行修改
    // {
    //     let mut s = String::new();
    //     let mut change_closure = || s.push_str("Hello World"); // 在变量前使用 mut 关键字代表该闭包所捕获的变量可以被修改
    //     change_closure();
    // }

    // FnMut
    // {
    //     let mut s = String::new();
    //     let change_closure = |str: &str| s.push_str(str);
    //     // 用特征约束或直接在闭包上给参数明确一个类型
    //     // fn exec<'a, F: FnMut(&'a str)>(mut f: F) {
    //     //     f("Hello World");
    //     // }
    //     fn exec<F: FnMut(&str)>(mut f: F) {
    //         f("Hello World");
    //     }
    //     exec(change_closure);
    //     println!("{s}");
    // }

    // {
    //     // 如果闭包捕获的变量实现了 Copy 特征, 它这个闭包也会自动实现 Copy 特征
    //     // 这里捕获的是 s1 的不可变引用, 因不可变引用永远是 Copy 的, 所以 c1 也实现了 Copy
    //     // 所以可以多次调用 exec(c1), 因为 c1 没有因为 exec 的削除而削除, 削除的只是 c1 的副本
    //     let s1 = String::new();
    //     let c1 = || println!("{s1}"); // 拿的是 s 的不可变引用
    //     exec(c1);
    //     exec(c1);

    //     // 捕获的 s2 是可变引用, 因可变引用没有实现 Copy, 所以不能多次调用 exec(c2), 因为在第一次调用时, c2 被削除
    //     let mut s2 = String::new();
    //     let c2 = || s2.push_str("Hello"); // 拿的是 s 的可变引用
    //     exec(c2);
    //     // exec(c2); // 不能再次调用
    //     println!("{s2}");

    //     // 夺走了 c3 的所有权, 又因为捕获的 s3 是在堆中分配的, 因次没有 Copy 特征, 所以 c3 也无法实现 Copy 特征
    //     // 所以不能多次调用 exec(c3), 因为在第一次调用时, c3 被削除
    //     let s3 = String::new();
    //     let c3 = move || println!("{s3}"); // 拿的是 s 的所有权
    //     exec(c3);
    //     // exec(c3); // 不能再次调用

    //     // 夺走了 c4 的所有权, 又因为捕获的 s4 是在栈中分配的, 拥有 Copy 特征, 所以 c4 也实现了 Copy 特征
    //     // 所以可以多次调用 exec(c4), 削除的只是 c4 的副本
    //     let s4 = 1;
    //     let c4 = move || println!("{s4}"); // 拿的是 s 的所有权
    //     exec(c4);
    //     exec(c4);

    //     // rust 中的特性: &mut F 也实现了 FnMut, 所以这里可以传入 c5 的可变引用
    //     let mut s5 = String::new();
    //     let mut c5 = || s5.push_str("Hello"); // 拿的是 s 的可变引用
    //     exec(&mut c5);
    //     exec(&mut c5);
    //     println!("{s5}");

    //     fn exec<F: FnMut()>(mut f: F) {
    //         f();
    //     }
    // }

    // Fn: 该特征表示闭包会以不可变借用的方式捕获变量
    // {
    //     let s = String::new();
    //     let c1 = || println!("{s}");
    //     exec(c1);

    //     fn exec<F: Fn()>(f: F) {
    //         f();
    //     }
    // }

    // {
    //     let s = String::new();
    //     let update_string = || println!("{}", s);

    //     exec(update_string);
    //     exec1(update_string);
    //     exec2(update_string);

    //     fn exec<F: FnOnce()>(f: F) {
    //         f()
    //     }

    //     fn exec1<F: FnMut()>(mut f: F) {
    //         f()
    //     }

    //     fn exec2<F: Fn()>(f: F) {
    //         f()
    //     }
    // }

    // 闭包当作函数的返回值
    // {
    //     fn f1() -> impl Fn(i32) -> i32 {
    //         let num = 5;
    //         move |x| num + x // 需要将 num 的所有权移入闭包
    //     }

    //     let c1 = f1();
    //     let r = c1(2);
    //     println!("{r}");
    // }

    // {
    //     fn f1(x: i32) -> Box<dyn Fn(i32) -> i32> {
    //         let num = 5;
    //         if x > 5 {
    //             Box::new(move |x| num + x)
    //         } else {
    //             Box::new(move |x| num - x)
    //         }
    //     }

    //     let c1 = f1(3);
    //     let r = c1(2);
    //     println!("{r}");
    // }

    // {
    //     let mut s = String::from("Hello");
    //     // c1 是定义了一个参数, 传递进行进行改变, 因次它没有执行, 所以借用检查器此时并不会工作
    //     let c1 = |s: &mut String| {
    //         s.push_str(" World");
    //     };
    //     // 而 c2 此时就已经捕获到换环境变量 s 了, 此时借用检查器工作, c2 获取了 s 的可变引用
    //     let mut c2 = || {
    //         s.push_str(" World");
    //     };

    //     println!("{s}");
    //     // c1(&mut s); // 只有调用 c1 的时候借用检查器才会工作, 在此之前任何引用了 s 的变量都不会受影响
    //     c2(); // 在 c2 调用之前, 它都已经获取了对 s 的可变引用, 所以在此之前不能再有任何对 s 的引用
    //     println!("{s}");
    // }
}
