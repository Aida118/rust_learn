// 1. OPtion是标准库定义的一个枚举，形式：
// enum Option<T> {
//     None,
//     Some(T),
// }
// 可以处理为None的情况

fn main() {
    let some_number = Some(5); // Some(5)是Option<i32>类型的，自动推导
    let some_string = Some(String::from("a string")); // Some(String)类型的
    let absent_number: Option<i32> = None; // None是Option<i32>类型的
    
    let x: i32 = 5; // i32类型
    let y: Option<i32> = Some(5); // Some(5)是Option<i32>类型的

    // let sum = x + y; // 编译错误，因为x是i32类型，y是Option<i32>类型
    // error[E0277]: cannot add `Option<i32>` to `i32`
    let mut temp = 0;
    match y { //可以处理为None的情况
        Some(i) => {temp = i; }
        None => {println!("do nothing");}
    }
    let sum = x + temp;
    println!("x + y = {}", sum);
    let result = plus_one(y);
    match result {
        None => {println!("do nothing");}
        Some(i) => {println!("result = {}", i);}
    }

    // 如果不处理为None的情况，使用if let
    if let Some(value) = plus_one(y){ //if let语法，如果plus_one(y)返回Some(i)，则执行这里；如果为None，则不执行
        println!("value = {}", value);
    }

    if let Some(value) = plus_one(y){ //if let语法，如果plus_one(y)返回Some(i)，则执行这里；如果为None，则不执行
        println!("value = {}", value);
    } else{
        println!("do nothing");
    }

    println!("result = {:?}", result); // {:?}是调试格式化输出

    println!("Hello, world!");
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x{
        None => None,
        Some(x) => Some(x+1),
    }
}
