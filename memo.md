# asm! vs naked_asm!
## asm!
* **Automatic Stack Frame:** The compiler automatically generate function prologue and epilogue (like "push ebp", "ret").
* **Variable Binding:** It supports accessing and manipulating local Rust variables within the Assembly blocks.
## naked_asm! 
* **No Stack Frame** The compiler does not generate function prologue and epilogue. (So, you have to write yourself if you need.)
* **No Variable Binding:** Cannot directly reference local Rust variables.

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
