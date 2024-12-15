mod factory{
    pub mod produce_refrigerator{
        pub fn produce(){ // private function/ pub: public function
            println!("produce refrigerator");
        }
    }
    mod produce_washing_machine {
        fn produce(){ // private function
            println!("produce washing machine");
        }
    }
}


fn main() {
    factory::produce_refrigerator::produce();
    println!("Hello, world!");

}
