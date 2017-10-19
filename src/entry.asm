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

	section .bss
	align 4096
p4_table:
	resb 4096
p3_table:
	resb 4096
p2_table:
	resb 4096
stack_bottom:
	resb 64
stack_top:

	section .rodata
gdt64:
	dq 0
	.code:
	dq (1 << 43) | (1 << 44) | (1 << 47) | (1 << 53)
	.pointer:
	dw $ - gdt64 - 1
	dq gdt64

	section .text
	global start
	extern long_mode_start

	bits 32
start:
	mov esp, stack_top
	
	call check_multiboot
	call check_cpuid
	call check_long_mode

	call set_up_page_tables
	call enable_paging

	lgdt [gdt64.pointer]
	jmp long 0x08:long_mode_start

set_up_page_tables:
	mov eax, p3_table	; Map in the P3 table
	or eax, 0b11
	mov [p4_table], eax

	mov eax, p2_table	; Map in the P2 table
	or eax, 0b11
	mov [p3_table], eax

	mov ecx, 0

	.map_p2_table:		; Identity map the lower gigabyte
	mov eax, 0x200000
	mul ecx
	or eax, 0b10000011
	mov [p2_table + ecx * 8], eax

	inc ecx
	cmp ecx, 512
	jne .map_p2_table

	ret

enable_paging:	
	mov eax, p4_table	; CR3 contains the pointer to the highest order page table, P4
	mov cr3, eax

	mov eax, cr4		; Enable Physical Address Extension (PAE), for entry into long mode
	or eax, 1 << 5
	mov cr4, eax

	mov ecx, 0xC0000080	; Set the long mode bit in EFER MSR
	rdmsr
	or eax, 1 << 8
	wrmsr

	mov eax, cr0		; Lastly, enable paging
	or eax, 1 << 31
	mov cr0, eax

	ret

check_multiboot:
	cmp eax, 0x36d76289
	jne .no_multiboot
	ret

	.no_multiboot:
	mov al, "0"
	jmp error
	
check_cpuid:
	pushfd			; If we can flip the ID bit in the flag register, CPUID is available
	pop eax

	mov ecx, eax
	xor eax, 1 << 21

	push eax
	popfd

	pushfd
	pop eax

	push ecx
	popfd
	
	cmp eax, ecx
	je .no_cpuid
	ret
	.no_cpuid:
	mov al, "1"
	jmp error
	
check_long_mode:
	mov eax, 0x80000000 	; This first check ensures that the correct CPUID call can be performed
	cpuid
	cmp eax, 0x80000001
	jb .no_long_mode

	mov eax, 0x80000001	; This second check ensures the LM-bit is set in the extended processor info
	cpuid
	test edx, 1 << 29
	jz .no_long_mode
	ret
	
	.no_long_mode:
	mov al, "2"
	jmp error
	
error:
	mov dword [0xb8000], 0x4f524f45
	mov dword [0xb8004], 0x4f3a4f52
	mov dword [0xb8008], 0x4f204f20
	mov byte  [0xb800a], al
	hlt
	
