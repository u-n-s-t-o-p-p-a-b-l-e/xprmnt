Compile it with nasm:

```
nasm -f elf64 raw_opcodes.asm -o raw_opcodes.o
ld raw_opcodes.o -o raw_opcodes
```

Run it with:

```
./raw_opcodes
```
