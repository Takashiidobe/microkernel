#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

use core::panic::PanicInfo;
use microkernel::plic;
use microkernel::print;
use microkernel::println;
use microkernel::uart::Uart;

extern crate alloc;

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec;
use microkernel::BSS_END;
use microkernel::BSS_START;
use microkernel::DATA_END;
use microkernel::DATA_START;
use microkernel::HEAP_SIZE;
use microkernel::HEAP_START;
use microkernel::KERNEL_STACK_END;
use microkernel::KERNEL_STACK_START;
use microkernel::KERNEL_TABLE;
use microkernel::RODATA_END;
use microkernel::RODATA_START;
use microkernel::TEXT_END;
use microkernel::TEXT_START;
use microkernel::consts::{NCPU, STACK_PAGE_NUM};
use microkernel::cpu;
use microkernel::id_map_range;
use microkernel::memory;
use microkernel::page;

#[repr(C, align(16))]
struct Stack([u8; 4096 * STACK_PAGE_NUM * NCPU]);

#[unsafe(no_mangle)]
static mut STACK0: Stack = Stack([0; 4096 * STACK_PAGE_NUM * NCPU]);

#[unsafe(no_mangle)]
pub fn kinit() {
    // We created kinit, which runs in super-duper mode
    // 3 called "machine mode".
    // The job of kinit() is to get us into supervisor mode
    // as soon as possible.
    // Interrupts are disabled for the duration of kinit()
    Uart::new(0x1000_0000).init();
    page::init();
    memory::init();

    // Map heap allocations
    let root_ptr = memory::get_page_table();
    let root_u = root_ptr as usize;
    let mut root = unsafe { root_ptr.as_mut().unwrap() };
    let kheap_head = memory::get_head() as usize;
    let total_pages = memory::get_num_allocations();
    println!();
    println!();
    unsafe {
        println!("TEXT:   0x{:x} -> 0x{:x}", TEXT_START, TEXT_END);
        println!("RODATA: 0x{:x} -> 0x{:x}", RODATA_START, RODATA_END);
        println!("DATA:   0x{:x} -> 0x{:x}", DATA_START, DATA_END);
        println!("BSS:    0x{:x} -> 0x{:x}", BSS_START, BSS_END);
        println!(
            "STACK:  0x{:x} -> 0x{:x}",
            KERNEL_STACK_START, KERNEL_STACK_END
        );
        println!(
            "HEAP:   0x{:x} -> 0x{:x}",
            kheap_head,
            kheap_head + total_pages * 4096
        );
    }
    id_map_range(
        &mut root,
        kheap_head,
        kheap_head + total_pages * 4096,
        page::EntryBits::ReadWrite.val(),
    );
    unsafe {
        // Map heap descriptors
        let num_pages = HEAP_SIZE / page::PAGE_SIZE;
        id_map_range(
            &mut root,
            HEAP_START,
            HEAP_START + num_pages,
            page::EntryBits::ReadWrite.val(),
        );
        // Map executable section
        id_map_range(
            &mut root,
            TEXT_START,
            TEXT_END,
            page::EntryBits::ReadExecute.val(),
        );
        // Map rodata section
        // We put the ROdata section into the text section, so they can
        // potentially overlap however, we only care that it's read
        // only.
        id_map_range(
            &mut root,
            RODATA_START,
            RODATA_END,
            page::EntryBits::ReadExecute.val(),
        );
        // Map data section
        id_map_range(
            &mut root,
            DATA_START,
            DATA_END,
            page::EntryBits::ReadWrite.val(),
        );
        // Map bss section
        id_map_range(
            &mut root,
            BSS_START,
            BSS_END,
            page::EntryBits::ReadWrite.val(),
        );
        // Map kernel stack
        id_map_range(
            &mut root,
            KERNEL_STACK_START,
            KERNEL_STACK_END,
            page::EntryBits::ReadWrite.val(),
        );
    }

    // UART
    page::map(
        &mut root,
        0x1000_0000,
        0x1000_0000,
        page::EntryBits::ReadWrite.val(),
        0,
    );

    // CLINT
    //  -> MSIP
    page::map(
        &mut root,
        0x0200_0000,
        0x0200_0000,
        page::EntryBits::ReadWrite.val(),
        0,
    );
    //  -> MTIMECMP
    page::map(
        &mut root,
        0x0200_b000,
        0x0200_b000,
        page::EntryBits::ReadWrite.val(),
        0,
    );
    //  -> MTIME
    page::map(
        &mut root,
        0x0200_c000,
        0x0200_c000,
        page::EntryBits::ReadWrite.val(),
        0,
    );
    // PLIC
    id_map_range(
        &mut root,
        0x0c00_0000,
        0x0c00_2000,
        page::EntryBits::ReadWrite.val(),
    );
    id_map_range(
        &mut root,
        0x0c20_0000,
        0x0c20_8000,
        page::EntryBits::ReadWrite.val(),
    );
    page::print_page_allocations();
    // The following shows how we're going to walk to translate a virtual
    // address into a physical address. We will use this whenever a user
    // space application requires services. Since the user space application
    // only knows virtual addresses, we have to translate silently behind
    // the scenes.
    let p = 0x8005_7000 as usize;
    let m = page::virt_to_phys(&root, p).unwrap_or(0);
    println!("Walk 0x{:x} = 0x{:x}", p, m);
    // When we return from here, we'll go back to boot.S and switch into
    // supervisor mode We will return the SATP register to be written when
    // we return. root_u is the root page table's address. When stored into
    // the SATP register, this is divided by 4 KiB (right shift by 12 bits).
    // We enable the MMU by setting mode 8. Bits 63, 62, 61, 60 determine
    // the mode.
    // 0 = Bare (no translation)
    // 8 = Sv39
    // 9 = Sv48
    unsafe {
        // We have to store the kernel's table. The tables will be moved back
        // and forth between the kernel's table and user applicatons' tables.
        KERNEL_TABLE = root_u;
    }
    // table / 4096    Sv39 mode
    (root_u >> 12) | (8 << 60);
    return main();
}

#[unsafe(no_mangle)]
extern "C" fn kinit_hart(hartid: usize) {
    // All non-0 harts initialize here.
    unsafe {
        // We have to store the kernel's table. The tables will be moved
        // back and forth between the kernel's table and user
        // applicatons' tables.
        cpu::mscratch_write((&mut cpu::KERNEL_TRAP_FRAME[hartid] as *mut cpu::TrapFrame) as usize);
        // Copy the same mscratch over to the supervisor version of the
        // same register.
        cpu::sscratch_write(cpu::mscratch_read());
        cpu::KERNEL_TRAP_FRAME[hartid].hartid = hartid;
        // We can't do the following until zalloc() is locked, but we
        // don't have locks, yet :( cpu::KERNEL_TRAP_FRAME[hartid].satp
        // = cpu::KERNEL_TRAP_FRAME[0].satp;
        // cpu::KERNEL_TRAP_FRAME[hartid].trap_stack = page::zalloc(1);
    }
}

#[unsafe(no_mangle)]
pub fn main() {
    // kmain() starts in supervisor mode. So, we should have the trap
    // vector setup and the MMU turned on when we get here.

    // We initialized my_uart in machine mode under kinit for debugging
    // prints, but this just grabs a pointer to it.
    let mut my_uart = Uart::new(0x1000_0000);
    // Create a new scope so that we can test the global allocator and
    // deallocator
    {
        // We have the global allocator, so let's see if that works!
        let k = Box::<u32>::new(100);
        println!("Boxed value = {}", *k);
        memory::print_table();
        // The following comes from the Rust documentation:
        // some bytes, in a vector
        let sparkle_heart = vec![240, 159, 146, 150];
        // We know these bytes are valid, so we'll use `unwrap()`.
        let sparkle_heart = String::from_utf8(sparkle_heart).unwrap();
        println!("String = {}", sparkle_heart);
    }
    // Let's set up the interrupt system via the PLIC. We have to set the threshold to something
    // that won't mask all interrupts.
    println!("Setting up interrupts and PLIC...");
    // We lower the threshold wall so our interrupts can jump over it.
    plic::set_threshold(0);
    // VIRTIO = [1..8]
    // UART0 = 10
    // PCIE = [32..35]
    // Enable the UART interrupt.
    plic::enable(10);
    plic::set_priority(10, 1);
    println!("UART interrupts have been enabled and are awaiting your command");

    // If we get here, the Box, vec, and String should all be freed since
    // they go out of scope. This calls their "Drop" trait.
    // Now see if we can read stuff:
    // Usually we can use #[test] modules in Rust, but it would convolute
    // the task at hand, and it requires us to create the testing harness
    // since the embedded testing system is part of the "std" library.
    loop {
        if let Some(c) = my_uart.get() {
            match c {
                8 => {
                    // This is a backspace, so we
                    // essentially have to write a space and
                    // backup again:
                    print!("{} {}", 8 as char, 8 as char);
                }
                10 | 13 => {
                    // Newline or carriage-return
                    println!();
                }
                _ => {
                    print!("{}", c as char);
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
