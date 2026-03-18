set -xue

QEMU=qemu-system-riscv32

$QEMU -machine virt -bios default -nographic -serial mon:stdio --no-reboot \
  -kernel ./kernel/target/riscv32imac-unknown-none-elf/debug/kernel
