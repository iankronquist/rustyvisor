#define KERNEL_DATA_SEGMENT $0x10
.extern dispatch_interrupt

.global _handle_interrupt
.type _handle_interrupt, @function
_handle_interrupt:
	# Save general purpose registers
	push %rax
	push %rbx
	push %rcx
	push %rdx
	push %rbp
	push %rsi
	push %rdi
	push %r8
	push %r9
	push %r10
	push %r11
	push %r12
	push %r13
	push %r14
	push %r15

	# Save data segment
	mov %ds, %ax
	push %rax

	# Load kernel data segment
	mov KERNEL_DATA_SEGMENT, %ax
	mov %ax, %ds
	mov %ax, %es
	mov %ax, %fs
	mov %ax, %gs

	push %rsp
	call dispatch_interrupt
	add $8, %rsp

	# Restore data segment
	pop %rax
	mov %ax, %ds
	mov %ax, %es
	mov %ax, %fs
	mov %ax, %gs

	# Restore general purpose registers
	pop %r15
	pop %r14
	pop %r13
	pop %r12
	pop %r11
	pop %r10
	pop %r9
	pop %r8
	pop %rdi
	pop %rsi
	pop %rbp
	pop %rdx
	pop %rcx
	pop %rbx
	pop %rax

	# Remove interrupt number & error code
	add $16, %rsp

	sti
	iret

.macro interrupt number
	.global _isr\number
	.type _isr\number, @function
	_isr\number:
		cli
		push \number
		jmp _handle_interrupt
.endm

.macro no_error_code_interrupt number
	.global _isr\number
	.type _isr\number, @function
	_isr\number:
		cli
		push 0
		push \number
		jmp _handle_interrupt
.endm

no_error_code_interrupt 0
no_error_code_interrupt 1
no_error_code_interrupt 2
no_error_code_interrupt 3
no_error_code_interrupt 4
no_error_code_interrupt 5
no_error_code_interrupt 6
no_error_code_interrupt 7
interrupt 8
no_error_code_interrupt 9
interrupt 10
interrupt 11
interrupt 12
interrupt 13
interrupt 14
no_error_code_interrupt 15
no_error_code_interrupt 16
interrupt 17
no_error_code_interrupt 18
no_error_code_interrupt 19
no_error_code_interrupt 20
no_error_code_interrupt 21
no_error_code_interrupt 22
no_error_code_interrupt 23
no_error_code_interrupt 24
no_error_code_interrupt 25
no_error_code_interrupt 26
no_error_code_interrupt 27
no_error_code_interrupt 28
no_error_code_interrupt 29
interrupt 30
no_error_code_interrupt 31
no_error_code_interrupt 32
no_error_code_interrupt 33
no_error_code_interrupt 34
