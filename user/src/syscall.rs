use core::arch::asm;
use crate::{fs::Stat, taskinfo::TaskInfo};

const SYSCALL_OPEN: usize = 56;
const SYSCALL_CLOSE: usize = 57;
const SYSCALL_READ: usize = 63;
const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_GET_TIME: usize = 169;
const SYSCALL_TASKINFO: usize = 410;
const SYSCALL_MMAP: usize = 222;
const SYSCALL_MUNMAP: usize = 215;
const SYSCALL_GETPID: usize = 172;
const SYSCALL_FORK: usize = 220;
const SYSCALL_EXEC: usize = 221;
const SYSCALL_WAITPID: usize = 260;
const SYSCALL_SPAWN: usize = 400;
const SYSCALL_SET_PRIORITY: usize = 140;
const SYSCALL_LINKAT: usize = 37;
const SYSCALL_UNLINKAT: usize = 35;
const SYSCALL_FSTAT: usize = 80;
pub const MAX_SYSCALL_NUM: usize = 19;

fn syscall(id: usize, args: [usize; 7]) -> isize {
    let mut ret: isize;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") args[0] => ret,
            in("x11") args[1],
            in("x12") args[2],
            in("x13") args[3],
            in("x14") args[4],
            in("x15") args[5],
            in("x16") args[6],
            in("x17") id
        );
    }
    ret
}

pub fn sys_open(path: &str, flags: u32) -> isize {
    syscall(SYSCALL_OPEN, [path.as_ptr() as usize, flags as usize, 0, 0, 0, 0, 0])
}

pub fn sys_close(fd: usize) -> isize {
    syscall(SYSCALL_CLOSE, [fd, 0, 0, 0, 0, 0, 0])
}

pub fn sys_read(fd: usize, buffer: &mut [u8]) -> isize {
    syscall(SYSCALL_READ, [fd, buffer.as_mut_ptr() as usize, buffer.len(), 0, 0, 0, 0])
}


pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len(), 0, 0, 0, 0])
}

pub fn sys_exit(exit_code: i32) -> isize {
    syscall(SYSCALL_EXIT, [exit_code as usize, 0, 0, 0, 0, 0, 0])
}

pub fn sys_yield() -> isize {
    syscall(SYSCALL_YIELD, [0;7])
}

pub fn sys_get_time() -> isize {
    syscall(SYSCALL_GET_TIME, [0;7])
}

pub fn sys_task_info(id: usize, ts: *mut TaskInfo) -> isize {
    syscall(SYSCALL_TASKINFO, [id, ts as usize, 0, 0, 0, 0, 0])
}

pub fn sys_mmap(start: usize, len: usize, prot: usize) -> isize {
    syscall(SYSCALL_MMAP, [start, len, prot, 0, 0, 0, 0])
}

pub fn sys_munmap(start: usize) -> isize {
    syscall(SYSCALL_MUNMAP, [start, 0, 0, 0, 0, 0, 0])
}

pub fn sys_getpid() -> isize {
    syscall(SYSCALL_GETPID, [0;7])
}

pub fn sys_fork() -> isize {
    syscall(SYSCALL_FORK, [0;7])
}

pub fn sys_exec(path: &str) -> isize {
    syscall(SYSCALL_EXEC, [path.as_ptr() as usize, 0, 0, 0, 0, 0, 0])
}

pub fn sys_spawn(path: &str) -> isize {
    syscall(SYSCALL_SPAWN, [path.as_ptr() as usize, 0, 0, 0, 0, 0, 0])
}

pub fn sys_waitpid(pid: isize, exit_code: *mut i32) -> isize {
    syscall(SYSCALL_WAITPID, [pid as usize, exit_code as usize, 0, 0, 0, 0, 0])
}

pub fn sys_set_priority(prio: u8) -> isize {
    syscall(SYSCALL_SET_PRIORITY, [prio as usize, 0, 0, 0, 0, 0, 0])
}

pub fn sys_linkat(olddirfd: i32, oldpath: *const u8, newdirfd: i32, newpath: *const u8, flags: u32) -> isize {
    syscall(SYSCALL_LINKAT, [olddirfd as usize, oldpath as usize, newdirfd as usize, newpath as usize, flags as usize, 0, 0])
}

pub fn sys_unlinkat(dirfd: i32, path: *const u8, flags: u32) -> isize {
    syscall(SYSCALL_UNLINKAT, [dirfd as usize, path as usize, flags as usize, 0, 0, 0, 0])
}

pub fn sys_fstat(fd: i32, st: *mut Stat) -> isize {
    syscall(SYSCALL_FSTAT, [fd as usize, st as usize, 0, 0, 0, 0, 0])
}