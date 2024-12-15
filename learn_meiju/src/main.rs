// 1. 类似于c语言的方式定义
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr{
    kind: IpAddrKind,
    address: String,
}

// 2. rust语言提倡的方式定义
enum IpAddr2 {
    V4(String),
    V6(String),
}

// 3. 可以是不同的类型
enum IpAddr3 {
    V4(u8, u8, u8, u8), // 4个u8类型 元组
    V6(String), 
}

// 4. 经典用法
enum Message {
    Quit, // 不带任何数据
    Move{x: i32, y: i32}, // 匿名结构体
    Write(String), // 字符串
    Change(i32, i32, i32),// 元组
}

// 等同于
// struct QuitMessage; // 类单元结构体
// struct MoveMessage { // 匿名结构体
//     x: i32,
//     y: i32,
// }
// struct WriteMessage(String); // 元组结构体
// struct ChangeMessage(i32, i32, i32); // 元组结构体

//5. 枚举类型的方法
impl Message { // 枚举类型的方法
    fn prin(&self){ // self 是枚举类型
        match *self{ // 解引用
            Message::Quit => println!("Quit"),
            Message::Move{x, y} => println!("Move x: {}, y: {}", x, y),
            Message::Change(a, b, c) => println!("Change a: {}, b: {}, c: {}", a, b, c),
            Message::Write(ref s) => println!("Write: {}", s), // 引用用ref
         }
    }
}

fn main() {

    let i1 = IpAddr{
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    
    let i2 = IpAddr{
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let i1 = IpAddr2::V4(String::from("127.0.0.1"));
    let i2 = IpAddr2::V6(String::from("::1"));

    let i1 = IpAddr3::V4(127, 0, 0, 1);
    let i2 = IpAddr3::V6(String::from("::1"));
    
    let quit = Message::Quit;
    quit.prin();

    let mo = Message::Move{x:1, y:2};
    mo.prin();

    let wr = Message::Write(String::from("Hello"));
    wr.prin();

    let ch = Message::Change(1, 2, 3);
    ch.prin();

    println!("Hello, world!");
}
