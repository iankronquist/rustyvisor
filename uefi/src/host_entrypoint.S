.intel_syntax noprefix

.global _guest_first_entry
_guest_first_entry:

    lea rcx, [rip+.guest_enters_here]
    mov rax, 0x0000681e // vmwrite Guest rip
    vmwrite rax, rcx
    jc .errorc
    jz .errorz

    mov rax, 0x00006820 // vmwrite Guest rflags
    pushfq
    pop rcx
    vmwrite rax, rcx
    jc .errorc
    jz .errorz

    mov rcx, rsp
    mov rax, 0x0000681c // vmwrite Guest rsp
    vmwrite rax, rcx
    jc .errorc
    jz .errorz

    vmlaunch
    jc .errorc
    jz .errorz
.errorz:
    mov rax, 1
    ret
.errorc:
    mov rax, 2
    ret
.guest_enters_here:
    xor rax, rax
    ret

.global _host_entrypoint
_host_entrypoint:

	mov fs:0 + 0x8, rax
	mov fs:0 + 0x10, rbx
	mov fs:0 + 0x18, rcx
	mov fs:0 + 0x20, rdx
	mov fs:0 + 0x28, rbp
	mov fs:0 + 0x30, rsi
	mov fs:0 + 0x38, rdi
	mov fs:0 + 0x40, r8
	mov fs:0 + 0x48, r9
	mov fs:0 + 0x50, r10
	mov fs:0 + 0x58, r11
	mov fs:0 + 0x60, r12
	mov fs:0 + 0x68, r13
	mov fs:0 + 0x70, r14
	mov fs:0 + 0x78, r15


	fxsave fs:0x80

    lea rcx, fs:[0]
    call hypervisor_handle_vmexit

	fxrstor fs:0x80
	mov rax, fs:0 + 0x8
	mov rbx, fs:0 + 0x10
	mov rcx, fs:0 + 0x18
	mov rdx, fs:0 + 0x20
	mov rbp, fs:0 + 0x28
	mov rsi, fs:0 + 0x30
	mov rdi, fs:0 + 0x38
	mov r8, fs:0 + 0x40
	mov r9, fs:0 + 0x48
	mov r10, fs:0 + 0x50
	mov r11, fs:0 + 0x58
	mov r12, fs:0 + 0x60
	mov r13, fs:0 + 0x68
	mov r14, fs:0 + 0x70
	mov r15, fs:0 + 0x78

    vmresume

	mov fs:0 + 0x8, rax
	mov fs:0 + 0x10, rbx
	mov fs:0 + 0x18, rcx
	mov fs:0 + 0x20, rdx
	mov fs:0 + 0x28, rbp
	mov fs:0 + 0x30, rsi
	mov fs:0 + 0x38, rdi
	mov fs:0 + 0x40, r8
	mov fs:0 + 0x48, r9
	mov fs:0 + 0x50, r10
	mov fs:0 + 0x58, r11
	mov fs:0 + 0x60, r12
	mov fs:0 + 0x68, r13
	mov fs:0 + 0x70, r14
	mov fs:0 + 0x78, r15

	fxsave fs:0x80

    lea rcx, fs:[0]

    pushf
    pop rdx
    call hypervisor_vmresume_failure
	xchg bx, bx
    int3


.global platform_halt
platform_halt:
    cli
    hlt
    jmp platform_halt

