section .multiboot
	dd 0x1BADB002
	dd 0x00
	dd - (0x1BADB002 + 0x00)

section .text
global start
extern kmain

start: 
	mov esp, stack_space
	call  kmain
	cli
	hlt

section .bss
	resb 8192
stack_space:
section .note.GNU-stack
	dd 0, 0, 0
