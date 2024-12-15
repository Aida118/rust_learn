fn main() {
    // 结构体
    // 定义
    #[derive(Clone)]
    #[derive(Debug)] // 打印结构体
    struct User{
        name: String,
        count: String,
        nonce: u64,
        active: bool,
    }
    // 创建实例
    let xiaoming = User{
        name: String::from("xiaoming"),
        count: String::from("12345678"),
        nonce: 10000,
        active: true,
    };
    // 修改实例
    let mut xiaohuang = User{ //mut 可变
        name: String::from("xiaoming"),
        count: String::from("12345678"),
        nonce: 10000,
        active: true,
    };
    xiaohuang.nonce = 20000;

    // 参数名字和字段名字相同的简写
    let name = String::from("xiaohong");
    let count = String::from("1203945");
    let nonce = 30000;
    let active = false;

    let user1 = User{
        name,
        count,
        nonce,
        active,
    };

    // 从其他结构体创建实例
    let user2 = User{
        name: String::from("xiaohuang"),
        ..user1.clone()
    };
    println!("name = {}", user2.name);

    // 元组结构体
    // 字段没有名字
    // 圆括号中的字段类型是元组结构体的一部分
    struct Point(i32, i32);

    let a = Point(10,20);
    let b = Point(30,40);

    println!("a = ({}, {}), b = ({}, {})", a.0, a.1, b.0, b.1);

    // 没有字段的类单元结构体
    struct A{};
    // 类单元结构体的实例
    let a = A{};

    //打印结构体
    // #[derive(Debug)]
    println!("{:?}", user1); //:? 表示调试格式输出
    println!("{:#?}", user1); //:#? 表示调试格式输出，更易读

    println!("Hello, world!");

}
