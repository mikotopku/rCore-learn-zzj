use core::fmt::Display;

use crate::syscall::MAX_SYSCALL_NUM;

#[derive(Clone, Copy)]
pub struct SyscallInfo {
    pub id: usize,
    pub times: usize
}

#[derive(Clone, Copy)]
pub struct TaskInfo {
    pub id: usize,
    pub status: TaskStatus,
    pub call: [SyscallInfo; MAX_SYSCALL_NUM],
    pub time: usize
}

#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}

impl TaskInfo {
    pub fn init(id: usize) -> Self {
        Self {
            id: id,
            status: TaskStatus::UnInit,
            call: [SyscallInfo {id: 0, times: 0}; MAX_SYSCALL_NUM],
            time: 0,
        }
    }
}

impl TaskStatus {
    pub fn to_str(&self) -> &str {
        match self{
            Self::Exited => "exited",
            Self::Ready => "ready",
            Self::Running => "running",
            Self::UnInit => "uninit",
        }
    }
}