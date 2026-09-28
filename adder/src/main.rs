use std::println;

use add_one;

fn main() {
    let value = 45;
    println!("The new value is {}", add_one::add_one(value));
}
