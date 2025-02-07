#![no_std]
#![no_main]

pub mod consts;
pub mod entry;
pub mod start;

use core::arch::global_asm;

global_asm!(include_str!("asm/kernelvec.S"));
global_asm!(include_str!("asm/swtch.S"));
global_asm!(include_str!("asm/trampoline.S"));
global_asm!(include_str!("asm/start.S"));

#[no_mangle]
pub fn kerneltrap() {}

#[macro_export]
macro_rules! print {
    ($($args:tt)+) => {{}};
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
