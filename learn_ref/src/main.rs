// fn main() {
//     let s1 = gives_ownership(); //返回了s的所有权
//     let s2 = String::from("hello"); 
//     let s3 = takes_and_gives_back(s2);//s2的所有权被转移给函数，函数返回了所有权给s3
//太麻烦了，所以选择使用引用
//     println!("Hello, world!");
// }

// fn gives_ownership() -> String{
//     let s = String::from("hello");
//     s
// }

// fn takes_and_gives_back(s: String) -> String{
//     s
// }
// 引用
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn main(){
    let mut s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("{}", len);
    println!("{}", s1);

    modify_s(&mut s1);
    println!("{}", s1);
}

fn modify_s(s: &mut String) {
    s.push_str(", world");
}