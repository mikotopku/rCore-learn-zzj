#![no_std]
#![no_main]

use user_lib::{exit, spawn, waitpid};

#[macro_use]
extern crate user_lib;

#[unsafe(no_mangle)]
pub fn main() -> i32 {
    println!("spawning hello_world");
    let pid = spawn("hello_world\0");
    if pid <= 0 {
        println!("spawn failed");
        exit(-114);
    }
    let mut exit_code = 0i32;
    waitpid(pid as usize, &mut exit_code);
    println!("test passed!");
    0
}
