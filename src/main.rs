#![no_std]
#![no_main]

use core::panic::PanicInfo;
use microkernel::print;
use microkernel::println;
use microkernel::uart::Uart;

use microkernel::consts::{NCPU, STACK_PAGE_NUM};

#[repr(C, align(16))]
struct Stack([u8; 4096 * STACK_PAGE_NUM * NCPU]);

#[unsafe(no_mangle)]
static mut STACK0: Stack = Stack([0; 4096 * STACK_PAGE_NUM * NCPU]);

#[unsafe(no_mangle)]
pub fn _start() {
    kmain()
}

#[unsafe(no_mangle)]
pub fn kmain() {
    let mut my_uart = Uart::new(0x1000_0000);

    my_uart.init();

    println!("This is the Operating system");

    loop {
        if let Some(c) = my_uart.get() {
            match c {
                8 => {
                    print!("{}{}{}", 8 as char, ' ', 8 as char);
                }
                10 | 13 => {
                    println!();
                }
                0x1b => {
                    if let Some(next_byte) = my_uart.get() {
                        if next_byte == 91 {
                            if let Some(b) = my_uart.get() {
                                match b as char {
                                    'A' => {
                                        println!("That's the up arrow!");
                                    }
                                    'B' => {
                                        println!("That's the down arrow!");
                                    }
                                    'C' => {
                                        println!("That's the right arrow!");
                                    }
                                    'D' => {
                                        println!("That's the left arrow!");
                                    }
                                    _ => {
                                        println!("That's something else.....");
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {
                    print!("{}", c);
                }
            }
        }
    }
}

#[panic_handler]
fn panic(p: &PanicInfo) -> ! {
    print!("Aborting: ");
    if let Some(loc) = p.location() {
        println!("line {}, file {}: {}", loc.line(), loc.file(), p.message());
    } else {
        println!("{}", p.message());
    }
    loop {}
}
