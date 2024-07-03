Compile the code:

```
gcc vm.c -o vm
```

Run the executable:

```
./vm
```
<br>
# Opcodes:<br><br>

`OP_HALT` : Stop execution. <br>

`OP_ADD` : Pop two values from the stack, add them, and push the result.<br>

`OP_SUB` : Pop two values from the stack, subtract the second from the first, and push the result.<br>

`OP_MUL` : Pop two values from the stack, multiply them, and push the result.<br>

`OP_DIV` : Pop two values from the stack, divide the first by the second, and push the result.<br>

`OP_LOAD` : Load a constant value onto the stack.<br>

`OP_PRINT` : Pop a value from the stack and print it.<br>
<br>

VM Structure: <br>
 
`ip`: Instruction pointer.<br>
`stack`: Stack for storing intermediate values.<br>
`sp`: Stack pointer.<br>
`code`: Pointer to the bytecode.<br>
