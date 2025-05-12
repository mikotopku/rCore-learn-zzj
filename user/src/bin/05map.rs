#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{mmap, munmap};

#[unsafe(no_mangle)]
fn main() -> i32 {
    if mmap(0x90200000, 4096 * 2, true, true, false) != 0 {
        println!("mmap failed");
        return -1;
    }
    let s = unsafe {core::slice::from_raw_parts_mut(0x90200000usize as *mut usize, 2048 * 3)};
    for i in 0..32 {
        s[i] = i;
        s[i + 512] = i;
    }
    for i in 0..32 {
        println!("{}", s[i]);
    }
    if munmap(0x90200000, 4096) != 0 {
        println!("munmap failed");
        return -1;
    }
    for i in 0..32 {
        println!("{}", s[i + 512]);
    }
    println!("{}", s[0]);
    0
}