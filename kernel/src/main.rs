#![no_std]
#![no_main]
mod common;
mod kernel;

use core::panic::PanicInfo;
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Kernel panic:");
    println!("{}", info);

    loop {
        unsafe {
            core::arch::asm!("wfi");
        }
    }
}
