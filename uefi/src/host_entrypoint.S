.intel_syntax noprefix

.global _guest_first_entry
_guest_first_entry:

    lea rdi, [rip+.guest_enters_here]
    mov rax, 0x0000681e // vmwrite Guest rip
    vmwrite rax, rdi
    jc .errorc
    jz .errorz

    mov rax, 0x00006820 // vmwrite Guest rflags
    pushfq
    pop rdi
    vmwrite rax, rdi
    jc .errorc
    jz .errorz

    mov rdi, rsp
    mov rax, 0x0000681c // vmwrite Guest rsp
    vmwrite rax, rdi
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


	push rax
	push rbx
	push rcx
	push rdx
	push rbp
	push rsi
	push rdi
	push r8
	push r9
	push r10
	push r11
	push r12
	push r13
	push r14
	push r15


    mov rcx, rsp
    call hypervisor_handle_vmexit

	pop r15
	pop r14
	pop r13
	pop r12
	pop r11
	pop r10
	pop r9
	pop r8
	pop rdi
	pop rsi
	pop rbp
	pop rdx
	pop rcx
	pop rbx
	pop rax


    vmresume

	push rax
	push rbx
	push rcx
	push rdx
	push rbp
	push rsi
	push rdi
	push r8
	push r9
	push r10
	push r11
	push r12
	push r13
	push r14
	push r15

    mov rcx, rsp

    pushf
    pop rdx
    call hypervisor_vmresume_failure
    int3


.global puts
puts:
    push rax
    push rdx
    push rbp
    push rcx
.puts_loop:
    xor rbp, rbp
.wait_again:
    mov dx, 0x3f8 + 5
    in al, dx
    and al, 0x20
    inc rbp
    #cmp rbp, 0xffffff
    #jg .wait_timeout
    cmp al, 0
    jz .wait_again
.wait_timeout:

    mov al, [rcx]
    cmp al, 0
    jz .puts_done
    inc rcx
    
    mov dx, 0x3f8
    out dx, al
    jmp .puts_loop
.puts_done:
    pop rcx
    pop rbp
    pop rdx
    pop rax
    ret

.global platform_halt
platform_halt:
    cli
    hlt
    jmp platform_halt

.data
hypervisor_handle_vmexit_msg:
.asciz "handling vm exit\r\n"
hypervisor_vmresume_failure_msg:
.asciz "handling vm resume failure\r\n"
hypervisor_resuming:
.asciz "resuming guest\r\n"



.bss

.global __vmm_stack1
__vmm_stack1:
.skip 0x1000
.global __vmm_stack1_end
__vmm_stack1_end:
