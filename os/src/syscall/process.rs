//! Process management syscalls
use crate::{
    task::{exit_current_and_run_next, suspend_current_and_run_next,ascend_sys_call,get_sys_call_times},
    timer::get_time_us,
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

// TODO: implement the syscall
pub fn sys_trace(trace_request: usize, id: usize, data: usize) -> isize {
    trace!("kernel: sys_trace");
    match trace_request {
        0 => {
            // trace_request == 0: 读取 user space 中 id 地址处的 u8 数据
            unsafe {
                let id_ptr = id as *const u8; // 转换为指向 u8 的指针
                let value = *id_ptr; // 读取该地址的值
                value as isize // 返回该值，转换为 isize
            }
        }
        1 => {
            // trace_request == 1: 将 data 写入到 id 地址处
            unsafe {
                let id_ptr = id as *mut u8; // 转换为可变指针
                *id_ptr = (data & 0xFF) as u8; // 只写入 data 的最低 8 位
            }
            0 // 返回 0，表示成功
        }
        2 => {
            // trace_request == 2: 查询当前任务的系统调用次数
            let sys_id = id as usize; // 转换 id 为系统调用编号
            ascend_sys_call(sys_id); // 增加系统调用次数
            let sys_call_times = get_sys_call_times(); // 获取当前任务的调用次数
            sys_call_times[sys_id] as isize // 返回指定系统调用的次数
        }
        _ => -1, // 其他无效 trace_request 值，返回 -1
    }
}
