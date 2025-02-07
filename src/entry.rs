use core::arch::asm;

use crate::consts::STACK_PAGE_NUM;

#[link_section = ".entry"]
#[no_mangle]
pub unsafe extern "C" fn _entry() -> ! {
    // set up stack for Rust.
    // sp = stack0 + (hartid * 4096 * STACK_PAGE_NUM)
    asm!(
        "la sp, STACK0",
        "li a0, 4096 * {ssz}",
        "csrr a1, mhartid",
        "addi a1, a1, 1",
        "mul a0, a0, a1",
        "add sp, sp, a0",
        "call _start",
        ssz = const STACK_PAGE_NUM,
    );
    loop {}
}
