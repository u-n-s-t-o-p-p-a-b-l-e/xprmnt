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
