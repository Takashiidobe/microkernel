#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

pub mod consts;
pub mod cpu;
pub mod memory;
pub mod page;
pub mod plic;
pub mod trap;
pub mod uart;

use core::arch::global_asm;

global_asm!(include_str!("asm/mem.S"));
global_asm!(include_str!("asm/boot.S"));
global_asm!(include_str!("asm/trap.S"));

unsafe extern "C" {
    pub static TEXT_START: usize;
    pub static TEXT_END: usize;
    pub static DATA_START: usize;
    pub static DATA_END: usize;
    pub static RODATA_START: usize;
    pub static RODATA_END: usize;
    pub static BSS_START: usize;
    pub static BSS_END: usize;
    pub static KERNEL_STACK_START: usize;
    pub static KERNEL_STACK_END: usize;
    pub static HEAP_START: usize;
    pub static HEAP_SIZE: usize;
    pub static mut KERNEL_TABLE: usize;
}

#[unsafe(no_mangle)]
pub fn kerneltrap() {}

#[macro_export]
macro_rules! print
{
    ($($args:tt)+) => ({
        use core::fmt::Write;
        let _ = write!(Uart::new(0x1000_0000), $($args)+);
    });
}
#[macro_export]
macro_rules! println
{
    () => ({
        print!("\r\n")
    });
    ($fmt:expr) => ({
        print!(concat!($fmt, "\r\n"))
    });
    ($fmt:expr, $($args:tt)+) => ({
        print!(concat!($fmt, "\r\n"), $($args)+)
    });
}
pub fn id_map_range(root: &mut page::Table, start: usize, end: usize, bits: i64) {
    let mut memaddr = start & !(page::PAGE_SIZE - 1);
    let num_kb_pages = (page::align_val(end, 12) - memaddr) / page::PAGE_SIZE;

    // I named this num_kb_pages for future expansion when
    // I decide to allow for GiB (2^30) and 2MiB (2^21) page
    // sizes. However, the overlapping memory regions are causing
    // nightmares.
    for _ in 0..num_kb_pages {
        page::map(root, memaddr, memaddr, bits, 0);
        memaddr += 1 << 12;
    }
}
