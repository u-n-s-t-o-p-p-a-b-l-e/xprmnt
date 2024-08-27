#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define STACK_SIZE 256
#define CODE_SIZE 256

typedef enum {
    PUSH, ADD, SUB, MUL, DIV, PRINT, HALT
} OpCode;

typedef struct {
    OpCode op;
    int operand;
} Instruction;

typedef struct {
    Instruction code[CODE_SIZE];
    int pc;
    int stack[STACK_SIZE];
    int sp;
} VM;

void vm_init(VM* vm) {
    vm->pc = 0;
    vm->sp = -1;
}

void vm_push(VM* vm, int value) {
    if (vm->sp < STACK_SIZE - 1) {
        vm->stack[++vm->sp] = value;
        } else {
            printf("Stack overflow\n");
            exit(1);
        }
}
