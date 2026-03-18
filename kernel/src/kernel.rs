use core::{
    arch::{asm, naked_asm},
    ptr,
};
unsafe extern "C" {
    static __bss: u8;
    static __bss_end: u8;
    static __stack_top: u8;
}

unsafe fn memset(buf: *mut u8, c: u8, n: usize) {
    let mut p = buf;
    for _ in 0..n {
        unsafe {
            ptr::write_volatile(p, c);
            p = p.add(1);
        }
    }
}

pub extern "C" fn kernel_main() {
    unsafe {
        let bss_start = ptr::addr_of!(__bss) as usize;
        let bss_end = ptr::addr_of!(__bss_end) as usize;

        memset(__bss as *mut u8, 0, bss_end - bss_start);
    }

    loop {
        unsafe {
            asm!("wfi");
        }
    }
}

#[unsafe(link_section = ".text.boot")]
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub extern "C" fn boot() -> ! {
    naked_asm!("la sp, {stack_top}", "j {kernel_main}",stack_top = sym __stack_top, kernel_main = sym kernel_main);
}
