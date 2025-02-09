// cpu.rs
// CPU and CPU-related routines
// Also contains the kernel's trap frame
// Stephen Marz
// 14 October 2019

use core::{arch::asm, ptr::null_mut};

#[repr(usize)]
pub enum SatpMode {
    Off = 0,
    Sv39 = 8,
    Sv48 = 9,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct TrapFrame {
    pub regs: [usize; 32],   // 0 - 255
    pub fregs: [usize; 32],  // 256 - 511
    pub satp: usize,         // 512 - 519
    pub trap_stack: *mut u8, // 520
    pub hartid: usize,       // 528
}

impl TrapFrame {
    pub const fn zero() -> Self {
        TrapFrame {
            regs: [0; 32],
            fregs: [0; 32],
            satp: 0,
            trap_stack: null_mut(),
            hartid: 0,
        }
    }
}

pub static mut KERNEL_TRAP_FRAME: [TrapFrame; 8] = [TrapFrame::zero(); 8];

pub const fn build_satp(mode: SatpMode, asid: usize, addr: usize) -> usize {
    ((mode as usize) << 60) | ((asid & 0xffff) << 44) | (addr >> 12) & 0xff_ffff_ffff
}

pub fn mhartid_read() -> usize {
    unsafe {
        let rval;
        asm!("csrr {}, mhartid", out(reg) rval);
        rval
    }
}

pub fn mstatus_write(val: usize) {
    unsafe {
        asm!("csrw	mstatus, {}", in(reg) val);
    }
}

pub fn mstatus_read() -> usize {
    unsafe {
        let rval;
        asm!("csrr	{}, mstatus", out(reg) rval);
        rval
    }
}

pub fn stvec_write(val: usize) {
    unsafe {
        asm!("csrw	stvec, {}", in(reg) val);
    }
}

pub fn stvec_read() -> usize {
    unsafe {
        let rval;
        asm!("csrr	{}, stvec",  out(reg) rval);
        rval
    }
}

pub fn mscratch_write(val: usize) {
    unsafe {
        asm!("csrw	mscratch, {}", in(reg) val);
    }
}

pub fn mscratch_read() -> usize {
    unsafe {
        let rval;
        asm!("csrr	{}, mscratch", out(reg) rval);
        rval
    }
}

pub fn mscratch_swap(to: usize) -> usize {
    unsafe {
        let from;
        asm!("csrrw	{}, mscratch, {}", out(reg) from,  in(reg) to);
        from
    }
}

pub fn sscratch_write(val: usize) {
    unsafe {
        asm!("csrw	sscratch, {}",in(reg)val);
    }
}

pub fn sscratch_read() -> usize {
    unsafe {
        let rval;
        asm!("csrr	{}, sscratch",  out(reg) rval);
        rval
    }
}

pub fn sscratch_swap(to: usize) -> usize {
    unsafe {
        let from;
        asm!("csrrw	{}, sscratch, {}", out(reg) from,  in(reg) to);
        from
    }
}

pub fn sepc_write(val: usize) {
    unsafe {
        asm!("csrw sepc, {}", in(reg) val);
    }
}

pub fn sepc_read() -> usize {
    unsafe {
        let rval;
        asm!("csrr {}, sepc", out(reg) rval);
        rval
    }
}

pub fn satp_write(val: usize) {
    unsafe {
        asm!("csrw satp, {}", in(reg) val);
    }
}

pub fn satp_read() -> usize {
    unsafe {
        let rval;
        asm!("csrr {}, satp",  out(reg) rval);
        rval
    }
}

pub fn satp_fence(vaddr: usize, asid: usize) {
    unsafe {
        asm!("sfence.vma {}, {}", in(reg) vaddr, in(reg) asid);
    }
}

pub fn satp_fence_asid(asid: usize) {
    unsafe {
        asm!("sfence.vma zero, {}",  in(reg) asid);
    }
}
