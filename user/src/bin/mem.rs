#![no_std]
#![no_main]

use user_lib::{mmap, munmap};

#[macro_use]
extern crate user_lib;

#[unsafe(no_mangle)]
pub fn main() -> isize {
    println!("{}", mmap(0xa0200000, 4096, true, true, false));
    println!("{}", munmap(0));
    println!("{}", munmap(0xFFFFFFFFFFFFF000));
    let mut arr = 0xa0200000usize as *mut i32;
    for i in 0..32 {
        unsafe {
            *arr = i;
            arr = arr.add(1);
        }
    }
    let mut arr = 0xa0200000usize as *mut i32;
    for _ in 0..32 {
        unsafe {
            println!("{}", *arr);
            arr = arr.add(1);
        }
    }
    println!("{}", munmap(0xa0200000));
    unsafe { *arr = 0; }
    panic!("it should have been killed");
}