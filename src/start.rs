use crate::{
    consts::{NCPU, STACK_PAGE_NUM},
    print, println,
};

#[repr(C, align(16))]
struct Stack([u8; 4096 * STACK_PAGE_NUM * NCPU]);

#[no_mangle]
static mut STACK0: Stack = Stack([0; 4096 * STACK_PAGE_NUM * NCPU]);

// #[no_mangle]
// pub fn _start() {
//     println!("hi");
// }
