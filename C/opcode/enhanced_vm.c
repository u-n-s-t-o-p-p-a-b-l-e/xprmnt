#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <arpa/inet.h>

#define OP_HALT 0x00
#define OP_ADD 0x02
#define OP_SUB 0x03
#define OP_MUL 0x04
#define OP_DIV 0x05
#define OP_LOAD 0x06
#define OP_PUSH 0x07
#define OP_POP 0x08
#define OP_JMP 0x09
#define OP_JZ 0x0A
#define OP_JNZ 0x0B
#define OP_INPUT 0x0C

typedef struct {
	int ip;
	int stack[256];
	int sp;
	unsigned char *code;
} VM;

void init_vm(VM *vm, unsigned char *code) {
	vm->ip = 0;
	vm->sp = -1;
	vm->code = code;
}

void push(VM *vm, int value) {
	vm->stack[++vm->sp] = value;
}

int pop(VM *vm)  {
	return vm->stack[vm->sp--];
}

void execute(VM *vm) {
	int running = 1;
	while (running) {
		unsigned char opcode = vm->code[vm->ip++];
		int a, b, addr;
		switch (opcode) {
			case OP_HALT:
				running = 0;
				break;
			case OP_ADD:
				b = pop(vm);
				a = pop(vm);
				push(vm, a + b);
				break;
			case OP_SUB:
				b = pop(vm);
				a = pop(vm);
				push(vm, a - b);
				break;
			case OP_MUL:
				b = pop(vm);
				a = pop(vm);
				push(vm, a * b);
				break;
			case OP_LOAD:
				a = vm->code[vm->ip++];
				push(vm, a);;
				break;
			case OP_PRINT:
				a = pop(vm);
				printf("%d\n", a);
				break;
			case OP_PUSH:
				a = vm->code[vm->ip++];
				push(vm, a);
				break;
			case OP_POP:
				pop(vm);
				break;
 			case OP_JMP:
				addr = vm->code[vm->ip++];
				vm->ip = addr;
				break;
			case OP_JZ:
				addr = vm->code[vm->ip++];
				if (pop(vm) == 0) {
					vm->ip = addr;
				}
				break;
			case OP_JNZ:
				addr = vm->code[vm->ip++];
				if (pop(vm) != 0) {
					vm->ip = addr;
				}
				break;
			case OP_INPUT:
				scanf("%d", &a);
				push(vm, a);
				break;
			default:
				printf("Unknown opcode: %02x\n", opcode);
				running = 0;
				break;

		}
	}
}
