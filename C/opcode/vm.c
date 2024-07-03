#include <stdio.h>
#include <stdlib.h>

#define OP_HALT 0x00
#define OP_ADD 0x01
#define OP_SUB 0x02
#define OP_MUL 0x03
#define OP_DIV 0x04
#define OP_LOAD 0x05
#define OP_PRINT 0x06

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

int pop(VM *vm) {
	return vm->stack[vm->sp--];
}
