//! Process management syscalls
use crate::{
    config::{MAX_SYSCALL_NUM, PAGE_SIZE},
    mm::{translated_byte_buffer, MapPermission},
    task::{
        change_program_brk, current_user_token, exit_current_and_run_next, get_start_time, get_syscall_times, get_task_status, mmap, munmap, suspend_current_and_run_next, TaskStatus
    },
    timer::{get_time_ms, get_time_us},
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// Copy from kernel to user
/// src: data reference in kernel, dst: data pointer in userspace
pub fn copyout<T>(src: &T, dst: *mut T) {
    let buffer = translated_byte_buffer(
        current_user_token(),
        dst as *const u8,
        core::mem::size_of::<T>(),
    );
    // SAFETY: src is valid for read
    let src = unsafe {
        core::slice::from_raw_parts(src as *const T as *const u8, core::mem::size_of::<T>())
    };
    buffer
        .into_iter()
        .flatten()
        .zip(src.iter())
        .for_each(|(b, s)| {
            *b = *s;
        });
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");

    let us = get_time_us();
    let timeval = TimeVal {
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };
    copyout(&timeval, ts);
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info");

    let task_info = TaskInfo {
        status: get_task_status(),
        syscall_times: get_syscall_times(),
        time: get_time_ms() - get_start_time(),
    };
    copyout(&task_info, ti);
    0
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(start: usize, mut len: usize, port: usize) -> isize {
    trace!("kernel: sys_mmap");
    if start % PAGE_SIZE != 0 {
        return -1;
    }
    // 向上取PAGE_SIZE整
    if len != 0 {
        len = (len + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);
    }
    if port & !0x7 != 0 || port & 0x7 == 0 {
        return -1;
    }
    let perm = MapPermission::from_bits((port << 1) as u8).unwrap() | MapPermission::U;
    if mmap(start.into(), (start + len).into(), perm) {
        return 0;
    }
    -1
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(start: usize, len: usize) -> isize {
    trace!("kernel: sys_munmap");
    if start % PAGE_SIZE != 0 {
        return -1;
    }
    if len % PAGE_SIZE != 0 {
        return -1;
    }
    if munmap(start.into(), (start + len).into()) {
        return 0;
    }
    -1

}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
