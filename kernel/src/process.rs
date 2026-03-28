use core::arch::naked_asm;

pub const PROCS_MAX: u8 = 8;
pub const PROCS_UNUSED: u8 = 0;
pub const PROCS_RUNNABLE: u8 = 1;

pub struct Process {
    pub pid: i32,
    pub state: i32,
    pub sp: u32,
    pub stack_addr: u32,
}

const DAMMY_PROC: Process = Process {
    pid: 0,
    state: 0,
    sp: 0,
    stack_addr: 0,
};

pub static mut PROCS: [Process; PROCS_MAX as usize] = [DAMMY_PROC; PROCS_MAX as usize];

#[unsafe(naked)]
pub unsafe extern "C" fn switch_context(prev_sp: *mut u32, next_sp: *const u32) {
    naked_asm!(
        "addi sp, sp, -13 * 4",
        "sw ra,  0  * 4(sp)",
        "sw s0,  1  * 4(sp)",
        "sw s1,  2  * 4(sp)",
        "sw s2,  3  * 4(sp)",
        "sw s3,  4  * 4(sp)",
        "sw s4,  5  * 4(sp)",
        "sw s5,  6  * 4(sp)",
        "sw s6,  7  * 4(sp)",
        "sw s7,  8  * 4(sp)",
        "sw s8,  9  * 4(sp)",
        "sw s9,  10 * 4(sp)",
        "sw s10, 11 * 4(sp)",
        "sw s11, 12 * 4(sp)",
        // スタックポインタの切り替え
        // a0 = prev_sp (保存先アドレス), a1 = next_sp (復元元アドレス)
        "sw sp, 0(a0)",
        "lw sp, 0(a1)",
        // 次のプロセスのスタックからレジスタを復元
        "lw ra,  0  * 4(sp)",
        "lw s0,  1  * 4(sp)",
        "lw s1,  2  * 4(sp)",
        "lw s2,  3  * 4(sp)",
        "lw s3,  4  * 4(sp)",
        "lw s4,  5  * 4(sp)",
        "lw s5,  6  * 4(sp)",
        "lw s6,  7  * 4(sp)",
        "lw s7,  8  * 4(sp)",
        "lw s8,  9  * 4(sp)",
        "lw s9,  10 * 4(sp)",
        "lw s10, 11 * 4(sp)",
        "lw s11, 12 * 4(sp)",
        "addi sp, sp, 13 * 4",
        "ret"
    );
}
