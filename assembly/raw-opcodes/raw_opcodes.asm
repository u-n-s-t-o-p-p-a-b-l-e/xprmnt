section .data
    hello db 'Hello, World!', 0

section .text
	global_start

_start:
	db 0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00
	db 0xbf, 0x02, 0x00, 0x00, 0x00
	db 0x48, 0xbe
	dq hello
	db 0xba, 0x0d, 0x00, 0x00, 0x00
	db 0x0f, 0x05

	db 0x48, 0xc7, 0xc0, 0x3c, 0x00, 0x00, 0x00
	db 0x31, 0xff
	db 0x0f, 0x05
