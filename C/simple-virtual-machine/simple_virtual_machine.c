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

int vm_pop(VM* vm) {
    if (vm->sp >= 0) {
        return vm->stack[vm->sp--];
    } else {
        printf("Stack underflow\n");
        exit(1);
    }
}

void vm_execute(VM* vm) {
    while (1) {
        Instruction instr = vm->code[vm->pc++];
        switch (instr.op) {
            case PUSH:
                vm_push(vm, instr.operand);
                break;
                case ADD: {
                    int b = vm_pop(vm);
                    int a = vm_pop(vm);
                    vm_push(vm, a + b);
                    break;
                }
                case SUB: {
                    int b = vm_pop(vm);
                    int a = vm_pop(vm);
                    vm_push(vm, a - b);
                    break;
                }
                case MUL: {
                    int b = vm_pop(vm);
                    int a = vm_pop(vm);
                    vm_push(vm, a * b);
                    break;
                }
                case DIV: {
                    int b = vm_pop(vm);
                    int a = vm_pop(vm);
                    if (b != 0) {
                        vm_push(vm, a / b);
                    } else {
                        printf("Division by zero\n");
                        exit(1);
                    }
                    break;
                }
                case PRINT:
                    printf("%d\n", vm_pop(vm));
                    break;
                case HALT:
                    return;;
        }
    }
}
