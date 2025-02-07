#![no_std]
#![no_main]

#[no_mangle]
pub fn main() {}

use core::panic::PanicInfo;
use microkernel::print;
use microkernel::println;

#[panic_handler]
fn panic(p: &PanicInfo) -> ! {
    print!("Aborting: ");
    if p.location().is_some() {
        println!("line {}, file {}: {}", p.line(), p.file(), p.message());
    } else {
        println!("{}", p.message());
    }
    loop {}
}
