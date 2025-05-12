#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{get_time, taskinfo::{SyscallInfo, TaskInfo, TaskStatus}, yield_, get_taskinfo};

#[unsafe(no_mangle)]
fn main() -> i32 {
    let mut ti = TaskInfo::init(0);
    let pti = &mut ti;
    loop {
        let mut i = 0;
        println!("TASKMGR:\nid\tstatus\ttime\tcallid\ttimes\t");
        let mut flag = 0;
        loop {
            let ret = get_taskinfo(i, pti);
            if ret != 0 {
                break;
            }
            if pti.status == TaskStatus::Exited {
                flag += 1;
            }
            println!("{}\t{}\t{}\t", pti.id, pti.status.to_str(), pti.time);
            for scinfo in pti.call {
                println!("\t\t\t{}\t{}", scinfo.id, scinfo.times);
            }
            i += 1;
        }
        if flag >= i - 1 {
            break;
        }
        let current_timer = get_time();
        let wait_for = current_timer + 500;
        while get_time() < wait_for {
            yield_();
        }
    }
    0
}
