	section .multiboot_header
header_start:
	dd 0xe85250d6
	dd 0			; Architecture: i386, protected mode, 32 bits
	dd header_end - header_start
	dd 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))

	dw 0			; end tag
	dw 0
	dd 8
header_end:

	section .text
	global start

	bits 32
start:
	mov dword [0xb8000], 0x2f4b2f4f
	hlt
	
