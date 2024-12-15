mod modA {
    #[derive(Debug)] // 打印结构体
    pub struct A {
        pub number: i32,
        name: String,
    }

    impl A {
        pub fn new_a() -> A {
            A {
                number: 1,
                name: String::from("A"),
            }
        }

        pub fn print_a(&self) {
            println!("number {} name {}", self.number, self.name);
        }
    }
    // 正确声明嵌套模块
    pub mod modB {
        pub fn print_b() {
            println!("print_b");
        }

        // modC 是 modB 的子模块
        pub mod modC {
            pub fn print_c() {
                println!("print_c");
                super::print_b(); // 调用父模块 modB 的函数
            }
        }
    }
}

use modA::A; // 引用 modA 中的 A 结构体

fn main() {
    let a = A::new_a();

    // 通过打印 A 的 Debug 输出
    println!("{:?}", a);

    // 正确地访问 modB 和 modC
    modA::modB::print_b(); // 这里的路径是正确的
    modA::modB::modC::print_c(); // 这里的路径是正确的
    println!("Hello, world!");
}
