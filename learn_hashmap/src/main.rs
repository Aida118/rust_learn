// 1. HashMap<K,V>
// 2. 创建HashMap
//3. 读取
//4.遍历

// 导入
use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 20);

    let keys = vec![String::from("Blue"), String::from("Red")];
    let values = vec![10, 20];
    let scores: HashMap<_, _> = keys.iter().zip(values.iter()).collect();  // 把元素和值组合成元组

    let k = String::from("Blue");
    let v = scores.get(&k); //返回一个Option<&V>
    match v {
        Some(v) => println!("v:{:?}", v), 
        None => println!("None"),
    }

    let k = String::from("Yellow");
    if let v = scores.get(&k){
        println!("v:{:?}", v);
    }

    // 遍历: 以任意顺序遍历
    for ( key, value) in &scores{
        println!("key:{}, value:{}", key, value);
    }

    // 直接插入，有的话就更新
    let mut ss = HashMap::new();
    ss.insert(String::from("Blue"), 10);
    ss.insert(String::from("Red"), 20);
    println!("ss:{:?}", ss);

    // 键不存在在
    let mut ss1 = HashMap::new();
    ss1.insert(String::from("Blue"), 10);
    ss1.insert(String::from("Red"), 20);
    ss1.entry(String::from("Yellow")).or_insert(30);
    println!("ss1:{:?}", ss1);

    println!("Hello, world!");
}
