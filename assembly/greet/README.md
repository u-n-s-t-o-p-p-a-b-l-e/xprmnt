Assemble and link the code:
```
nasm -f elf64 hi.asm -o hi.o
ld hi.o -o hi
```

Run the program:
```
./hi
```
