// syscall.rs
// System calls
// Stephen Marz
// 3 Jan 2020

use crate::cpu::TrapFrame;

use crate::print;

pub unsafe fn do_syscall(mepc: usize, frame: *mut TrapFrame) -> usize {
    let syscall_number = unsafe { (*frame).regs[10] };
    match syscall_number {
        0 => {
            // Exit
            mepc + 4
        }
        _ => {
            print!("Unknown syscall number {}", syscall_number);
            mepc + 4
        }
    }
}
