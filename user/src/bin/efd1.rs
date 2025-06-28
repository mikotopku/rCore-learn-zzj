#![no_std]
#![no_main]

use user_lib::{eventfd, exit, read, thread_create, waittid, write, yield_, EFDFlags};

#[macro_use]
extern crate user_lib;
extern crate alloc;

use alloc::vec::Vec;

static PER_THREAD: usize = 1000;
static THREADS: usize = 8;
static EFD: usize = 3;

pub fn t(_: usize) {
    for _ in 0..PER_THREAD {
        let mut tmp = [0u8; core::mem::size_of::<usize>()];
        read(EFD, &mut tmp);
        let tmp = usize::from_ne_bytes(tmp) + 1;
        println!("{}", tmp);
        write(EFD, &tmp.to_ne_bytes());
    }
    exit(0);
}

#[unsafe(no_mangle)]
pub fn main(_: usize, _: &[&str]) -> i32 {
    assert!(eventfd(1, EFDFlags::from_bits(0).unwrap()) as usize == EFD);
    let mut threads = Vec::new();
    for _ in 0..THREADS {
        threads.push(thread_create(t as usize, 0) as usize);
    }
    for i in threads {
        loop {
            if waittid(i) >= 0 { break; }
            else { yield_(); }
        }
    }
    0
}