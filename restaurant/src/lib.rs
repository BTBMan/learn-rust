// // 使用 mod 来创建模块
// mod front_of_house {
//     // 如果一个结构体是 pub, 那它内部的字段并不是 pub
//     // 可以逐一设置须要 pub 的字段
//     pub struct User {
//         pub name: String,
//         age: i32,
//     }

//     impl User {
//         pub fn new(name: String) -> User {
//             User { name, age: 1 }
//         }
//     }

//     pub enum Color {
//         Red,
//         Blue,
//         Green,
//     }

//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }

//     pub mod serving {
//         //
//     }

//     fn begin_eat() {
//         // 使用 super 的方式引用, super 代表父模块 (不是顶层)
//         super::eat_at_restaurant();
//     }

//     fn ready_to_eat() {
//         // 通过 self 引用, self 代表自身模块
//         self::begin_eat();
//         // 等同于直接调用
//         begin_eat();
//     }
// }

// // 引用
// fn eat_at_restaurant() {
//     // 绝对路径
//     crate::front_of_house::hosting::add_to_waitlist();
//     // 相对路径
//     front_of_house::hosting::add_to_waitlist();
// }

// fn use_struct_and_enum() {
//     // 如何结构体内部的字段未全部公开, 则这种方式构造会出错
//     // let user1 = front_of_house::User {
//     //     name: String::from("John"),
//     // };

//     // 可以通过关联函数构造
//     let user1 = front_of_house::User::new(String::from("John"));
//     // user1.name; // 可以访问
//     // user1.age; // 不可以访问私有字段

//     let color1 = front_of_house::Color::Blue;
// }

// -------------------------------------------------

// mod front_of_house1;
// use crate::front_of_house1::hosting; // 只用有 mod 声明了才可以用 use
// // use front_of_house1::hosting; // 也可以通过相对路径引入

// // mod front_of_house2;
// // use crate::front_of_house2::hosting as hosting2;
// // use front_of_house2::hosting as hosting2;

// // pub use front_of_house2::hosting as hosting_to_export; // 默认导入引来的模块会自动转为私有的, 如果想再次把这个引用的模块导出供外部使用, 则在前面加上 pub 变为可见的

// // use std::io::{self, Write}; // 通过 self 引入 io 模块自身
// // use std::io::*; // 通过 * 引入 io 模块中的所有项

// // 引用
// fn eat_at_restaurant() {
//     // 引用
//     crate::front_of_house1::hosting::add_to_waitlist();
//     hosting::add_to_waitlist();

//     // crate::front_of_house2::hosting::add_to_waitlist();
//     // hosting2::add_to_waitlist();

//     // io::Write;
//     // Write;
// }

// -------------------------------------------------

// 如果我们希望某个 item 在特定的模块中可见
// 可以使用 pub(in crate::module) 的方式
pub mod a {
    pub const I: i32 = 3;

    fn semisecret(x: i32) -> i32 {
        use self::b::c::J; // 由于 self 是 mod a 即是父模块, 所以不能访问子模块 b 内部的 item
        // use self::b::d; // error, d 只在 d 当前的作用域可见
        use self::bb::e;
        e();
        use self::bb::f;
        f();
        x + J
    }

    pub fn bar(z: i32) -> i32 {
        semisecret(I) * z
    }
    pub fn foo(y: i32) -> i32 {
        semisecret(I) + y
    }

    mod b {
        // 通过 pub (in crate::xxx) 的方式指定当前 c 只在 a 中可见
        // xxx 必须是父模块或祖先模块
        pub(in crate::a) mod c {
            pub(in crate::a) const J: i32 = 4;
        }
    }

    pub mod bb {
        #[allow(dead_code)]
        pub(in crate::a) fn g() {}

        // 使用 pub(self) 定义 item 只在当前模块内可见
        pub(self) fn d() {
            e();
        }

        // 使用 pub(super) 定义的 item 只在父模块可见
        pub(super) fn e() {
            d();
        }

        // 使用 pub(crate) 定义的 item 只能在当前包中可见
        pub(crate) fn f() {}
    }
}

#[allow(dead_code)]
fn test() {
    // a::bb::e(); // error, e 只能在 mod a 中使用
    // a::bb::g(); // error, g 只能在 mod a 中使用
    a::bb::f(); // ok, 可以在同一个 crate 中的任何地方使用, 如果被其他 crate 引用的话, 就访问不到了
}
