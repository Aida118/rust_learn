// 1. 创建空string
// 2. 通过字面值创建一个String
// 3. 更新字符串 push_str, push, 
fn main() {
    let mut s0 = String::new();
    s0.push_str("hello");
    
    let s1 = String::from("init sth");
    println!("s1: {}", s1);

    let s1 = ("init sth").to_string();
    println!("s1: {}", s1);

    let mut s2 = String::from("hello");
    s2.push_str(", world");
    println!("s2: {}", s2);

    let ss = "!".to_string();
    s2.push_str(&ss); //
    println!("s2: {}", s2);

    let mut s3 = String::from("tea");
    s3.push('m'); // push 只能push一个字符，用单引号，不能用双引号

    let s1 = "hello".to_string();
    let s2 = String::from(", World");
    let s3 = s1 + &s2; // s1的所有权被转移，s1不能再使用

    let s341 = String::from("tic");
    let s342 = String::from("tac");
    let s343 = String::from("toe");
    let s344 = format!("{}-{}-{}", s341, s342, s343); ///format! 宏不会获取任何参数的所有权，跟println!类似
    println!("s344: {}", s344);

    let s4 = String::from("hello");
    // let s41 = s4[0]; // string不支持索引访问
    println!("s4.len: {}", s4.len());

    let s5 = String::from("你好"); // utf-8 中文字符占3个字节
    println!("s5.len: {}", s5.len());

    let hello = "你好";
    let h5 = &hello[0..3]; // 0..3 代表从0开始，到3之前的字符 byte index 2 is not a char boundary
    println!("h5: {}", h5);

    // 遍历字符串
    // chars
    for c in hello.chars(){
        println!("c: {}", c);
    } //中英文字符都可以遍历

    //bytes
    for b in s4.bytes(){
        println!("b: {}", b);
    }
    println!("s3: {}", s3);


}

