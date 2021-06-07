.intel_syntax noprefix
.extern interrupts_dispatcher
.global _service_interrupt
_service_interrupt:
	/* Save all registers. */
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

	/* We don't need to clear the high bits of rax but we do anyway */
	xor rax, rax
	/* Save data segment. */
	mov ax, ds
	push rax
	mov ax, es
	push rax
	mov ax, fs
	push rax
	mov ax, gs
	push rax

	# Load kernel data segment.
	mov ax, 0x10
	mov ds, ax
	mov es, ax
	mov fs, ax
	mov gs, ax

	mov rcx, rsp

	/* Call interrupt dispatcher. */
.dispatch:
	.extern interrupt_dispatcher
	call interrupt_dispatcher

	/* Restore data segment. */
	pop rax
	mov ds, ax
	pop rax
	mov es, ax
	pop rax
	mov fs, ax
	pop rax
	mov gs, ax

	/* Restore all registers. */
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

.global iret
iret:
	# Add back extra bit pushed to the stack by the interrupt handler.
	add rsp, 16
	iretq


# Certain interrupts push an error code to the stack.
# When one of these interrupts happens, push the interrupt number after
# the error code which is already on the stack.
.macro interrupt number
	.global _isr\number
	_isr\number:
		cli
		push \number
		jmp _service_interrupt
.endm

# Other interrupts don't push an error code to the stack. In the name of
# consistency, push a dummy value, and then the interrupt number
.macro no_error_code_interrupt number
	.global _isr\number
	_isr\number:
		cli
		push 0
		push \number
		jmp _service_interrupt
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

# Filler irqs
no_error_code_interrupt 21
no_error_code_interrupt 22
no_error_code_interrupt 23
no_error_code_interrupt 24
no_error_code_interrupt 25
no_error_code_interrupt 26
no_error_code_interrupt 27
no_error_code_interrupt 28
no_error_code_interrupt 29
# end of Filler irqs


interrupt 30

no_error_code_interrupt 31



# User defined IRQs
no_error_code_interrupt 32
no_error_code_interrupt 33
no_error_code_interrupt 34
no_error_code_interrupt 35
no_error_code_interrupt 36
no_error_code_interrupt 37
# End of user defined irqs

no_error_code_interrupt 38
no_error_code_interrupt 39
no_error_code_interrupt 40
no_error_code_interrupt 41
no_error_code_interrupt 42
no_error_code_interrupt 43
no_error_code_interrupt 44
no_error_code_interrupt 45
no_error_code_interrupt 46
no_error_code_interrupt 47
no_error_code_interrupt 48
no_error_code_interrupt 49
no_error_code_interrupt 50
no_error_code_interrupt 51
no_error_code_interrupt 52
no_error_code_interrupt 53
no_error_code_interrupt 54
no_error_code_interrupt 55
no_error_code_interrupt 56
no_error_code_interrupt 57
no_error_code_interrupt 58
no_error_code_interrupt 59
no_error_code_interrupt 60
no_error_code_interrupt 61
no_error_code_interrupt 62
no_error_code_interrupt 63
no_error_code_interrupt 64
no_error_code_interrupt 65
no_error_code_interrupt 66
no_error_code_interrupt 67
no_error_code_interrupt 68
no_error_code_interrupt 69
no_error_code_interrupt 70
no_error_code_interrupt 71
no_error_code_interrupt 72
no_error_code_interrupt 73
no_error_code_interrupt 74
no_error_code_interrupt 75
no_error_code_interrupt 76
no_error_code_interrupt 77
no_error_code_interrupt 78
no_error_code_interrupt 79
no_error_code_interrupt 80
no_error_code_interrupt 81
no_error_code_interrupt 82
no_error_code_interrupt 83
no_error_code_interrupt 84
no_error_code_interrupt 85
no_error_code_interrupt 86
no_error_code_interrupt 87
no_error_code_interrupt 88
no_error_code_interrupt 89
no_error_code_interrupt 90
no_error_code_interrupt 91
no_error_code_interrupt 92
no_error_code_interrupt 93
no_error_code_interrupt 94
no_error_code_interrupt 95
no_error_code_interrupt 96
no_error_code_interrupt 97
no_error_code_interrupt 98
no_error_code_interrupt 99
no_error_code_interrupt 100
no_error_code_interrupt 101
no_error_code_interrupt 102
no_error_code_interrupt 103
no_error_code_interrupt 104
no_error_code_interrupt 105
no_error_code_interrupt 106
no_error_code_interrupt 107
no_error_code_interrupt 108
no_error_code_interrupt 109
no_error_code_interrupt 110
no_error_code_interrupt 111
no_error_code_interrupt 112
no_error_code_interrupt 113
no_error_code_interrupt 114
no_error_code_interrupt 115
no_error_code_interrupt 116
no_error_code_interrupt 117
no_error_code_interrupt 118
no_error_code_interrupt 119
no_error_code_interrupt 120
no_error_code_interrupt 121
no_error_code_interrupt 122
no_error_code_interrupt 123
no_error_code_interrupt 124
no_error_code_interrupt 125
no_error_code_interrupt 126
no_error_code_interrupt 127
no_error_code_interrupt 128
no_error_code_interrupt 129
no_error_code_interrupt 130
no_error_code_interrupt 131
no_error_code_interrupt 132
no_error_code_interrupt 133
no_error_code_interrupt 134
no_error_code_interrupt 135
no_error_code_interrupt 136
no_error_code_interrupt 137
no_error_code_interrupt 138
no_error_code_interrupt 139
no_error_code_interrupt 140
no_error_code_interrupt 141
no_error_code_interrupt 142
no_error_code_interrupt 143
no_error_code_interrupt 144
no_error_code_interrupt 145
no_error_code_interrupt 146
no_error_code_interrupt 147
no_error_code_interrupt 148
no_error_code_interrupt 149
no_error_code_interrupt 150
no_error_code_interrupt 151
no_error_code_interrupt 152
no_error_code_interrupt 153
no_error_code_interrupt 154
no_error_code_interrupt 155
no_error_code_interrupt 156
no_error_code_interrupt 157
no_error_code_interrupt 158
no_error_code_interrupt 159
no_error_code_interrupt 160
no_error_code_interrupt 161
no_error_code_interrupt 162
no_error_code_interrupt 163
no_error_code_interrupt 164
no_error_code_interrupt 165
no_error_code_interrupt 166
no_error_code_interrupt 167
no_error_code_interrupt 168
no_error_code_interrupt 169
no_error_code_interrupt 170
no_error_code_interrupt 171
no_error_code_interrupt 172
no_error_code_interrupt 173
no_error_code_interrupt 174
no_error_code_interrupt 175
no_error_code_interrupt 176
no_error_code_interrupt 177
no_error_code_interrupt 178
no_error_code_interrupt 179
no_error_code_interrupt 180
no_error_code_interrupt 181
no_error_code_interrupt 182
no_error_code_interrupt 183
no_error_code_interrupt 184
no_error_code_interrupt 185
no_error_code_interrupt 186
no_error_code_interrupt 187
no_error_code_interrupt 188
no_error_code_interrupt 189
no_error_code_interrupt 190
no_error_code_interrupt 191
no_error_code_interrupt 192
no_error_code_interrupt 193
no_error_code_interrupt 194
no_error_code_interrupt 195
no_error_code_interrupt 196
no_error_code_interrupt 197
no_error_code_interrupt 198
no_error_code_interrupt 199
no_error_code_interrupt 200
no_error_code_interrupt 201
no_error_code_interrupt 202
no_error_code_interrupt 203
no_error_code_interrupt 204
no_error_code_interrupt 205
no_error_code_interrupt 206
no_error_code_interrupt 207
no_error_code_interrupt 208
no_error_code_interrupt 209
no_error_code_interrupt 210
no_error_code_interrupt 211
no_error_code_interrupt 212
no_error_code_interrupt 213
no_error_code_interrupt 214
no_error_code_interrupt 215
no_error_code_interrupt 216
no_error_code_interrupt 217
no_error_code_interrupt 218
no_error_code_interrupt 219
no_error_code_interrupt 220
no_error_code_interrupt 221
no_error_code_interrupt 222
no_error_code_interrupt 223
no_error_code_interrupt 224
no_error_code_interrupt 225
no_error_code_interrupt 226
no_error_code_interrupt 227
no_error_code_interrupt 228
no_error_code_interrupt 229
no_error_code_interrupt 230
no_error_code_interrupt 231
no_error_code_interrupt 232
no_error_code_interrupt 233
no_error_code_interrupt 234
no_error_code_interrupt 235
no_error_code_interrupt 236
no_error_code_interrupt 237
no_error_code_interrupt 238
no_error_code_interrupt 239
no_error_code_interrupt 240
no_error_code_interrupt 241
no_error_code_interrupt 242
no_error_code_interrupt 243
no_error_code_interrupt 244
no_error_code_interrupt 245
no_error_code_interrupt 246
no_error_code_interrupt 247
no_error_code_interrupt 248
no_error_code_interrupt 249
no_error_code_interrupt 250
no_error_code_interrupt 251
no_error_code_interrupt 252
no_error_code_interrupt 253
no_error_code_interrupt 254
no_error_code_interrupt 255
