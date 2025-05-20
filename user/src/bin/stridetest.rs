#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
use user_lib::{fork, get_time, set_priority, wait, yield_};

#[unsafe(no_mangle)]
pub fn main() -> i32 {
    let mut prio = 16u8;
    let mut cnt = 0;
    let start = get_time();
    if fork() == 0 {
        prio = 32;
        if fork() == 0 {
            prio = 64;
        }
    } else {
        if fork() == 0 {
            prio = 8;
        } else {
            if fork() == 0 {
                prio = 4;
            }else if fork() == 0 {
                prio = 2;
            }
        }
    }
    set_priority(prio);
    loop {
        cnt += 1;
        if get_time() >= start + 1000 {
            break;
        }
        yield_();
    }
    let mut exit_code = 0;
    wait(&mut exit_code);
    println!("{}: {}", prio, cnt);
    0
}
