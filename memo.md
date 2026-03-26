# asm! vs naked_asm!
## asm!
* **Automatic Stack Frame:** The compiler automatically generate function prologue and epilogue (like "push ebp", "ret").
* **Variable Binding:** It supports accessing and manipulating local Rust variables within the Assembly blocks.
## naked_asm! 
* **No Stack Frame** The compiler does not generate function prologue and epilogue. (So, you have to write yourself if you need.)
* **No Variable Binding:** Cannot directly reference local Rust variables.

