# asm! vs naked_asm!
## asm!
* **Automatic Stack Frame:** The compiler automatically generate function prologue and epilogue (like "push ebp", "ret").
* **Variable Binding:** It supports accessing and manipulating local Rust variables within the Assembly blocks.
## naked_asm! 
* **No Stack Frame** The compiler does not generate function prologue and epilogue. (So, you have to write yourself if you need.)
* **No Variable Binding:** Cannot directly reference local Rust variables.

# C language's "__volatile__" in Rust
## asm!
asm! macro works like __asm__ __volatile__() by default.

# Why are linker symbols typed as u8 in Rust?
```rust
unsafe extern "C" {
    static __bss: u8;
    static __bss_end: u8;
    static __stack_top: u8;
    static __free_ram: u8;
    static __free_ram_end: u8;
    fn trap_handler();
}
```
These symbols are defined in the linker script to represent memory boundaries, not actual data variables.
'u8' is chosen simply as a minimal, arbitrary placeholder.

# RISC-V General Purpose Registers(GPRs)
In RISC-V Architecture, there are 32 GPRs: x0 ~ x31
## Special Registers for specific purpose
- x0(zero) : It always has 'zero'.
- x1(ra)   : It is used for Return Adress.
- x2(sp)   : Stack Pointer.
- x3(gp)   : Global Pointer.
- x4(tp)   : Thread Pointer.
## Caller-saved registers
- x10-17(a0-7)      : They are used for arguments and return values of functions.
- x5-7,x28-31(t0-9) : They have no specific.
## Callee-saved registers
- x8,9,18-27(s0/fp,s1,s2-11) : Strictly speaking, they are not perfectly the same, but I omit here:) 
## What are Caller-saved and Callee-saved?
__Caller-saved__ means "When someone calls a function, the caller have responsibility to save registers."
In other words, callee can use these registers as he wishes.
__Callee-saved__ means "When someone calls a function, the callee have to save registers for the caller."
If the caller wants to use some values after he called a function, he uses Callee-saved registers.
## switch_context() and Caller-saved registers

# How switch_context() works
CPU doesn't know which process it is running.
CPU only know "I have to run the instruction pc points and use the memory sp points as a workplace."
So, we change values of these registers to change running process.

