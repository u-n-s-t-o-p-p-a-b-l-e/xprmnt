Compile the code:

```
gcc vm.c -o vm
```

Run the executable:

```
./vm
```
<br>
Opcodes:
`OP_HALT`: Stop execution.
`OP_ADD`: Pop two values from the stack, add them, and push the result.
`OP_SUB`: Pop two values from the stack, subtract the second from the first, and push the result.
`OP_MUL`: Pop two values from the stack, multiply them, and push the result.
`OP_DIV`: Pop two values from the stack, divide the first by the second, and push the result.
`OP_LOAD`: Load a constant value onto the stack.
`OP_PRINT`: Pop a value from the stack and print it.
<br>

VM Structure:
`ip`: Instruction pointer.
`stack`: Stack for storing intermediate values.
`sp`: Stack pointer.
`code`: Pointer to the bytecode.
