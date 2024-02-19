#![no_std] // Telling the compiler to not link the standard library.
#![no_main] // Telling the compiler to not link the main function.

use core::{arch::asm, panic::PanicInfo};

mod boot {
    use core::arch::global_asm;

    global_asm!(".section .text._start");
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        // Turn PIN 21 as an output
        core::ptr::write_volatile(0x3F20_0008 as *mut u32, 1 << 3);

        loop {
            // Turn PIN 21 on
            core::ptr::write_volatile(0x3F20_001C as *mut u32, 1 << 21);

            for _ in 1..50000 {
                asm!("nop");
            }

            // Turn PIN 21 off
            core::ptr::write_volatile(0x3F20_0028 as *mut u32, 0 << 21);

            for _ in 1..50000 {
                asm!("nop");
            }
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
