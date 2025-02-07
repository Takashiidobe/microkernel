#![no_std]
#![no_main]

use core::panic::PanicInfo;
use microkernel::print;
use microkernel::println;
use microkernel::uart::Uart;

use microkernel::consts::{NCPU, STACK_PAGE_NUM};

#[repr(C, align(16))]
struct Stack([u8; 4096 * STACK_PAGE_NUM * NCPU]);

#[no_mangle]
static mut STACK0: Stack = Stack([0; 4096 * STACK_PAGE_NUM * NCPU]);

#[no_mangle]
pub fn _start() {
    kmain()
}

#[no_mangle]
pub fn kmain() {
    let mut my_uart = Uart::new(0x1000_0000);

    my_uart.init();

    // Now test println! macro!
    println!("This is my operating system!");
    println!("I'm so awesome. If you start typing something, I'll show you what you typed!");

    // Now see if we can read stuff:
    // Usually we can use #[test] modules in Rust, but it would convolute the
    // task at hand. So, we'll just add testing snippets.
    loop {
        if let Some(c) = my_uart.get() {
            match c {
                8 => {
                    // This is a backspace, so we essentially have
                    // to write a space and backup again:
                    print!("{}{}{}", 8 as char, ' ', 8 as char);
                }
                10 | 13 => {
                    // Newline or carriage-return
                    println!();
                }
                0x1b => {
                    // Those familiar with ANSI escape sequences
                    // knows that this is one of them. The next
                    // thing we should get is the left bracket [
                    // These are multi-byte sequences, so we can take
                    // a chance and get from UART ourselves.
                    // Later, we'll button this up.
                    if let Some(next_byte) = my_uart.get() {
                        if next_byte == 91 {
                            // This is a right bracket! We're on our way!
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
