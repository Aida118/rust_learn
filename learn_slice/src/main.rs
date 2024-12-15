// 字符串slice是指向字符串的引用，不是字符串的所有权
// 字符串slice的类型是&str
fn main() {
    let s = String::from("hello world");

    let h = &s[0..5]; // 表示从0开始到5之前的位置
    let h = &s[0..=4]; // 表示从0开始到5的位置
    let h = &s[..5]; // 表示从0开始到5的位置

    println!("h = {}", h);

    let w = &s[6..11];
    let w = &s[6..=10];
    let w = &s[6..];
    let ss = &s[..];
    println!("w = {}", w);
    let ss = String::from("你好");
    //let w1 = &ss[0..1]; // 会报错，因为一个中文字符占3个字节
    //let w1 = &ss[0..3]; // 一个中文字符占3个字节
    //println!("w1 = {}", w1);

    let s3 = "hh"; //&str, 不可变引用

    let a = [1, 2, 3, 4];
    let sss = &a[1..3];
    println!("sss = {:?}", sss); // :?表示打印数组
    

    println!("Hello, world!");
}
