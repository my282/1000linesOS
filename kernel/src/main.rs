#![no_std] // dismiss this error
#![no_main]
mod common;
mod kernel;
mod trap;

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
