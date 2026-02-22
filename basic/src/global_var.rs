pub fn main() {
    // 全局变量用来共享一些数据等
    // 生命周期是 static

    // 编译期初始化
    {
        // // 静态常量
        // {
        //     const ID: i32 = 4 / 2; // 可以是一个常量或一个表达式, 必须在编译期可以确定(计算出来)的
        //     println!("{ID}");
        // }

        // // 静态变量
        // {
        //     static mut ID: i32 = 0; // 存储在静态变量中的值必须实现 Sync trait

        //     // 访问和修改静态变量必须在 unsafe 块中
        //     unsafe {
        //         ID += 1;
        //         println!("ID: {}", ID);
        //     }
        // }
    }

    // 运行期初始化
    {
        // 1. 可以使用 lazy_static 包, 详情查看其 doc

        // // 使用 Box::leak
        // {
        //     #[allow(dead_code)]
        //     struct Config {
        //         name: String,
        //     }

        //     static mut CONFIG: Option<&mut Config> = None;

        //     // 创建一个存储在堆上的数据
        //     let c = Box::new(Config {
        //         name: String::from("John"),
        //     });

        //     // 运行期初始化
        //     unsafe {
        //         CONFIG = Some(Box::leak(c));
        //     }
        // }

        // // 从函数中返回全局变量
        // {
        //     #[allow(dead_code)]
        //     struct Config {
        //         name: String,
        //     }

        //     static mut CONFIG: Option<&mut Config> = None;

        //     fn init() -> Option<&'static mut Config> {
        //         Some(Box::leak(Box::new(Config {
        //             name: String::from("John"),
        //         })))
        //     }

        //     unsafe {
        //         CONFIG = init();
        //     }
        // }
    }
}
