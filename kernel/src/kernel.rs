use crate::println;
use core::arch::asm;
use core::arch::naked_asm;
use core::ptr;

unsafe extern "C" {
    static __bss: u8;
    static __bss_end: u8;
    static __stack_top: u8;
}

struct Sbiret {
    error: i32,
    value: i32,
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

#[allow(clippy::too_many_arguments)]
fn sbi_call(
    arg0: i32,
    arg1: i32,
    arg2: i32,
    arg3: i32,
    arg4: i32,
    arg5: i32,
    fid: i32,
    eid: i32,
) -> Sbiret {
    let a0_out: i32;
    let a1_out: i32;
    unsafe {
        asm!("ecall",inout("a0") arg0 => a0_out,inout("a1") arg1 => a1_out,in("a2") arg2,in("a3") arg3,in("a4") arg4,in("a5") arg5,in("a6") fid,in("a7") eid,);
    }

    Sbiret {
        error: a0_out,
        value: a1_out,
    }
}

pub fn putchar(ch: char) {
    sbi_call(ch as i32, 0, 0, 0, 0, 0, 0, 1);
}

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() {
    unsafe {
        let bss_start = ptr::addr_of!(__bss) as usize;
        let bss_end = ptr::addr_of!(__bss_end) as usize;
        memset(__bss as *mut u8, 0, bss_end - bss_start);
    }

    panic!("booted!");
    println!("unreachable here!");
}

#[unsafe(link_section = ".text.boot")]
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub extern "C" fn boot() -> ! {
    naked_asm!("la sp, {stack_top}", "j {kernel_main}",stack_top = sym __stack_top, kernel_main = sym kernel_main);
}
