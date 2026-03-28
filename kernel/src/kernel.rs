use crate::common::PAGE_SIZE;
use crate::println;
use core::arch::asm;
use core::arch::global_asm;
use core::arch::naked_asm;
use core::ptr;

use crate::trap::write_stvec;

global_asm!(
    r#"
.balign 4
.global trap_handler
trap_handler:
    csrw sscratch, sp

    addi sp, sp, -4 * 31

    sw ra,  4 * 0(sp)
    sw gp,  4 * 1(sp)
    sw tp,  4 * 2(sp)
    sw t0,  4 * 3(sp)
    sw t1,  4 * 4(sp)
    sw t2,  4 * 5(sp)
    sw t3,  4 * 6(sp)
    sw t4,  4 * 7(sp)
    sw t5,  4 * 8(sp)
    sw t6,  4 * 9(sp)
    sw a0,  4 * 10(sp)
    sw a1,  4 * 11(sp)
    sw a2,  4 * 12(sp)
    sw a3,  4 * 13(sp)
    sw a4,  4 * 14(sp)
    sw a5,  4 * 15(sp)
    sw a6,  4 * 16(sp)
    sw a7,  4 * 17(sp)
    sw s0,  4 * 18(sp)
    sw s1,  4 * 19(sp)
    sw s2,  4 * 20(sp)
    sw s3,  4 * 21(sp)
    sw s4,  4 * 22(sp)
    sw s5,  4 * 23(sp)
    sw s6,  4 * 24(sp)
    sw s7,  4 * 25(sp)
    sw s8,  4 * 26(sp)
    sw s9,  4 * 27(sp)
    sw s10, 4 * 28(sp)
    sw s11, 4 * 29(sp)

    csrr a0, sscratch
    sw a0, 4 * 30(sp)

    mv a0, sp

    call handle_trap

    lw ra,  4 * 0(sp)
    lw gp,  4 * 1(sp)
    lw tp,  4 * 2(sp)
    lw t0,  4 * 3(sp)
    lw t1,  4 * 4(sp)
    lw t2,  4 * 5(sp)
    lw t3,  4 * 6(sp)
    lw t4,  4 * 7(sp)
    lw t5,  4 * 8(sp)
    lw t6,  4 * 9(sp)
    lw a0,  4 * 10(sp)
    lw a1,  4 * 11(sp)
    lw a2,  4 * 12(sp)
    lw a3,  4 * 13(sp)
    lw a4,  4 * 14(sp)
    lw a5,  4 * 15(sp)
    lw a6,  4 * 16(sp)
    lw a7,  4 * 17(sp)
    lw s0,  4 * 18(sp)
    lw s1,  4 * 19(sp)
    lw s2,  4 * 20(sp)
    lw s3,  4 * 21(sp)
    lw s4,  4 * 22(sp)
    lw s5,  4 * 23(sp)
    lw s6,  4 * 24(sp)
    lw s7,  4 * 25(sp)
    lw s8,  4 * 26(sp)
    lw s9,  4 * 27(sp)
    lw s10, 4 * 28(sp)
    lw s11, 4 * 29(sp)
    
    lw sp,  4 * 30(sp)

    sret
"#
);

unsafe extern "C" {
    static __bss: u8;
    static __bss_end: u8;
    static __stack_top: u8;
    static __free_ram: u8;
    static __free_ram_end: u8;
    fn trap_handler();
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

static mut NEXT_PADDR: u32 = 0; // global!
unsafe fn init_allocater() {
    unsafe {
        NEXT_PADDR = ptr::addr_of!(__free_ram) as u32;
    }
}

unsafe fn alloc_pages(n: u32) -> u32 {
    let free_ram_end = ptr::addr_of!(__free_ram_end) as u32;

    unsafe {
        let paddr = NEXT_PADDR;
        NEXT_PADDR += n * PAGE_SIZE;

        if NEXT_PADDR > free_ram_end {
            panic!("out of memory");
        }
        memset(paddr as *mut u8, 0, (n * PAGE_SIZE) as usize);
        paddr
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() {
    unsafe {
        let bss_start = ptr::addr_of!(__bss) as usize;
        let bss_end = ptr::addr_of!(__bss_end) as usize;
        memset(bss_start as *mut u8, 0, bss_end - bss_start);

        let trap_addr = trap_handler as *const () as usize as u32;
        write_stvec(trap_addr);

        init_allocater();
        let paddr0: u32 = alloc_pages(2);
        let paddr1: u32 = alloc_pages(1);
        println!("alloc_pages test: paddr0=0x{:x}", paddr0);
        println!("alloc_pages test: paddr1=0x{:x}", paddr1);

        panic!("Booted!");
    }
}

#[unsafe(link_section = ".text.boot")]
#[unsafe(naked)]
#[unsafe(no_mangle)]
pub extern "C" fn boot() -> ! {
    naked_asm!("la sp, {stack_top}", "j {kernel_main}",stack_top = sym __stack_top, kernel_main = sym kernel_main);
}
