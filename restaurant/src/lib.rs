// 使用 mod 来创建模块
mod front_of_house {
    // 如果一个结构体是 pub, 那它内部的字段并不是 pub
    // 可以逐一设置须要 pub 的字段
    pub struct User {
        pub name: String,
        age: i32,
    }

    impl User {
        pub fn new(name: String) -> User {
            User { name, age: 1 }
        }
    }

    pub enum Color {
        Red,
        Blue,
        Green,
    }

    pub mod hosting {
        pub fn add_to_waitlist() {}
    }

    pub mod serving {
        //
    }

    fn begin_eat() {
        // 使用 super 的方式引用, super 代表父模块 (不是顶层)
        super::eat_at_restaurant();
    }

    fn ready_to_eat() {
        // 通过 self 引用, self 代表自身模块
        self::begin_eat();
        // 等同于直接调用
        begin_eat();
    }
}

fn use_struct_and_enum() {
    // 如何结构体内部的字段未全部公开, 则这种方式构造会出错
    // let user1 = front_of_house::User {
    //     name: String::from("John"),
    // };

    // 可以通过关联函数构造
    let user1 = front_of_house::User::new(String::from("John"));
    // user1.name; // 可以访问
    // user1.age; // 不可以访问私有字段

    let color1 = front_of_house::Color::Blue;
}

// 引用
fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();
    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}
