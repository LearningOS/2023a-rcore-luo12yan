//! Types related to task management

use super::TaskContext;
use crate::config::MAX_SYSCALL_NUM;

/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    // LAB1: Add whatever you need about the Task.
    /// task_syscall_times
    pub task_syscall_times: [u32; MAX_SYSCALL_NUM],
    /// task_start_time
    pub task_start_time: usize,
    /// task_end_time
    pub task_end_time: usize,
    /// have_start
    pub have_start: bool,
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}
