pub fn main() {
    // {
    //     let mut num = 5;
    //     let r1 = &num as *const i32; // 裸指针/原生指针(raw pointer)

    //     println!("{r1:?}"); // 内存地址
    //     println!("{r2:?}"); // 内存地址

    //     // unsafe 绕不开 rust 的借用检查
    //     unsafe {
    //         println!("{}", *r1);
    //     }
    // }

    // // 裸指针
    // {
    //     // 使用 *const T 和 *mut T 来表示, 前者是不可变, 后者是可变
    //     // * 是类型名称的一部分, 并不是解引用操作
    //     // 裸指针可以绕开 rust 的借用检查, 可以同时拥有多个可变和不可变指针, 没有 drop

    //     let mut num = 5;
    //     // 创建一个不可变裸指针和一个可变裸指针(创建裸指针是安全的行为)
    //     let r1 = &num as *const i32;
    //     let r2 = &mut num as *mut i32;

    //     // 基于内存创建裸指针不安全
    //     let address = 0x012345usize;
    //     let _r3 = address as *const i32; // 不安全的

    //     // 解引用裸指针是不安全的行为
    //     unsafe {
    //         let _deref_r1 = *r1; // 使用 * 对裸指针进行解引用
    //         let _deref_r2 = *r2;
    //     }
    // }

    // // 基于智能指针创建裸指针
    // {
    //     let a = Box::new(5); // a->pointer->5
    //     let _r1 = &*a as *const i32; // 首先解引用 a, 然后创建裸指针
    //     let _r2 = Box::into_raw(a); // 通过 into_raw 转换为裸指针
    // }

    // // 调用不安全的函数
    // {
    //     unsafe fn unsafe_function() {}

    //     unsafe {
    //         unsafe_function();
    //     }
    // }

    // {
    //     use std::slice::{self as std_slice};

    //     let mut v = vec![1, 2, 3, 4, 5, 6];
    //     let r = &mut v[..]; // 转换为切片
    //     let (a, b) = split_at_mut(r, 3);

    //     println!("a: {:?}", a);
    //     println!("b: {:?}", b);

    //     // fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {

    //     //     let len = slice.len();
    //     //     assert!(mid <= len);

    //     //     // 这里正常是会报错的, 因为同时拥有两个可变引用
    //     //     (&mut slice[..mid], &mut slice[mid..])
    //     // }

    //     // unsafe 版
    //     fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    //         let len = slice.len();
    //         let ptr = slice.as_mut_ptr(); // 获取切片的裸指针, 也就是第一个元素的地址
    //         assert!(mid <= len);

    //         unsafe {
    //             println!("ptr: {:?}", *ptr); // 切片的第一个元素
    //         }

    //         unsafe {
    //             (
    //                 std_slice::from_raw_parts_mut(ptr, mid),
    //                 std_slice::from_raw_parts_mut(ptr.add(mid), len - mid),
    //             )
    //         }
    //     }
    // }

    // // FFI (Foreign Function Interface)
    // // 可以与其他语言进行交互
    // {
    //     // 定义其他语言的代码, 以 C 语言为例
    //     // 这里的 C 就是 ABI, 除了 C 还有其他的
    //     extern "C" {
    //         fn abs(input: i32) -> i32;
    //     }

    //     unsafe {
    //         println!("{}", abs(-1));
    //     }

    //     // 声明一个其他语言可以调用的 rust 函数
    //     // 需要使用 #[no_mangle] 来避免 rust 编译器对函数名进行混淆
    //     #[no_mangle]
    //     pub extern "C" fn add(a: i32, b: i32) -> i32 {
    //         a + b
    //     }
    // }
}
