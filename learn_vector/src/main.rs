// 1. 创建空的vector：Vec<T>
// 2. 创建包含初始值的vector：vec![]
// 3. 丢弃vec
// 4. 读取元素
// 5. 更新元素
// 6. 遍历元素
// 7. 使用枚举存储多种类型
fn main() {
    //1
    let mut v: Vec<i32> = Vec::new(); //一般vector是可变的，所以要用mut
    // v.push(1);
    //2
    let v = vec![1,2,3]; //rust可以根据内容自动推断类型
    //3
    {
        let v1 = vec![1,2,3];
        // 离开作用域时，v1会被丢弃
    }
    
    //4
    //索引
    let one: &i32 = &v[0];
    println!("one: {}", one); //不用*解引用，为什么？ 因为rust的解引用是自动的
    println!("one: {}", *one); //这样也可以

    //get方法，更推荐，不会因为越界而panic
    match v.get(0) {
        Some(value) => {println!("second: {}", value);}, //这里的value是引用，
        _ => println!("no second value"),
    }

    //5
    let mut v2: Vec<i32> = Vec::new();
    v2.push(1);
    v2.push(2);
    v2.push(3);

    //6
    // 不可变遍历
    for i in &v2{
        println!("{}", i);
    }

    // 可变遍历
    for i in &mut v2{ //这里的i是可变引用
        *i +=1;
        println!("{}", i);
    }

    //7
    // Vector中存储枚举，可以存储不同类型的值，但是要在同一个枚举中，不同的枚举类型是不行的
    enum Context {
        Text(String),
        Float(f32),
        Int(i32),
    }

    let c = vec![
        Context::Text(String::from("hello")),
        Context::Float(1.0),
        Context::Int(1),
    ]

    // 8. 补充
    let mut v = vec![100, 32, 57];
    let first = &v[0]; // 不可变引用
    v.push(6); // 这里会报错，因为first是不可变引用，而push会改变vector的长度，可能会导致first指向的内存被释放
    println!("The first element is: {}", first);

    
    println!("Hello, world!");
}
