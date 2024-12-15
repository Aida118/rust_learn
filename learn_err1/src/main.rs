// 可恢复错误
// 1. panic! 宏

use std::fs::File;
fn main() {
    // panic!("crash and burn");

    let f = File::open("hello.txt");
    // 2. Result 枚举
    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    let f = File::open("hello.txt");
    let r = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error)
        },
    };
    println!("Hello, world!");
}
// $env:RUST_BACKTRACE="1"
// cargo run

