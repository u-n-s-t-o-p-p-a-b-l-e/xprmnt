Assemble the bootloader:
```
nasm -f elf32 boot.asm -o boot.o
```
Compile the kernel code:
```
gcc -m32 -ffreestanding -c kernel.c -o kernel.o
```
Link everything with the updated linker script:
```
ld -m elf_i386 -T linker.ld -o kernel.bin boot.o kernel.o
```
Run with Qemu:
```
qemu-system-i386 -kernel kernel.bin
```
