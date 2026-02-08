#[macro_export]
macro_rules! vec2 {
    ($($x:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn main() {
    let a: Vec<i32> = vec2![1, 2, 3];
    println!("{a:?}");
}
