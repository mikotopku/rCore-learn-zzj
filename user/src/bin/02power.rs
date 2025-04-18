#![no_std]
#![no_main]

use user_lib::console;

#[macro_use]
extern crate user_lib;

#[unsafe(no_mangle)]
fn main() -> i32 {
    user_lib::write(console::STDOUT, b"bonjour\n");
    2
}
