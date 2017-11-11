extern crate factory;
extern crate rand;

fn main() {
    let num = 10;
    println!("Hello, world! {} plus one is {}!", num, factory::add_one(num));
}