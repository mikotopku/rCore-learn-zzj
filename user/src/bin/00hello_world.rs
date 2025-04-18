#![no_std]
#![no_main]

use user_lib::yield_;

#[macro_use]
extern crate user_lib;

#[unsafe(no_mangle)]
fn main() -> i32 {
    println!("hello");
    yield_();
    println!("bonjour");
    user_lib::console::print(format_args!("{} foo {:?}", 1, 2));
    0
}
