use super::kernel::alloc_pages;
use core::arch::naked_asm;

pub const PROCS_MAX: u8 = 8;
pub const PROCS_UNUSED: u8 = 0;
pub const PROCS_RUNNABLE: u8 = 1;

pub struct Process {
    pub pid: usize,
    pub state: u8,
    pub sp: usize,
    pub stack_addr: usize,
    pub stack_size: usize,
}

impl Process {
    pub const fn empty() -> Self {
        Process {
            pid: 0,
            state: 0,
            sp: 0,
            stack_addr: 0,
            stack_size: 0,
        }
    }
    pub fn create(pc: usize) -> &'static mut Self {
        // pc(program counter): adress of funcion you want the process to call first
        let mut i = 0;

        unsafe {
            for _ in 0..(PROCS_MAX as usize) {
                if PROCS[i].state == PROCS_UNUSED {
                    break;
                }
                i += 1;
            }
        }

        if i == (PROCS_MAX - 1) as usize {
            panic!("no free process slots");
        }

        unsafe {
            let proc = &mut PROCS[i as usize];
            proc.pid = i + 1;
            proc.state = PROCS_RUNNABLE;
            proc.stack_addr = alloc_pages(2) as usize;
            proc.stack_size = 8192;
            let mut sp: *mut u32 = (proc.stack_addr + proc.stack_size) as *mut u32;
            for _ in 0..12 {
                sp = sp.sub(1);
                sp.write(0);
            }
            sp = sp.sub(1);
            sp.write(pc as u32);
            proc.sp = sp as usize;
            proc
        }
    }
}

const DAMMY_PROC: Process = Process {
    pid: 0,
    state: 0,
    sp: 0,
    stack_addr: 0,
    stack_size: 0,
};

pub static mut PROCS: [Process; PROCS_MAX as usize] = [DAMMY_PROC; PROCS_MAX as usize];
pub static mut CURRENT_PROC: Process = Process::empty();
pub static mut IDLE_PROC: Process = Process::empty();

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

// fn yeild() {
//     unsafe {
//         let mut next = IDLE_PROC;
//         for i in 0..PROCS_MAX {
//             let proc = &PROCS[(CURRENT_PROC.pid + i as usize) % PROCS_MAX as usize];
//             if proc.state == PROCS_RUNNABLE && proc.pid > 0 {
//                 next = proc;
//                 break;
//             }
//         }
//     }
// }
