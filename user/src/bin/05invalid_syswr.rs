#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

#[unsafe(no_mangle)]
fn main() -> i32 {
    user_lib::write(1, unsafe{core::slice::from_raw_parts(0x80200000 as *const u8, 10)});
    0
}