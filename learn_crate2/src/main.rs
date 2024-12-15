

// use mylib::factory::produce_refrigerator as A; //import the module

use mylib::factory::*; //import all the modules

// use到上一层级，不要直接到函数层级
fn main() {
    produce_refrigerator::produce(); //call the function
    // produce_refrigerator::produce();
    
    println!("Hello, world!");
}
