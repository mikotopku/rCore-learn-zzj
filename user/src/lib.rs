#![no_std]
#![feature(linkage)]

#[macro_use]
pub mod console;
mod lang_items;
mod syscall;
pub mod taskinfo;

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.entry")]
pub extern "C" fn _start() -> ! {
    exit(main());
    panic!("unreachable after sys_exit!");
}

#[linkage = "weak"]
#[unsafe(no_mangle)]
fn main() -> i32 {
    panic!("Cannot find main!");
}

use syscall::*;
use taskinfo::TaskInfo;

pub fn write(fd: usize, buf: &[u8]) -> isize {
    sys_write(fd, buf)
}
pub fn exit(exit_code: i32) -> isize {
    sys_exit(exit_code)
}
pub fn yield_() -> isize {
    sys_yield()
}
pub fn get_time() -> isize {
    sys_get_time()
}

pub fn get_taskinfo(id: usize, ts: *mut TaskInfo) -> isize {
    sys_task_info(id, ts)
}

pub fn mmap(start: usize, len: usize, R: bool, W: bool, X: bool) -> isize {
    let mut prot: usize = 0;
    if R { prot |= 1 << 0; }
    if W { prot |= 1 << 1; }
    if X { prot |= 1 << 2; }
    sys_mmap(start, len, prot)
}

pub fn munmap(start: usize, len: usize) -> isize {
    sys_munmap(start, len)
}