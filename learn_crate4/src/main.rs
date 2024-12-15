extern crate crypto;

use self::crypto::digest::Digest;
use crypto::sha3::Sha3;
 
fn main() {
    let mut hasher = Sha3::sha3_256();
    hasher.input_str("Hello, world!");
    let result = hasher.result_str();
    println!("Hash is: {}", result);
    println!("Hello, world!");
}
