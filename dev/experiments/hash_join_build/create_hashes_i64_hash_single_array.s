_RINvNtCspC5jgAXZxx_17datafusion_common10hash_utils17hash_single_arrayNtNtCsRVR2D38dYC_8foldhash4fast10FixedStateECsg1h6yN5JQqd_7hashasm:
.Lfunc_begin30:
	.cfi_startproc
	.cfi_personality 155, DW.ref.rust_eh_personality
	.cfi_lsda 27, .Lexception30
	push	rbp
	.cfi_def_cfa_offset 16
	push	r15
	.cfi_def_cfa_offset 24
	push	r14
	.cfi_def_cfa_offset 32
	push	r13
	.cfi_def_cfa_offset 40
	push	r12
	.cfi_def_cfa_offset 48
	push	rbx
	.cfi_def_cfa_offset 56
	sub	rsp, 952
	.cfi_def_cfa_offset 1008
	.cfi_offset rbx, -56
	.cfi_offset r12, -48
	.cfi_offset r13, -40
	.cfi_offset r14, -32
	.cfi_offset r15, -24
	.cfi_offset rbp, -16
	mov	qword ptr [rsp + 24], r9
	mov	qword ptr [rsp + 8], r8
	mov	qword ptr [rsp + 16], rcx
	mov	rbp, rdx
	mov	r13, rsi
	mov	qword ptr [rsp + 32], rdi
	movzx	r12d, byte ptr [rsp + 1008]
	mov	rbx, qword ptr [rdx + 56]
	mov	rdi, rsi
	call	rbx
	movzx	ecx, byte ptr [rax]
	add	ecx, -2
	cmp	ecx, 36
	ja	.LBB75_2
	lea	rdx, [rip + .LJTI75_0]
	movsxd	rcx, dword ptr [rdx + 4*rcx]
	add	rcx, rdx
	jmp	rcx
.LBB75_4:
	mov	rdi, r13
	call	qword ptr [rbp + 32]
	mov	rbx, rax
	lea	rdi, [rsp + 144]
	mov	rsi, rax
	call	qword ptr [rdx + 24]
	movdqu	xmm0, xmmword ptr [rsp + 144]
	pcmpeqb	xmm0, xmmword ptr [rip + .LCPI75_14]
	pmovmskb	eax, xmm0
	cmp	eax, 65535
	jne	.LBB75_734
	mov	rcx, qword ptr [rsp + 24]
	mov	qword ptr [rsp + 784], rcx
	mov	rax, qword ptr [rbx + 40]
	mov	qword ptr [rsp + 656], rax
	cmp	rcx, rax
	jne	.LBB75_6
	cmp	qword ptr [rbx + 48], 0
	je	.LBB75_10
	cmp	qword ptr [rbx + 88], 0
	je	.LBB75_10
	mov	rsi, rbx
	add	rsi, 48
	test	r12b, r12b
	je	.LBB75_18
	movabs	r15, 589684135938649225
	lea	rdi, [rsp + 144]
	call	qword ptr [rip + _RNvMNtNtCs7SjCkj1olcZ_12arrow_buffer6buffer4nullNtB2_10NullBuffer13valid_indices@GOTPCREL]
	mov	r9, qword ptr [rsp + 144]
	mov	rcx, qword ptr [rsp + 152]
	mov	r12, qword ptr [rsp + 160]
	mov	rsi, qword ptr [rsp + 168]
	mov	rdi, qword ptr [rsp + 176]
	mov	r8, qword ptr [rsp + 184]
	mov	r13, qword ptr [rsp + 192]
	mov	r10, qword ptr [rsp + 200]
	mov	r11, qword ptr [rbx + 32]
	movabs	rbx, -4563226453097033507
	mov	r14, qword ptr [rsp + 8]
	jmp	.LBB75_31
	.p2align	4
.LBB75_34:
	mov	r13, rsi
	xor	r12d, r12d
	add	r10, 64
.LBB75_31:
	test	r13, r13
	jne	.LBB75_41
	cmp	r12, -2
	je	.LBB75_38
	cmp	r12, 1
	je	.LBB75_34
	test	rdi, rdi
	je	.LBB75_38
	cmp	rdi, r8
	je	.LBB75_38
	mov	r13, qword ptr [rdi]
	add	rdi, 8
	mov	r12, -1
	add	r10, 64
	jmp	.LBB75_31
	.p2align	4
.LBB75_38:
	dec	r9
	cmp	r9, -3
	ja	.LBB75_721
	mov	r12, -2
	mov	r13, rcx
	xor	r9d, r9d
	add	r10, 64
	jmp	.LBB75_31
.LBB75_41:
	rep		bsf	rbp, r13
	add	rbp, r10
	movzx	eax, byte ptr [r11 + rbp]
	lea	rdx, [r13 - 1]
	and	r13, rdx
	xor	rax, qword ptr [r14 + 8*rbp]
	xor	rax, r15
	mul	rbx
	xor	rdx, rax
	mov	qword ptr [r14 + 8*rbp], rdx
	jmp	.LBB75_31
.LBB75_472:
	mov	rdi, r13
	call	qword ptr [rbp + 32]
	mov	rbx, rax
	lea	rdi, [rsp + 144]
	mov	rsi, rax
	call	qword ptr [rdx + 24]
	movdqu	xmm0, xmmword ptr [rsp + 144]
	pcmpeqb	xmm0, xmmword ptr [rip + .LCPI75_3]
	pmovmskb	eax, xmm0
	cmp	eax, 65535
	jne	.LBB75_734
	mov	rdx, qword ptr [rsp + 24]
	mov	qword ptr [rsp + 784], rdx
	mov	rax, qword ptr [rbx + 40]
	mov	rcx, rax
	shr	rcx, 2
	mov	qword ptr [rsp + 656], rcx
	cmp	rdx, rcx
	jne	.LBB75_6
	cmp	qword ptr [rbx + 48], 0
	je	.LBB75_476
	cmp	qword ptr [rbx + 88], 0
	je	.LBB75_476
	mov	rsi, rbx
	add	rsi, 48
	test	r12b, r12b
	je	.LBB75_484
	movabs	r15, 589684135938649225
	lea	rdi, [rsp + 144]
	call	qword ptr [rip + _RNvMNtNtCs7SjCkj1olcZ_12arrow_buffer6buffer4nullNtB2_10NullBuffer13valid_indices@GOTPCREL]
	mov	r9, qword ptr [rsp + 144]
	mov	rcx, qword ptr [rsp + 152]
	mov	r12, qword ptr [rsp + 160]
	mov	rsi, qword ptr [rsp + 168]
	mov	rdi, qword ptr [rsp + 176]
	mov	r8, qword ptr [rsp + 184]
	mov	r13, qword ptr [rsp + 192]
	mov	r10, qword ptr [rsp + 200]
	mov	r11, qword ptr [rbx + 32]
	movabs	rbx, -4563226453097033507
	mov	r14, qword ptr [rsp + 8]
	jmp	.LBB75_497
	.p2align	4
.LBB75_500:
	mov	r13, rsi
	xor	r12d, r12d
	add	r10, 64
.LBB75_497:
	test	r13, r13
	jne	.LBB75_507
	cmp	r12, -2
	je	.LBB75_504
	cmp	r12, 1
	je	.LBB75_500
	test	rdi, rdi
	je	.LBB75_504
	cmp	rdi, r8
	je	.LBB75_504
	mov	r13, qword ptr [rdi]
	add	rdi, 8
	mov	r12, -1
	add	r10, 64
	jmp	.LBB75_497
	.p2align	4
.LBB75_504:
	dec	r9
	cmp	r9, -3
	ja	.LBB75_721
	mov	r12, -2
	mov	r13, rcx
	xor	r9d, r9d
	add	r10, 64
	jmp	.LBB75_497
.LBB75_507:
	rep		bsf	rbp, r13
	add	rbp, r10
	mov	eax, dword ptr [r11 + 4*rbp]
	lea	rdx, [r13 - 1]
	and	r13, rdx
	xor	rax, qword ptr [r14 + 8*rbp]
	xor	rax, r15
	mul	rbx
	xor	rdx, rax
	mov	qword ptr [r14 + 8*rbp], rdx
	jmp	.LBB75_497
.LBB75_426:
	mov	rdi, r13
	call	qword ptr [rbp + 32]
	mov	rbx, rax
	lea	rdi, [rsp + 144]
	mov	rsi, rax
	call	qword ptr [rdx + 24]
	movdqu	xmm0, xmmword ptr [rsp + 144]
	pcmpeqb	xmm0, xmmword ptr [rip + .LCPI75_4]
	pmovmskb	eax, xmm0
	cmp	eax, 65535
	jne	.LBB75_734
	mov	rdx, qword ptr [rsp + 24]
	mov	qword ptr [rsp + 784], rdx
	mov	rax, qword ptr [rbx + 40]
	mov	rcx, rax
	shr	rcx, 3
	mov	qword ptr [rsp + 656], rcx
	cmp	rdx, rcx
	jne	.LBB75_6
	cmp	qword ptr [rbx + 48], 0
	je	.LBB75_430
	cmp	qword ptr [rbx + 88], 0
	je	.LBB75_430
	mov	rsi, rbx
	add	rsi, 48
	test	r12b, r12b
	je	.LBB75_438
	movabs	r12, 589684135938649225
	lea	rdi, [rsp + 144]
	call	qword ptr [rip + _RNvMNtNtCs7SjCkj1olcZ_12arrow_buffer6buffer4nullNtB2_10NullBuffer13valid_indices@GOTPCREL]
	mov	r9, qword ptr [rsp + 144]
	mov	rcx, qword ptr [rsp + 152]
	mov	r13, qword ptr [rsp + 160]
	mov	rsi, qword ptr [rsp + 168]
	mov	rdi, qword ptr [rsp + 176]
	mov	r8, qword ptr [rsp + 184]
	mov	rbp, qword ptr [rsp + 192]
	mov	r10, qword ptr [rsp + 200]
	mov	r11, qword ptr [rbx + 32]
	mov	r14, qword ptr [rsp + 8]
	jmp	.LBB75_451
	.p2align	4
.LBB75_454:
	mov	rbp, rsi
	xor	r13d, r13d
	add	r10, 64
.LBB75_451:
	test	rbp, rbp
	jne	.LBB75_461
	cmp	r13, -2
	je	.LBB75_458
	cmp	r13, 1
	je	.LBB75_454
	test	rdi, rdi
	je	.LBB75_458
	cmp	rdi, r8
	je	.LBB75_458
	mov	rbp, qword ptr [rdi]
	add	rdi, 8
	mov	r13, -1
	add	r10, 64
	jmp	.LBB75_451
	.p2align	4
.LBB75_458:
	dec	r9
	cmp	r9, -3
	ja	.LBB75_721
	mov	r13, -2
	mov	rbp, rcx
	xor	r9d, r9d
	add	r10, 64
	jmp	.LBB75_451
.LBB75_461:
	rep		bsf	rbx, rbp
	add	rbx, r10
	mov	r15, qword ptr [r11 + 8*rbx]
	lea	rax, [rbp - 1]
	and	rbp, rax
	mov	rax, qword ptr [r14 + 8*rbx]
	xor	rax, r12
	mov	rdx, r15
	neg	rdx
	mov	edx, 0
	cmovno	rdx, r15
	rol	rax, 56
	xor	rax, rdx
	movabs	r15, 4577018097722394903
	xor	rdx, r15
	mul	rdx
	xor	rdx, rax
	mov	qword ptr [r14 + 8*rbx], rdx
	jmp	.LBB75_451
.LBB75_174:
	mov	rdi, r13
	call	qword ptr [rbp + 32]
	mov	rbx, rax
	lea	rdi, [rsp + 144]
	mov	rsi, rax
	call	qword ptr [rdx + 24]
	movdqu	xmm0, xmmword ptr [rsp + 144]
	pcmpeqb	xmm0, xmmword ptr [rip + .LCPI75_10]
	pmovmskb	eax, xmm0
	cmp	eax, 65535
	jne	.LBB75_734
	mov	rcx, qword ptr [rsp + 24]
	mov	qword ptr [rsp + 784], rcx
	mov	rax, qword ptr [rbx + 40]
	mov	qword ptr [rsp + 656], rax
	cmp	rcx, rax
	jne	.LBB75_6
	cmp	qword ptr [rbx + 48], 0
	je	.LBB75_178
	cmp	qword ptr [rbx + 88], 0
	je	.LBB75_178
	mov	rsi, rbx
	add	rsi, 48
	test	r12b, r12b
	je	.LBB75_186
	movabs	r15, 589684135938649225
	lea	rdi, [rsp + 144]
	call	qword ptr [rip + _RNvMNtNtCs7SjCkj1olcZ_12arrow_buffer6buffer4nullNtB2_10NullBuffer13valid_indices@GOTPCREL]
	mov	r9, qword ptr [rsp + 144]
	mov	rcx, qword ptr [rsp + 152]
	mov	r12, qword ptr [rsp + 160]
	mov	rsi, qword ptr [rsp + 168]
	mov	rdi, qword ptr [rsp + 176]
	mov	r8, qword ptr [rsp + 184]
	mov	r13, qword ptr [rsp + 192]
	mov	r10, qword ptr [rsp + 200]
	mov	r11, qword ptr [rbx + 32]
	movabs	rbx, -4563226453097033507
	mov	r14, qword ptr [rsp + 8]
	jmp	.LBB75_199
	.p2align	4
.LBB75_202:
	mov	r13, rsi
	xor	r12d, r12d
	add	r10, 64
.LBB75_199:
	test	r13, r13
	jne	.LBB75_209
	cmp	r12, -2
	je	.LBB75_206
	cmp	r12, 1
	je	.LBB75_202
	test	rdi, rdi
	je	.LBB75_206
	cmp	rdi, r8
	je	.LBB75_206
	mov	r13, qword ptr [rdi]
	add	rdi, 8
	mov	r12, -1
	add	r10, 64
	jmp	.LBB75_199
	.p2align	4
.LBB75_206:
	dec	r9
	cmp	r9, -3
	ja	.LBB75_721
	mov	r12, -2
	mov	r13, rcx
	xor	r9d, r9d
	add	r10, 64
	jmp	.LBB75_199
.LBB75_209:
	rep		bsf	rbp, r13
	add	rbp, r10
	movzx	eax, byte ptr [r11 + rbp]
	lea	rdx, [r13 - 1]
	and	r13, rdx
	xor	rax, qword ptr [r14 + 8*rbp]
	xor	rax, r15
	mul	rbx
	xor	rdx, rax
	mov	qword ptr [r14 + 8*rbp], rdx
	jmp	.LBB75_199
.LBB75_469:
	movzx	eax, byte ptr [rax + 1]
	mov	rsi, qword ptr [rbp + 32]
	mov	rdi, r13
	test	eax, eax
	je	.LBB75_646
	cmp	eax, 1
	jne	.LBB75_647
	call	_RINvNtCs33DBFqlTBxO_11arrow_array4cast18as_primitive_arrayNtNtB4_5types19IntervalDayTimeTypeECsg1h6yN5JQqd_7hashasm
	movzx	r8d, r12b
	mov	rdi, rax
	mov	rsi, qword ptr [rsp + 16]
	mov	rdx, qword ptr [rsp + 8]
	mov	rcx, qword ptr [rsp + 24]
	call	_RINvNtCspC5jgAXZxx_17datafusion_common10hash_utils20hash_array_primitiveNtNtCs33DBFqlTBxO_11arrow_array5types19IntervalDayTimeTypeNtNtCsRVR2D38dYC_8foldhash4fast10FixedStateECsg1h6yN5JQqd_7hashasm
	jmp	.LBB75_721
.LBB75_48:
	mov	rdi, r13
	call	qword ptr [rbp + 32]
	mov	rbx, rax
	lea	rdi, [rsp + 144]
	mov	rsi, rax
	call	qword ptr [rdx + 24]
	movdqu	xmm0, xmmword ptr [rsp + 144]
	pcmpeqb	xmm0, xmmword ptr [rip + .LCPI75_13]
	pmovmskb	eax, xmm0
	cmp	eax, 65535
	jne	.LBB75_734
	mov	rdx, qword ptr [rsp + 24]
	mov	qword ptr [rsp + 784], rdx
	mov	rax, qword ptr [rbx + 40]
	mov	rcx, rax
	shr	rcx
	mov	qword ptr [rsp + 656], rcx
	cmp	rdx, rcx
	jne	.LBB75_6
	cmp	qword ptr [rbx + 48], 0
	je	.LBB75_52
	cmp	qword ptr [rbx + 88], 0
	je	.LBB75_52
	mov	rsi, rbx
	add	rsi, 48
	test	r12b, r12b
	je	.LBB75_60
	movabs	r15, 589684135938649225
	lea	rdi, [rsp + 144]
	call	qword ptr [rip + _RNvMNtNtCs7SjCkj1olcZ_12arrow_buffer6buffer4nullNtB2_10NullBuffer13valid_indices@GOTPCREL]
	mov	r9, qword ptr [rsp + 144]
	mov	rcx, qword ptr [rsp + 152]
	mov	r12, qword ptr [rsp + 160]
	mov	rsi, qword ptr [rsp + 168]
	mov	rdi, qword ptr [rsp + 176]
	mov	r8, qword ptr [rsp + 184]
	mov	r13, qword ptr [rsp + 192]
	mov	r10, qword ptr [rsp + 200]
	mov	r11, qword ptr [rbx + 32]
	movabs	rbx, -4563226453097033507
	mov	r14, qword ptr [rsp + 8]
	jmp	.LBB75_73
	.p2align	4
.LBB75_76:
	mov	r13, rsi
	xor	r12d, r12d
	add	r10, 64
.LBB75_73:
	test	r13, r13
	jne	.LBB75_83
	cmp	r12, -2
	je	.LBB75_80
	cmp	r12, 1
	je	.LBB75_76
	test	rdi, rdi
	je	.LBB75_80
	cmp	rdi, r8
	je	.LBB75_80
	mov	r13, qword ptr [rdi]
	add	rdi, 8
	mov	r12, -1
	add	r10, 64
	jmp	.LBB75_73
	.p2align	4
.LBB75_80:
	dec	r9
	cmp	r9, -3
	ja	.LBB75_721
	mov	r12, -2
	mov	r13, rcx
	xor	r9d, r9d
	add	r10, 64
	jmp	.LBB75_73
.LBB75_83:
	rep		bsf	rbp, r13
	add	rbp, r10
	movzx	eax, word ptr [r11 + 2*rbp]
	lea	rdx, [r13 - 1]
	and	r13, rdx
	xor	rax, qword ptr [r14 + 8*rbp]
	xor	rax, r15
	mul	rbx
	xor	rdx, rax
	mov	qword ptr [r14 + 8*rbp], rdx
	jmp	.LBB75_73
.LBB75_216:
	mov	rdi, r13
	call	qword ptr [rbp + 32]
	mov	rbx, rax
	lea	rdi, [rsp + 144]
	mov	rsi, rax
	call	qword ptr [rdx + 24]
	movdqu	xmm0, xmmword ptr [rsp + 144]
	pcmpeqb	xmm0, xmmword ptr [rip + .LCPI75_9]
	pmovmskb	eax, xmm0
	cmp	eax, 65535
	jne	.LBB75_734
	mov	rdx, qword ptr [rsp + 24]
	mov	qword ptr [rsp + 784], rdx
	mov	rax, qword ptr [rbx + 40]
	mov	rcx, rax
	shr	rcx
	mov	qword ptr [rsp + 656], rcx
	cmp	rdx, rcx
	jne	.LBB75_6
	cmp	qword ptr [rbx + 48], 0
	je	.LBB75_220
	cmp	qword ptr [rbx + 88], 0
	je	.LBB75_220
	mov	rsi, rbx
	add	rsi, 48
	test	r12b, r12b
	je	.LBB75_228
	movabs	r15, 589684135938649225
	lea	rdi, [rsp + 144]
	call	qword ptr [rip + _RNvMNtNtCs7SjCkj1olcZ_12arrow_buffer6buffer4nullNtB2_10NullBuffer13valid_indices@GOTPCREL]
	mov	r9, qword ptr [rsp + 144]
	mov	rcx, qword ptr [rsp + 152]
	mov	r12, qword ptr [rsp + 160]
	mov	rsi, qword ptr [rsp + 168]
	mov	rdi, qword ptr [rsp + 176]
	mov	r8, qword ptr [rsp + 184]
	mov	r13, qword ptr [rsp + 192]
	mov	r10, qword ptr [rsp + 200]
	mov	r11, qword ptr [rbx + 32]
	movabs	rbx, -4563226453097033507
	mov	r14, qword ptr [rsp + 8]
	jmp	.LBB75_241
	.p2align	4
.LBB75_244:
	mov	r13, rsi
	xor	r12d, r12d
	add	r10, 64
.LBB75_241:
	test	r13, r13
	jne	.LBB75_251
	cmp	r12, -2
	je	.LBB75_248
	cmp	r12, 1
	je	.LBB75_244
	test	rdi, rdi
	je	.LBB75_248
	cmp	rdi, r8
	je	.LBB75_248
	mov	r13, qword ptr [rdi]
	add	rdi, 8
	mov	r12, -1
	add	r10, 64
	jmp	.LBB75_241
	.p2align	4
.LBB75_248:
	dec	r9
	cmp	r9, -3
	ja	.LBB75_721
	mov	r12, -2
	mov	r13, rcx
	xor	r9d, r9d
	add	r10, 64
	jmp	.LBB75_241
.LBB75_251:
	rep		bsf	rbp, r13
	add	rbp, r10
	movzx	eax, word ptr [r11 + 2*rbp]
	lea	rdx, [r13 - 1]
	and	r13, rdx
	xor	rax, qword ptr [r14 + 8*rbp]
	xor	rax, r15
	mul	rbx
	xor	rdx, rax
	mov	qword ptr [r14 + 8*rbp], rdx
	jmp	.LBB75_241
.LBB75_468:
	movzx	eax, byte ptr [rax + 1]
	mov	rsi, qword ptr [rbp + 32]
	lea	rcx, [rip + .LJTI75_1]
	movsxd	rax, dword ptr [rcx + 4*rax]
	add	rax, rcx
	jmp	rax
.LBB75_720:
	mov	rdi, r13
	call	_RINvNtCs33DBFqlTBxO_11arrow_array4cast18as_primitive_arrayNtNtB4_5types18DurationSecondTypeECsg1h6yN5JQqd_7hashasm
	movzx	r8d, r12b
	mov	rdi, rax
	mov	rsi, qword ptr [rsp + 16]
	mov	rdx, qword ptr [rsp + 8]
	mov	rcx, qword ptr [rsp + 24]
	call	_RINvNtCspC5jgAXZxx_17datafusion_common10hash_utils20hash_array_primitiveNtNtCs33DBFqlTBxO_11arrow_array5types18DurationSecondTypeNtNtCsRVR2D38dYC_8foldhash4fast10FixedStateECsg1h6yN5JQqd_7hashasm
	jmp	.LBB75_721
.LBB75_132:
	mov	rdi, r13
	call	qword ptr [rbp + 32]
	mov	rbx, rax
	lea	rdi, [rsp + 144]
	mov	rsi, rax
	call	qword ptr [rdx + 24]
	movdqu	xmm0, xmmword ptr [rsp + 144]
	pcmpeqb	xmm0, xmmword ptr [rip + .LCPI75_11]
	pmovmskb	eax, xmm0
	cmp	eax, 65535
	jne	.LBB75_734
	mov	rdx, qword ptr [rsp + 24]
	mov	qword ptr [rsp + 784], rdx
	mov	rax, qword ptr [rbx + 40]
	mov	rcx, rax
	shr	rcx, 3
	mov	qword ptr [rsp + 656], rcx
	cmp	rdx, rcx
	jne	.LBB75_6
	cmp	qword ptr [rbx + 48], 0
	je	.LBB75_136
	cmp	qword ptr [rbx + 88], 0
	je	.LBB75_136
	mov	rsi, rbx
	add	rsi, 48
	test	r12b, r12b
	je	.LBB75_144
	movabs	r15, 589684135938649225
	lea	rdi, [rsp + 144]
	call	qword ptr [rip + _RNvMNtNtCs7SjCkj1olcZ_12arrow_buffer6buffer4nullNtB2_10NullBuffer13valid_indices@GOTPCREL]
	mov	r9, qword ptr [rsp + 144]
	mov	rcx, qword ptr [rsp + 152]
	mov	r12, qword ptr [rsp + 160]
	mov	rsi, qword ptr [rsp + 168]
	mov	rdi, qword ptr [rsp + 176]
	mov	r8, qword ptr [rsp + 184]
	mov	r13, qword ptr [rsp + 192]
	mov	r10, qword ptr [rsp + 200]
	mov	r11, qword ptr [rbx + 32]
	movabs	rbx, -4563226453097033507
	mov	r14, qword ptr [rsp + 8]
	jmp	.LBB75_157
	.p2align	4
.LBB75_160:
	mov	r13, rsi
	xor	r12d, r12d
	add	r10, 64
.LBB75_157:
	test	r13, r13
	jne	.LBB75_167
	cmp	r12, -2
	je	.LBB75_164
	cmp	r12, 1
	je	.LBB75_160
	test	rdi, rdi
	je	.LBB75_164
	cmp	rdi, r8
	je	.LBB75_164
	mov	r13, qword ptr [rdi]
	add	rdi, 8
	mov	r12, -1
	add	r10, 64
	jmp	.LBB75_157
	.p2align	4
.LBB75_164:
	dec	r9
	cmp	r9, -3
	ja	.LBB75_721
	mov	r12, -2
	mov	r13, rcx
	xor	r9d, r9d
	add	r10, 64
	jmp	.LBB75_157
.LBB75_167:
	rep		bsf	rbp, r13
	add	rbp, r10
	mov	rax, qword ptr [r11 + 8*rbp]
	lea	rdx, [r13 - 1]
	and	r13, rdx
	xor	rax, qword ptr [r14 + 8*rbp]
	xor	rax, r15
	mul	rbx
	xor	rdx, rax
	mov	qword ptr [r14 + 8*rbp], rdx
	jmp	.LBB75_157
.LBB75_2:
	mov	rdi, r13
	call	rbx
	movzx	ecx, byte ptr [rax]
	cmp	rcx, 40
	ja	.LBB75_728
	lea	rdx, [rip + .LJTI75_2]
	movsxd	rcx, dword ptr [rdx + 4*rcx]
	add	rcx, rdx
	jmp	rcx
.LBB75_648:
	mov	rax, qword ptr [rsp + 16]
	mov	rdi, qword ptr [rax]
	movzx	ecx, r12b
	mov	rsi, qword ptr [rsp + 8]
	mov	rdx, qword ptr [rsp + 24]
	call	_RINvNtCspC5jgAXZxx_17datafusion_common10hash_utils9hash_nullNtNtCsRVR2D38dYC_8foldhash4fast10FixedStateECsg1h6yN5JQqd_7hashasm
	jmp	.LBB75_721
.LBB75_90:
	mov	rdi, r13
	call	qword ptr [rbp + 32]
	mov	rbx, rax
	lea	rdi, [rsp + 144]
	mov	rsi, rax
	call	qword ptr [rdx + 24]
	movdqu	xmm0, xmmword ptr [rsp + 144]
	pcmpeqb	xmm0, xmmword ptr [rip + .LCPI75_12]
	pmovmskb	eax, xmm0
	cmp	eax, 65535
	jne	.LBB75_734
	mov	rdx, qword ptr [rsp + 24]
	mov	qword ptr [rsp + 784], rdx
	mov	rax, qword ptr [rbx + 40]
	mov	rcx, rax
	shr	rcx, 2
	mov	qword ptr [rsp + 656], rcx
	cmp	rdx, rcx
	jne	.LBB75_6
	cmp	qword ptr [rbx + 48], 0
	je	.LBB75_94
	cmp	qword ptr [rbx + 88], 0
	je	.LBB75_94
	mov	rsi, rbx
	add	rsi, 48
	test	r12b, r12b
	je	.LBB75_102
	movabs	r15, 589684135938649225
	lea	rdi, [rsp + 144]
	call	qword ptr [rip + _RNvMNtNtCs7SjCkj1olcZ_12arrow_buffer6buffer4nullNtB2_10NullBuffer13valid_indices@GOTPCREL]
	mov	r9, qword ptr [rsp + 144]
	mov	rcx, qword ptr [rsp + 152]
	mov	r12, qword ptr [rsp + 160]
	mov	rsi, qword ptr [rsp + 168]
	mov	rdi, qword ptr [rsp + 176]
	mov	r8, qword ptr [rsp + 184]
	mov	r13, qword ptr [rsp + 192]
	mov	r10, qword ptr [rsp + 200]
	mov	r11, qword ptr [rbx + 32]
	movabs	rbx, -4563226453097033507
	mov	r14, qword ptr [rsp + 8]
	jmp	.LBB75_115
	.p2align	4
.LBB75_118:
	mov	r13, rsi
	xor	r12d, r12d
	add	r10, 64
.LBB75_115:
	test	r13, r13
	jne	.LBB75_125
	cmp	r12, -2
	je	.LBB75_122
	cmp	r12, 1
	je	.LBB75_118
	test	rdi, rdi
	je	.LBB75_122
	cmp	rdi, r8
	je	.LBB75_122
	mov	r13, qword ptr [rdi]
	add	rdi, 8
	mov	r12, -1
	add	r10, 64
	jmp	.LBB75_115
	.p2align	4
.LBB75_122:
	dec	r9
	cmp	r9, -3
	ja	.LBB75_721
	mov	r12, -2
	mov	r13, rcx
	xor	r9d, r9d
	add	r10, 64
	jmp	.LBB75_115
.LBB75_125:
	rep		bsf	rbp, r13
	add	rbp, r10
	mov	eax, dword ptr [r11 + 4*rbp]
	lea	rdx, [r13 - 1]
	and	r13, rdx
	xor	rax, qword ptr [r14 + 8*rbp]
	xor	rax, r15
	mul	rbx
	xor	rdx, rax
	mov	qword ptr [r14 + 8*rbp], rdx
	jmp	.LBB75_115
.LBB75_258:
	mov	rdi, r13
	call	qword ptr [rbp + 32]
	mov	rbx, rax
	lea	rdi, [rsp + 144]
	mov	rsi, rax
	call	qword ptr [rdx + 24]
	movdqu	xmm0, xmmword ptr [rsp + 144]
	pcmpeqb	xmm0, xmmword ptr [rip + .LCPI75_8]
	pmovmskb	eax, xmm0
	cmp	eax, 65535
	jne	.LBB75_734
	mov	rdx, qword ptr [rsp + 24]
	mov	qword ptr [rsp + 784], rdx
	mov	rax, qword ptr [rbx + 40]
	mov	rcx, rax
	shr	rcx, 2
	mov	qword ptr [rsp + 656], rcx
	cmp	rdx, rcx
	jne	.LBB75_6
	cmp	qword ptr [rbx + 48], 0
	je	.LBB75_262
	cmp	qword ptr [rbx + 88], 0
	je	.LBB75_262
	mov	rsi, rbx
	add	rsi, 48
	test	r12b, r12b
	je	.LBB75_270
	movabs	r15, 589684135938649225
	lea	rdi, [rsp + 144]
	call	qword ptr [rip + _RNvMNtNtCs7SjCkj1olcZ_12arrow_buffer6buffer4nullNtB2_10NullBuffer13valid_indices@GOTPCREL]
	mov	r9, qword ptr [rsp + 144]
	mov	rcx, qword ptr [rsp + 152]
	mov	r12, qword ptr [rsp + 160]
	mov	rsi, qword ptr [rsp + 168]
	mov	rdi, qword ptr [rsp + 176]
	mov	r8, qword ptr [rsp + 184]
	mov	r13, qword ptr [rsp + 192]
	mov	r10, qword ptr [rsp + 200]
	mov	r11, qword ptr [rbx + 32]
	movabs	rbx, -4563226453097033507
	mov	r14, qword ptr [rsp + 8]
	jmp	.LBB75_283
	.p2align	4
.LBB75_286:
	mov	r13, rsi
	xor	r12d, r12d
	add	r10, 64
.LBB75_283:
	test	r13, r13
	jne	.LBB75_293
	cmp	r12, -2
	je	.LBB75_290
	cmp	r12, 1
	je	.LBB75_286
	test	rdi, rdi
	je	.LBB75_290
	cmp	rdi, r8
	je	.LBB75_290
	mov	r13, qword ptr [rdi]
	add	rdi, 8
	mov	r12, -1
	add	r10, 64
	jmp	.LBB75_283
	.p2align	4
.LBB75_290:
	dec	r9
	cmp	r9, -3
	ja	.LBB75_721
	mov	r12, -2
	mov	r13, rcx
	xor	r9d, r9d
	add	r10, 64
	jmp	.LBB75_283
.LBB75_293:
	rep		bsf	rbp, r13
	add	rbp, r10
	mov	eax, dword ptr [r11 + 4*rbp]
	lea	rdx, [r13 - 1]
	and	r13, rdx
	xor	rax, qword ptr [r14 + 8*rbp]
	xor	rax, r15
	mul	rbx
	xor	rdx, rax
	mov	qword ptr [r14 + 8*rbp], rdx
	jmp	.LBB75_283
.LBB75_300:
	mov	rdi, r13
	call	qword ptr [rbp + 32]
	mov	rbx, rax
	lea	rdi, [rsp + 144]
	mov	rsi, rax
	call	qword ptr [rdx + 24]
	movdqu	xmm0, xmmword ptr [rsp + 144]
	pcmpeqb	xmm0, xmmword ptr [rip + .LCPI75_7]
	pmovmskb	eax, xmm0
	cmp	eax, 65535
	jne	.LBB75_734
	mov	rdx, qword ptr [rsp + 24]
	mov	qword ptr [rsp + 784], rdx
	mov	rax, qword ptr [rbx + 40]
	mov	rcx, rax
	shr	rcx, 3
	mov	qword ptr [rsp + 656], rcx
	cmp	rdx, rcx
	jne	.LBB75_6
	cmp	qword ptr [rbx + 48], 0
	je	.LBB75_304
	cmp	qword ptr [rbx + 88], 0
	je	.LBB75_304
	mov	rsi, rbx
	add	rsi, 48
	test	r12b, r12b
	je	.LBB75_312
	movabs	r15, 589684135938649225
	lea	rdi, [rsp + 144]
	call	qword ptr [rip + _RNvMNtNtCs7SjCkj1olcZ_12arrow_buffer6buffer4nullNtB2_10NullBuffer13valid_indices@GOTPCREL]
	mov	r9, qword ptr [rsp + 144]
	mov	rcx, qword ptr [rsp + 152]
	mov	r12, qword ptr [rsp + 160]
	mov	rsi, qword ptr [rsp + 168]
	mov	rdi, qword ptr [rsp + 176]
	mov	r8, qword ptr [rsp + 184]
	mov	r13, qword ptr [rsp + 192]
	mov	r10, qword ptr [rsp + 200]
	mov	r11, qword ptr [rbx + 32]
	movabs	rbx, -4563226453097033507
	mov	r14, qword ptr [rsp + 8]
	jmp	.LBB75_325
	.p2align	4
.LBB75_328:
	mov	r13, rsi
	xor	r12d, r12d
	add	r10, 64
.LBB75_325:
	test	r13, r13
	jne	.LBB75_335
	cmp	r12, -2
	je	.LBB75_332
	cmp	r12, 1
	je	.LBB75_328
	test	rdi, rdi
	je	.LBB75_332
	cmp	rdi, r8
	je	.LBB75_332
	mov	r13, qword ptr [rdi]
	add	rdi, 8
	mov	r12, -1
	add	r10, 64
	jmp	.LBB75_325
	.p2align	4
.LBB75_332:
	dec	r9
	cmp	r9, -3
	ja	.LBB75_721
	mov	r12, -2
	mov	r13, rcx
	xor	r9d, r9d
	add	r10, 64
	jmp	.LBB75_325
.LBB75_335:
	rep		bsf	rbp, r13
	add	rbp, r10
	mov	rax, qword ptr [r11 + 8*rbp]
	lea	rdx, [r13 - 1]
	and	r13, rdx
	xor	rax, qword ptr [r14 + 8*rbp]
	xor	rax, r15
	mul	rbx
	xor	rdx, rax
	mov	qword ptr [r14 + 8*rbp], rdx
	jmp	.LBB75_325
.LBB75_514:
	mov	rdi, r13
	call	qword ptr [rbp + 32]
	mov	rbx, rax
	lea	rdi, [rsp + 144]
	mov	rsi, rax
	call	qword ptr [rdx + 24]
	movdqu	xmm0, xmmword ptr [rsp + 144]
	pcmpeqb	xmm0, xmmword ptr [rip + .LCPI75_2]
	pmovmskb	eax, xmm0
	cmp	eax, 65535
	jne	.LBB75_734
	mov	rdx, qword ptr [rsp + 24]
	mov	qword ptr [rsp + 784], rdx
	mov	rax, qword ptr [rbx + 40]
	mov	rcx, rax
	shr	rcx, 3
	mov	qword ptr [rsp + 656], rcx
	cmp	rdx, rcx
	jne	.LBB75_6
	cmp	qword ptr [rbx + 48], 0
	je	.LBB75_518
	cmp	qword ptr [rbx + 88], 0
	je	.LBB75_518
	mov	rsi, rbx
	add	rsi, 48
	test	r12b, r12b
	je	.LBB75_526
	movabs	r15, 589684135938649225
	lea	rdi, [rsp + 144]
	call	qword ptr [rip + _RNvMNtNtCs7SjCkj1olcZ_12arrow_buffer6buffer4nullNtB2_10NullBuffer13valid_indices@GOTPCREL]
	mov	r9, qword ptr [rsp + 144]
	mov	rcx, qword ptr [rsp + 152]
	mov	r12, qword ptr [rsp + 160]
	mov	rsi, qword ptr [rsp + 168]
	mov	rdi, qword ptr [rsp + 176]
	mov	r8, qword ptr [rsp + 184]
	mov	r13, qword ptr [rsp + 192]
	mov	r10, qword ptr [rsp + 200]
	mov	r11, qword ptr [rbx + 32]
	movabs	rbx, -4563226453097033507
	mov	r14, qword ptr [rsp + 8]
	jmp	.LBB75_539
	.p2align	4
.LBB75_542:
	mov	r13, rsi
	xor	r12d, r12d
	add	r10, 64
.LBB75_539:
	test	r13, r13
	jne	.LBB75_549
	cmp	r12, -2
	je	.LBB75_546
	cmp	r12, 1
	je	.LBB75_542
	test	rdi, rdi
	je	.LBB75_546
	cmp	rdi, r8
	je	.LBB75_546
	mov	r13, qword ptr [rdi]
	add	rdi, 8
	mov	r12, -1
	add	r10, 64
	jmp	.LBB75_539
	.p2align	4
.LBB75_546:
	dec	r9
	cmp	r9, -3
	ja	.LBB75_721
	mov	r12, -2
	mov	r13, rcx
	xor	r9d, r9d
	add	r10, 64
	jmp	.LBB75_539
.LBB75_549:
	rep		bsf	rbp, r13
	add	rbp, r10
	mov	rax, qword ptr [r11 + 8*rbp]
	lea	rdx, [r13 - 1]
	and	r13, rdx
	xor	rax, qword ptr [r14 + 8*rbp]
	xor	rax, r15
	mul	rbx
	xor	rdx, rax
	mov	qword ptr [r14 + 8*rbp], rdx
	jmp	.LBB75_539
.LBB75_556:
	mov	rdi, r13
	call	qword ptr [rbp + 32]
	mov	rbx, rax
	lea	rdi, [rsp + 144]
	mov	rsi, rax
	call	qword ptr [rdx + 24]
	movdqu	xmm0, xmmword ptr [rsp + 144]
	pcmpeqb	xmm0, xmmword ptr [rip + .LCPI75_1]
	pmovmskb	eax, xmm0
	cmp	eax, 65535
	jne	.LBB75_734
	mov	rdx, qword ptr [rsp + 24]
	mov	qword ptr [rsp + 784], rdx
	mov	rax, qword ptr [rbx + 40]
	mov	rcx, rax
	shr	rcx, 4
	mov	qword ptr [rsp + 656], rcx
	cmp	rdx, rcx
	jne	.LBB75_6
	cmp	qword ptr [rbx + 48], 0
	je	.LBB75_560
	cmp	qword ptr [rbx + 88], 0
	je	.LBB75_560
	mov	rsi, rbx
	add	rsi, 48
	movabs	r15, -4563226453097033507
	test	r12b, r12b
	je	.LBB75_568
	movabs	r12, 589684135938649225
	lea	rdi, [rsp + 144]
	call	qword ptr [rip + _RNvMNtNtCs7SjCkj1olcZ_12arrow_buffer6buffer4nullNtB2_10NullBuffer13valid_indices@GOTPCREL]
	mov	r9, qword ptr [rsp + 144]
	mov	rcx, qword ptr [rsp + 152]
	mov	r13, qword ptr [rsp + 160]
	mov	rsi, qword ptr [rsp + 168]
	mov	rdi, qword ptr [rsp + 176]
	mov	r8, qword ptr [rsp + 184]
	mov	rbp, qword ptr [rsp + 192]
	mov	r10, qword ptr [rsp + 200]
	mov	r11, qword ptr [rbx + 32]
	mov	r14, qword ptr [rsp + 8]
	jmp	.LBB75_581
	.p2align	4
.LBB75_584:
	mov	rbp, rsi
	xor	r13d, r13d
	add	r10, 64
.LBB75_581:
	test	rbp, rbp
	jne	.LBB75_591
	cmp	r13, -2
	je	.LBB75_588
	cmp	r13, 1
	je	.LBB75_584
	test	rdi, rdi
	je	.LBB75_588
	cmp	rdi, r8
	je	.LBB75_588
	mov	rbp, qword ptr [rdi]
	add	rdi, 8
	mov	r13, -1
	add	r10, 64
	jmp	.LBB75_581
	.p2align	4
.LBB75_588:
	dec	r9
	cmp	r9, -3
	ja	.LBB75_721
	mov	r13, -2
	mov	rbp, rcx
	xor	r9d, r9d
	add	r10, 64
	jmp	.LBB75_581
.LBB75_591:
	rep		bsf	rbx, rbp
	add	rbx, r10
	mov	rax, rbx
	shl	rax, 4
	lea	rdx, [rbp - 1]
	and	rbp, rdx
	mov	rdx, qword ptr [r14 + 8*rbx]
	xor	rdx, qword ptr [r11 + rax]
	xor	rdx, r12
	mov	rax, qword ptr [r11 + rax + 8]
	xor	rax, r15
	mul	rdx
	xor	rdx, rax
	mov	qword ptr [r14 + 8*rbx], rdx
	jmp	.LBB75_581
.LBB75_384:
	mov	rdi, r13
	call	qword ptr [rbp + 32]
	mov	rbx, rax
	lea	rdi, [rsp + 144]
	mov	rsi, rax
	call	qword ptr [rdx + 24]
	movdqu	xmm0, xmmword ptr [rsp + 144]
	pcmpeqb	xmm0, xmmword ptr [rip + .LCPI75_5]
	pmovmskb	eax, xmm0
	cmp	eax, 65535
	jne	.LBB75_734
	mov	rdx, qword ptr [rsp + 24]
	mov	qword ptr [rsp + 784], rdx
	mov	rax, qword ptr [rbx + 40]
	mov	rcx, rax
	shr	rcx, 2
	mov	qword ptr [rsp + 656], rcx
	cmp	rdx, rcx
	jne	.LBB75_6
	cmp	qword ptr [rbx + 48], 0
	je	.LBB75_388
	cmp	qword ptr [rbx + 88], 0
	je	.LBB75_388
	mov	rsi, rbx
	add	rsi, 48
	lea	rdi, [rsp + 144]
	test	r12b, r12b
	je	.LBB75_396
	call	qword ptr [rip + _RNvMNtNtCs7SjCkj1olcZ_12arrow_buffer6buffer4nullNtB2_10NullBuffer13valid_indices@GOTPCREL]
	mov	r9, qword ptr [rsp + 144]
	mov	rcx, qword ptr [rsp + 152]
	mov	r13, qword ptr [rsp + 160]
	mov	rsi, qword ptr [rsp + 168]
	mov	rdi, qword ptr [rsp + 176]
	mov	r8, qword ptr [rsp + 184]
	mov	rbp, qword ptr [rsp + 192]
	mov	r10, qword ptr [rsp + 200]
	mov	r11, qword ptr [rbx + 32]
	mov	ebx, 2147483648
	mov	r14, qword ptr [rsp + 8]
	jmp	.LBB75_409
	.p2align	4
.LBB75_412:
	mov	rbp, rsi
	xor	r13d, r13d
	add	r10, 64
.LBB75_409:
	test	rbp, rbp
	jne	.LBB75_419
	cmp	r13, -2
	je	.LBB75_416
	cmp	r13, 1
	je	.LBB75_412
	test	rdi, rdi
	je	.LBB75_416
	cmp	rdi, r8
	je	.LBB75_416
	mov	rbp, qword ptr [rdi]
	add	rdi, 8
	mov	r13, -1
	add	r10, 64
	jmp	.LBB75_409
	.p2align	4
.LBB75_416:
	dec	r9
	cmp	r9, -3
	ja	.LBB75_721
	mov	r13, -2
	mov	rbp, rcx
	xor	r9d, r9d
	add	r10, 64
	jmp	.LBB75_409
.LBB75_419:
	rep		bsf	r15, rbp
	add	r15, r10
	mov	edx, dword ptr [r11 + 4*r15]
	lea	rax, [rbp - 1]
	and	rbp, rax
	mov	r12, qword ptr [r14 + 8*r15]
	movabs	rax, 589684135938649225
	xor	r12, rax
	rol	r12, 60
	cmp	rdx, rbx
	mov	eax, 0
	cmovne	rax, rdx
	xor	r12, rax
	movabs	rdx, 4577018097722394903
	xor	rax, rdx
	mul	r12
	xor	rdx, rax
	mov	qword ptr [r14 + 8*r15], rdx
	jmp	.LBB75_409
.LBB75_342:
	mov	rdi, r13
	call	qword ptr [rbp + 32]
	mov	rbx, rax
	lea	rdi, [rsp + 144]
	mov	rsi, rax
	call	qword ptr [rdx + 24]
	movdqu	xmm0, xmmword ptr [rsp + 144]
	pcmpeqb	xmm0, xmmword ptr [rip + .LCPI75_6]
	pmovmskb	eax, xmm0
	cmp	eax, 65535
	jne	.LBB75_734
	mov	rdx, qword ptr [rsp + 24]
	mov	qword ptr [rsp + 784], rdx
	mov	rax, qword ptr [rbx + 40]
	mov	rcx, rax
	shr	rcx
	mov	qword ptr [rsp + 656], rcx
	cmp	rdx, rcx
	jne	.LBB75_6
	cmp	qword ptr [rbx + 48], 0
	je	.LBB75_346
	cmp	qword ptr [rbx + 88], 0
	je	.LBB75_346
	mov	rsi, rbx
	add	rsi, 48
	lea	rdi, [rsp + 144]
	test	r12b, r12b
	je	.LBB75_354
	call	qword ptr [rip + _RNvMNtNtCs7SjCkj1olcZ_12arrow_buffer6buffer4nullNtB2_10NullBuffer13valid_indices@GOTPCREL]
	mov	r9, qword ptr [rsp + 144]
	mov	rcx, qword ptr [rsp + 152]
	mov	r13, qword ptr [rsp + 160]
	mov	rsi, qword ptr [rsp + 168]
	mov	rdi, qword ptr [rsp + 176]
	mov	r8, qword ptr [rsp + 184]
	mov	rbp, qword ptr [rsp + 192]
	mov	r10, qword ptr [rsp + 200]
	mov	r11, qword ptr [rbx + 32]
	mov	r14, qword ptr [rsp + 8]
	jmp	.LBB75_367
	.p2align	4
.LBB75_370:
	mov	rbp, rsi
	xor	r13d, r13d
	add	r10, 64
.LBB75_367:
	test	rbp, rbp
	jne	.LBB75_377
	cmp	r13, -2
	je	.LBB75_374
	cmp	r13, 1
	je	.LBB75_370
	test	rdi, rdi
	je	.LBB75_374
	cmp	rdi, r8
	je	.LBB75_374
	mov	rbp, qword ptr [rdi]
	add	rdi, 8
	mov	r13, -1
	add	r10, 64
	jmp	.LBB75_367
	.p2align	4
.LBB75_374:
	dec	r9
	cmp	r9, -3
	ja	.LBB75_721
	mov	r13, -2
	mov	rbp, rcx
	xor	r9d, r9d
	add	r10, 64
	jmp	.LBB75_367
.LBB75_377:
	rep		bsf	r15, rbp
	add	r15, r10
	lea	rax, [rbp - 1]
	and	rbp, rax
	mov	rdx, qword ptr [r14 + 8*r15]
	movabs	rax, 589684135938649225
	xor	rdx, rax
	movzx	eax, word ptr [r11 + 2*r15]
	cmp	eax, 32768
	mov	ebx, 0
	cmove	eax, ebx
	mov	r12d, eax
	shr	r12d, 8
	rol	rdx, 62
	movzx	ebx, al
	xor	rbx, rdx
	and	eax, 65280
	or	eax, r12d
	movabs	rdx, 4577018097722394903
	xor	rax, rdx
	mul	rbx
	xor	rdx, rax
	mov	qword ptr [r14 + 8*r15], rdx
	jmp	.LBB75_367
.LBB75_598:
	mov	rdi, r13
	call	qword ptr [rbp + 32]
	mov	rbx, rax
	lea	rdi, [rsp + 144]
	mov	rsi, rax
	call	qword ptr [rdx + 24]
	movdqu	xmm0, xmmword ptr [rsp + 144]
	pcmpeqb	xmm0, xmmword ptr [rip + .LCPI75_0]
	pmovmskb	eax, xmm0
	cmp	eax, 65535
	jne	.LBB75_734
	mov	rdx, qword ptr [rsp + 24]
	mov	qword ptr [rsp + 912], rdx
	mov	rax, qword ptr [rbx + 40]
	mov	rcx, rax
	shr	rcx, 5
	mov	qword ptr [rsp + 920], rcx
	cmp	rdx, rcx
	jne	.LBB75_600
	cmp	qword ptr [rbx + 48], 0
	je	.LBB75_603
	cmp	qword ptr [rbx + 88], 0
	je	.LBB75_603
	mov	rsi, rbx
	add	rsi, 48
	test	r12b, r12b
	je	.LBB75_613
	lea	rdi, [rsp + 144]
	call	qword ptr [rip + _RNvMNtNtCs7SjCkj1olcZ_12arrow_buffer6buffer4nullNtB2_10NullBuffer13valid_indices@GOTPCREL]
	mov	r9, qword ptr [rsp + 144]
	mov	rcx, qword ptr [rsp + 152]
	mov	r13, qword ptr [rsp + 160]
	mov	rdx, qword ptr [rsp + 168]
	mov	rdi, qword ptr [rsp + 176]
	mov	r8, qword ptr [rsp + 184]
	mov	rbp, qword ptr [rsp + 192]
	mov	r10, qword ptr [rsp + 200]
	mov	rsi, qword ptr [rbx + 32]
	mov	qword ptr [rsp + 24], rsi
	mov	qword ptr [rsp + 16], rdx
	mov	r14, qword ptr [rsp + 8]
	movabs	r11, -4563226453097033507
	jmp	.LBB75_626
	.p2align	4
.LBB75_629:
	mov	rbp, rdx
	xor	r13d, r13d
	add	r10, 64
.LBB75_626:
	test	rbp, rbp
	jne	.LBB75_636
	cmp	r13, -2
	je	.LBB75_633
	cmp	r13, 1
	je	.LBB75_629
	test	rdi, rdi
	je	.LBB75_633
	cmp	rdi, r8
	je	.LBB75_633
	mov	rbp, qword ptr [rdi]
	add	rdi, 8
	mov	r13, -1
	add	r10, 64
	jmp	.LBB75_626
	.p2align	4
.LBB75_633:
	dec	r9
	cmp	r9, -3
	ja	.LBB75_721
	mov	r13, -2
	mov	rbp, rcx
	xor	r9d, r9d
	add	r10, 64
	jmp	.LBB75_626
.LBB75_636:
	rep		bsf	r12, rbp
	add	r12, r10
	mov	rbx, r12
	shl	rbx, 5
	mov	rdx, qword ptr [r14 + 8*r12]
	mov	rsi, rcx
	mov	rcx, qword ptr [rsp + 24]
	xor	rdx, qword ptr [rcx + rbx]
	movabs	rax, 589684135938649225
	xor	rdx, rax
	mov	rax, qword ptr [rcx + rbx + 8]
	xor	rax, r11
	mul	rdx
	lea	r15, [rbp - 1]
	xor	rdx, qword ptr [rcx + rbx + 16]
	and	rbp, r15
	xor	rax, rdx
	mov	rdx, qword ptr [rcx + rbx + 24]
	mov	rcx, rsi
	xor	rdx, r11
	mul	rdx
	xor	rdx, rax
	mov	qword ptr [r14 + 8*r12], rdx
	mov	rdx, qword ptr [rsp + 16]
	jmp	.LBB75_626
.LBB75_476:
	mov	rcx, qword ptr [rbx + 32]
	test	r12b, r12b
	je	.LBB75_477
	movabs	r12, 589684135938649225
	mov	rdx, qword ptr [rsp + 24]
	mov	esi, edx
	and	esi, 7
	mov	qword ptr [rsp + 40], rsi
	shr	rax, 5
	shr	rdx, 3
	cmp	rax, rdx
	cmovb	rdx, rax
	mov	qword ptr [rsp + 16], rdx
	test	rdx, rdx
	je	.LBB75_511
	shl	qword ptr [rsp + 16], 5
	xor	r13d, r13d
	movabs	rbp, -4563226453097033507
	mov	r14, qword ptr [rsp + 8]
	.p2align	4
.LBB75_510:
	mov	eax, dword ptr [rcx + r13]
	mov	esi, dword ptr [rcx + r13 + 4]
	xor	rax, qword ptr [r14 + 2*r13]
	xor	rax, r12
	mul	rbp
	mov	r15, rdx
	xor	r15, rax
	xor	rsi, qword ptr [r14 + 2*r13 + 8]
	xor	rsi, r12
	mov	rax, rsi
	mul	rbp
	mov	r8, rax
	mov	rsi, rdx
	mov	eax, dword ptr [rcx + r13 + 8]
	xor	rax, qword ptr [r14 + 2*r13 + 16]
	xor	rax, r12
	mul	rbp
	mov	rdi, rdx
	xor	rsi, r8
	xor	rdi, rax
	mov	eax, dword ptr [rcx + r13 + 12]
	xor	rax, qword ptr [r14 + 2*r13 + 24]
	xor	rax, r12
	mul	rbp
	mov	r10, rax
	mov	r8, rdx
	mov	eax, dword ptr [rcx + r13 + 16]
	xor	rax, qword ptr [r14 + 2*r13 + 32]
	xor	rax, r12
	mul	rbp
	mov	r9, rdx
	xor	r8, r10
	xor	r9, rax
	mov	eax, dword ptr [rcx + r13 + 20]
	xor	rax, qword ptr [r14 + 2*r13 + 40]
	xor	rax, r12
	mul	rbp
	mov	r11, rax
	mov	r10, rdx
	mov	eax, dword ptr [rcx + r13 + 24]
	xor	rax, qword ptr [r14 + 2*r13 + 48]
	xor	rax, r12
	mul	rbp
	mov	rbx, rdx
	xor	r10, r11
	xor	rbx, rax
	mov	eax, dword ptr [rcx + r13 + 28]
	xor	rax, qword ptr [r14 + 2*r13 + 56]
	xor	rax, r12
	mul	rbp
	xor	rdx, rax
	mov	qword ptr [r14 + 2*r13], r15
	mov	qword ptr [r14 + 2*r13 + 8], rsi
	mov	qword ptr [r14 + 2*r13 + 16], rdi
	mov	qword ptr [r14 + 2*r13 + 24], r8
	mov	qword ptr [r14 + 2*r13 + 32], r9
	mov	qword ptr [r14 + 2*r13 + 40], r10
	mov	qword ptr [r14 + 2*r13 + 48], rbx
	mov	qword ptr [r14 + 2*r13 + 56], rdx
	add	r13, 32
	cmp	qword ptr [rsp + 16], r13
	jne	.LBB75_510
.LBB75_511:
	mov	r9, qword ptr [rsp + 40]
	test	r9, r9
	je	.LBB75_721
	movabs	rax, 1152921504606846960
	or	rax, 8
	mov	rdx, qword ptr [rsp + 24]
	and	rdx, rax
	lea	rcx, [rcx + 4*rdx]
	mov	rax, qword ptr [rsp + 8]
	lea	rsi, [rax + 8*rdx]
	xor	r8d, r8d
	movabs	rdi, -4563226453097033507
	.p2align	4
.LBB75_513:
	mov	eax, dword ptr [rcx + 4*r8]
	xor	rax, qword ptr [rsi + 8*r8]
	xor	rax, r12
	mul	rdi
	xor	rdx, rax
	mov	qword ptr [rsi + 8*r8], rdx
	inc	r8
	cmp	r9, r8
	jne	.LBB75_513
	jmp	.LBB75_721
.LBB75_430:
	mov	r13, qword ptr [rbx + 32]
	test	r12b, r12b
	je	.LBB75_431
	movabs	r12, 4577018097722394903
	mov	rcx, qword ptr [rsp + 24]
	movabs	r15, 589684135938649225
	mov	edx, ecx
	and	edx, 7
	mov	qword ptr [rsp + 48], rdx
	shr	rax, 6
	shr	rcx, 3
	cmp	rax, rcx
	cmovb	rcx, rax
	mov	qword ptr [rsp + 40], rcx
	test	rcx, rcx
	je	.LBB75_465
	shl	qword ptr [rsp + 40], 6
	xor	ebp, ebp
	mov	r14, qword ptr [rsp + 8]
	.p2align	4
.LBB75_464:
	mov	rcx, qword ptr [r14 + rbp]
	xor	rcx, r15
	mov	rax, qword ptr [r13 + rbp]
	mov	rsi, qword ptr [r13 + rbp + 8]
	mov	rdx, rax
	neg	rdx
	mov	edx, 0
	cmovo	rax, rdx
	xor	edi, edi
	rol	rcx, 56
	xor	rcx, rax
	xor	rax, r12
	mul	rcx
	xor	rdx, rax
	mov	qword ptr [rsp + 16], rdx
	mov	rdx, qword ptr [r14 + rbp + 8]
	xor	rdx, r15
	mov	rax, rsi
	neg	rax
	cmovo	rsi, rdi
	xor	ecx, ecx
	rol	rdx, 56
	xor	rdx, rsi
	xor	rsi, r12
	mov	rax, rsi
	mul	rdx
	mov	rsi, rdx
	xor	rsi, rax
	mov	rax, qword ptr [r13 + rbp + 16]
	mov	rdx, qword ptr [r14 + rbp + 16]
	xor	rdx, r15
	mov	rdi, rax
	neg	rdi
	cmovo	rax, rcx
	rol	rdx, 56
	xor	rdx, rax
	xor	rax, r12
	mul	rdx
	mov	rdi, rdx
	xor	rdi, rax
	mov	rax, qword ptr [r13 + rbp + 24]
	mov	rdx, qword ptr [r14 + rbp + 24]
	xor	rdx, r15
	mov	r8, rax
	neg	r8
	cmovo	rax, rcx
	rol	rdx, 56
	xor	rdx, rax
	xor	rax, r12
	mul	rdx
	mov	r8, rdx
	xor	r8, rax
	mov	rax, qword ptr [r13 + rbp + 32]
	mov	rdx, qword ptr [r14 + rbp + 32]
	xor	rdx, r15
	mov	r9, rax
	neg	r9
	cmovo	rax, rcx
	rol	rdx, 56
	xor	rdx, rax
	xor	rax, r12
	mul	rdx
	mov	r9, rdx
	xor	r9, rax
	mov	rax, qword ptr [r13 + rbp + 40]
	mov	rdx, qword ptr [r14 + rbp + 40]
	xor	rdx, r15
	mov	r10, rax
	neg	r10
	cmovo	rax, rcx
	rol	rdx, 56
	xor	rdx, rax
	xor	rax, r12
	mul	rdx
	mov	r10, rdx
	xor	r10, rax
	mov	rax, qword ptr [r13 + rbp + 48]
	mov	rdx, qword ptr [r14 + rbp + 48]
	xor	rdx, r15
	mov	r11, rax
	neg	r11
	cmovo	rax, rcx
	rol	rdx, 56
	xor	rdx, rax
	xor	rax, r12
	mul	rdx
	mov	r11, rdx
	xor	r11, rax
	mov	rbx, qword ptr [r13 + rbp + 56]
	mov	rdx, qword ptr [r14 + rbp + 56]
	xor	rdx, r15
	mov	rax, rbx
	neg	rax
	cmovo	rbx, rcx
	rol	rdx, 56
	xor	rdx, rbx
	xor	rbx, r12
	mov	rax, rbx
	mul	rdx
	xor	rdx, rax
	mov	rax, qword ptr [rsp + 16]
	mov	qword ptr [r14 + rbp], rax
	mov	qword ptr [r14 + rbp + 8], rsi
	mov	qword ptr [r14 + rbp + 16], rdi
	mov	qword ptr [r14 + rbp + 24], r8
	mov	qword ptr [r14 + rbp + 32], r9
	mov	qword ptr [r14 + rbp + 40], r10
	mov	qword ptr [r14 + rbp + 48], r11
	mov	qword ptr [r14 + rbp + 56], rdx
	add	rbp, 64
	cmp	qword ptr [rsp + 40], rbp
	jne	.LBB75_464
.LBB75_465:
	mov	r10, qword ptr [rsp + 48]
	test	r10, r10
	je	.LBB75_721
	movabs	rax, 1152921504606846960
	or	rax, 8
	mov	rdx, qword ptr [rsp + 24]
	and	rdx, rax
	lea	rcx, [8*rdx]
	add	rcx, r13
	mov	rax, qword ptr [rsp + 8]
	lea	rsi, [rax + 8*rdx]
	xor	edi, edi
	.p2align	4
.LBB75_467:
	mov	rdx, qword ptr [rcx + 8*rdi]
	mov	rax, qword ptr [rsi + 8*rdi]
	xor	rax, r15
	mov	r8, rdx
	neg	r8
	mov	r8d, 0
	cmovno	r8, rdx
	rol	rax, 56
	xor	rax, r8
	xor	r8, r12
	mul	r8
	xor	rdx, rax
	mov	qword ptr [rsi + 8*rdi], rdx
	inc	rdi
	cmp	r10, rdi
	jne	.LBB75_467
	jmp	.LBB75_721
.LBB75_178:
	mov	rcx, qword ptr [rbx + 32]
	test	r12b, r12b
	je	.LBB75_179
	movabs	r12, 589684135938649225
	movabs	rsi, 1152921504606846960
	or	rsi, 8
	mov	rax, qword ptr [rsp + 24]
	and	rsi, rax
	mov	edx, eax
	and	edx, 7
	mov	qword ptr [rsp + 40], rdx
	test	rax, -8
	je	.LBB75_213
	xor	r13d, r13d
	movabs	rbp, -4563226453097033507
	mov	qword ptr [rsp + 16], rsi
	mov	r14, qword ptr [rsp + 8]
	.p2align	4
.LBB75_212:
	movzx	eax, byte ptr [rcx + r13]
	xor	rax, qword ptr [r14 + 8*r13]
	xor	rax, r12
	mul	rbp
	mov	r15, rdx
	xor	r15, rax
	movzx	eax, byte ptr [rcx + r13 + 1]
	xor	rax, qword ptr [r14 + 8*r13 + 8]
	xor	rax, r12
	mul	rbp
	mov	r8, rax
	mov	rsi, rdx
	movzx	eax, byte ptr [rcx + r13 + 2]
	xor	rax, qword ptr [r14 + 8*r13 + 16]
	xor	rax, r12
	mul	rbp
	mov	rdi, rdx
	xor	rsi, r8
	xor	rdi, rax
	movzx	eax, byte ptr [rcx + r13 + 3]
	xor	rax, qword ptr [r14 + 8*r13 + 24]
	xor	rax, r12
	mul	rbp
	mov	r10, rax
	mov	r8, rdx
	movzx	eax, byte ptr [rcx + r13 + 4]
	xor	rax, qword ptr [r14 + 8*r13 + 32]
	xor	rax, r12
	mul	rbp
	mov	r9, rdx
	xor	r8, r10
	xor	r9, rax
	movzx	eax, byte ptr [rcx + r13 + 5]
	xor	rax, qword ptr [r14 + 8*r13 + 40]
	xor	rax, r12
	mul	rbp
	mov	r11, rax
	mov	r10, rdx
	movzx	eax, byte ptr [rcx + r13 + 6]
	xor	rax, qword ptr [r14 + 8*r13 + 48]
	xor	rax, r12
	mul	rbp
	mov	rbx, rdx
	xor	r10, r11
	xor	rbx, rax
	movzx	eax, byte ptr [rcx + r13 + 7]
	xor	rax, qword ptr [r14 + 8*r13 + 56]
	xor	rax, r12
	mul	rbp
	xor	rdx, rax
	mov	qword ptr [r14 + 8*r13], r15
	mov	qword ptr [r14 + 8*r13 + 8], rsi
	mov	rsi, qword ptr [rsp + 16]
	mov	qword ptr [r14 + 8*r13 + 16], rdi
	mov	qword ptr [r14 + 8*r13 + 24], r8
	mov	qword ptr [r14 + 8*r13 + 32], r9
	mov	qword ptr [r14 + 8*r13 + 40], r10
	mov	qword ptr [r14 + 8*r13 + 48], rbx
	mov	qword ptr [r14 + 8*r13 + 56], rdx
	add	r13, 8
	cmp	rsi, r13
	jne	.LBB75_212
.LBB75_213:
	mov	r9, qword ptr [rsp + 40]
	test	r9, r9
	je	.LBB75_721
	add	rcx, rsi
	mov	rax, qword ptr [rsp + 8]
	lea	rsi, [rax + 8*rsi]
	xor	r8d, r8d
	movabs	rdi, -4563226453097033507
	.p2align	4
.LBB75_215:
	movzx	eax, byte ptr [rcx + r8]
	xor	rax, qword ptr [rsi + 8*r8]
	xor	rax, r12
	mul	rdi
	xor	rdx, rax
	mov	qword ptr [rsi + 8*r8], rdx
	inc	r8
	cmp	r9, r8
	jne	.LBB75_215
	jmp	.LBB75_721
.LBB75_52:
	mov	rcx, qword ptr [rbx + 32]
	test	r12b, r12b
	je	.LBB75_53
	movabs	r12, 589684135938649225
	mov	rdx, qword ptr [rsp + 24]
	mov	esi, edx
	and	esi, 7
	mov	qword ptr [rsp + 40], rsi
	shr	rax, 4
	shr	rdx, 3
	cmp	rax, rdx
	cmovb	rdx, rax
	mov	qword ptr [rsp + 16], rdx
	test	rdx, rdx
	je	.LBB75_87
	shl	qword ptr [rsp + 16], 4
	xor	r13d, r13d
	movabs	rbp, -4563226453097033507
	mov	r14, qword ptr [rsp + 8]
	.p2align	4
.LBB75_86:
	movzx	eax, word ptr [rcx + r13]
	xor	rax, qword ptr [r14 + 4*r13]
	xor	rax, r12
	mul	rbp
	mov	r15, rdx
	xor	r15, rax
	movzx	eax, word ptr [rcx + r13 + 2]
	xor	rax, qword ptr [r14 + 4*r13 + 8]
	xor	rax, r12
	mul	rbp
	mov	r8, rax
	mov	rsi, rdx
	movzx	eax, word ptr [rcx + r13 + 4]
	xor	rax, qword ptr [r14 + 4*r13 + 16]
	xor	rax, r12
	mul	rbp
	mov	rdi, rdx
	xor	rsi, r8
	xor	rdi, rax
	movzx	eax, word ptr [rcx + r13 + 6]
	xor	rax, qword ptr [r14 + 4*r13 + 24]
	xor	rax, r12
	mul	rbp
	mov	r10, rax
	mov	r8, rdx
	movzx	eax, word ptr [rcx + r13 + 8]
	xor	rax, qword ptr [r14 + 4*r13 + 32]
	xor	rax, r12
	mul	rbp
	mov	r9, rdx
	xor	r8, r10
	xor	r9, rax
	movzx	eax, word ptr [rcx + r13 + 10]
	xor	rax, qword ptr [r14 + 4*r13 + 40]
	xor	rax, r12
	mul	rbp
	mov	r11, rax
	mov	r10, rdx
	movzx	eax, word ptr [rcx + r13 + 12]
	xor	rax, qword ptr [r14 + 4*r13 + 48]
	xor	rax, r12
	mul	rbp
	mov	rbx, rdx
	xor	r10, r11
	xor	rbx, rax
	movzx	eax, word ptr [rcx + r13 + 14]
	xor	rax, qword ptr [r14 + 4*r13 + 56]
	xor	rax, r12
	mul	rbp
	xor	rdx, rax
	mov	qword ptr [r14 + 4*r13], r15
	mov	qword ptr [r14 + 4*r13 + 8], rsi
	mov	qword ptr [r14 + 4*r13 + 16], rdi
	mov	qword ptr [r14 + 4*r13 + 24], r8
	mov	qword ptr [r14 + 4*r13 + 32], r9
	mov	qword ptr [r14 + 4*r13 + 40], r10
	mov	qword ptr [r14 + 4*r13 + 48], rbx
	mov	qword ptr [r14 + 4*r13 + 56], rdx
	add	r13, 16
	cmp	qword ptr [rsp + 16], r13
	jne	.LBB75_86
.LBB75_87:
	mov	r9, qword ptr [rsp + 40]
	test	r9, r9
	je	.LBB75_721
	movabs	rax, 1152921504606846960
	or	rax, 8
	mov	rdx, qword ptr [rsp + 24]
	and	rdx, rax
	lea	rcx, [rcx + 2*rdx]
	mov	rax, qword ptr [rsp + 8]
	lea	rsi, [rax + 8*rdx]
	xor	r8d, r8d
	movabs	rdi, -4563226453097033507
	.p2align	4
.LBB75_89:
	movzx	eax, word ptr [rcx + 2*r8]
	xor	rax, qword ptr [rsi + 8*r8]
	xor	rax, r12
	mul	rdi
	xor	rdx, rax
	mov	qword ptr [rsi + 8*r8], rdx
	inc	r8
	cmp	r9, r8
	jne	.LBB75_89
	jmp	.LBB75_721
.LBB75_220:
	mov	rcx, qword ptr [rbx + 32]
	test	r12b, r12b
	je	.LBB75_221
	movabs	r12, 589684135938649225
	mov	rdx, qword ptr [rsp + 24]
	mov	esi, edx
	and	esi, 7
	mov	qword ptr [rsp + 40], rsi
	shr	rax, 4
	shr	rdx, 3
	cmp	rax, rdx
	cmovb	rdx, rax
	mov	qword ptr [rsp + 16], rdx
	test	rdx, rdx
	je	.LBB75_255
	shl	qword ptr [rsp + 16], 4
	xor	r13d, r13d
	movabs	rbp, -4563226453097033507
	mov	r14, qword ptr [rsp + 8]
	.p2align	4
.LBB75_254:
	movzx	eax, word ptr [rcx + r13]
	xor	rax, qword ptr [r14 + 4*r13]
	xor	rax, r12
	mul	rbp
	mov	r15, rdx
	xor	r15, rax
	movzx	eax, word ptr [rcx + r13 + 2]
	xor	rax, qword ptr [r14 + 4*r13 + 8]
	xor	rax, r12
	mul	rbp
	mov	r8, rax
	mov	rsi, rdx
	movzx	eax, word ptr [rcx + r13 + 4]
	xor	rax, qword ptr [r14 + 4*r13 + 16]
	xor	rax, r12
	mul	rbp
	mov	rdi, rdx
	xor	rsi, r8
	xor	rdi, rax
	movzx	eax, word ptr [rcx + r13 + 6]
	xor	rax, qword ptr [r14 + 4*r13 + 24]
	xor	rax, r12
	mul	rbp
	mov	r10, rax
	mov	r8, rdx
	movzx	eax, word ptr [rcx + r13 + 8]
	xor	rax, qword ptr [r14 + 4*r13 + 32]
	xor	rax, r12
	mul	rbp
	mov	r9, rdx
	xor	r8, r10
	xor	r9, rax
	movzx	eax, word ptr [rcx + r13 + 10]
	xor	rax, qword ptr [r14 + 4*r13 + 40]
	xor	rax, r12
	mul	rbp
	mov	r11, rax
	mov	r10, rdx
	movzx	eax, word ptr [rcx + r13 + 12]
	xor	rax, qword ptr [r14 + 4*r13 + 48]
	xor	rax, r12
	mul	rbp
	mov	rbx, rdx
	xor	r10, r11
	xor	rbx, rax
	movzx	eax, word ptr [rcx + r13 + 14]
	xor	rax, qword ptr [r14 + 4*r13 + 56]
	xor	rax, r12
	mul	rbp
	xor	rdx, rax
	mov	qword ptr [r14 + 4*r13], r15
	mov	qword ptr [r14 + 4*r13 + 8], rsi
	mov	qword ptr [r14 + 4*r13 + 16], rdi
	mov	qword ptr [r14 + 4*r13 + 24], r8
	mov	qword ptr [r14 + 4*r13 + 32], r9
	mov	qword ptr [r14 + 4*r13 + 40], r10
	mov	qword ptr [r14 + 4*r13 + 48], rbx
	mov	qword ptr [r14 + 4*r13 + 56], rdx
	add	r13, 16
	cmp	qword ptr [rsp + 16], r13
	jne	.LBB75_254
.LBB75_255:
	mov	r9, qword ptr [rsp + 40]
	test	r9, r9
	je	.LBB75_721
	movabs	rax, 1152921504606846960
	or	rax, 8
	mov	rdx, qword ptr [rsp + 24]
	and	rdx, rax
	lea	rcx, [rcx + 2*rdx]
	mov	rax, qword ptr [rsp + 8]
	lea	rsi, [rax + 8*rdx]
	xor	r8d, r8d
	movabs	rdi, -4563226453097033507
	.p2align	4
.LBB75_257:
	movzx	eax, word ptr [rcx + 2*r8]
	xor	rax, qword ptr [rsi + 8*r8]
	xor	rax, r12
	mul	rdi
	xor	rdx, rax
	mov	qword ptr [rsi + 8*r8], rdx
	inc	r8
	cmp	r9, r8
	jne	.LBB75_257
	jmp	.LBB75_721
.LBB75_136:
	mov	r15, qword ptr [rbx + 32]
	test	r12b, r12b
	je	.LBB75_137
	movabs	r12, 589684135938649225
	mov	rcx, qword ptr [rsp + 24]
	mov	edx, ecx
	and	edx, 7
	mov	qword ptr [rsp + 40], rdx
	shr	rax, 6
	shr	rcx, 3
	cmp	rax, rcx
	cmovb	rcx, rax
	mov	qword ptr [rsp + 16], rcx
	test	rcx, rcx
	je	.LBB75_171
	shl	qword ptr [rsp + 16], 6
	xor	r13d, r13d
	movabs	rbp, -4563226453097033507
	mov	r14, qword ptr [rsp + 8]
	.p2align	4
.LBB75_170:
	mov	rax, qword ptr [r14 + r13]
	mov	rsi, qword ptr [r14 + r13 + 8]
	xor	rax, qword ptr [r15 + r13]
	xor	rax, r12
	mul	rbp
	mov	rcx, rdx
	xor	rcx, rax
	xor	rsi, qword ptr [r15 + r13 + 8]
	xor	rsi, r12
	mov	rax, rsi
	mul	rbp
	mov	r8, rax
	mov	rsi, rdx
	mov	rax, qword ptr [r14 + r13 + 16]
	xor	rax, qword ptr [r15 + r13 + 16]
	xor	rax, r12
	mul	rbp
	mov	rdi, rdx
	xor	rsi, r8
	xor	rdi, rax
	mov	rax, qword ptr [r14 + r13 + 24]
	xor	rax, qword ptr [r15 + r13 + 24]
	xor	rax, r12
	mul	rbp
	mov	r10, rax
	mov	r8, rdx
	mov	rax, qword ptr [r14 + r13 + 32]
	xor	rax, qword ptr [r15 + r13 + 32]
	xor	rax, r12
	mul	rbp
	mov	r9, rdx
	xor	r8, r10
	xor	r9, rax
	mov	rax, qword ptr [r14 + r13 + 40]
	xor	rax, qword ptr [r15 + r13 + 40]
	xor	rax, r12
	mul	rbp
	mov	r11, rax
	mov	r10, rdx
	mov	rax, qword ptr [r14 + r13 + 48]
	xor	rax, qword ptr [r15 + r13 + 48]
	xor	rax, r12
	mul	rbp
	mov	rbx, rdx
	xor	r10, r11
	xor	rbx, rax
	mov	rax, qword ptr [r14 + r13 + 56]
	xor	rax, qword ptr [r15 + r13 + 56]
	xor	rax, r12
	mul	rbp
	xor	rdx, rax
	mov	qword ptr [r14 + r13], rcx
	mov	qword ptr [r14 + r13 + 8], rsi
	mov	qword ptr [r14 + r13 + 16], rdi
	mov	qword ptr [r14 + r13 + 24], r8
	mov	qword ptr [r14 + r13 + 32], r9
	mov	qword ptr [r14 + r13 + 40], r10
	mov	qword ptr [r14 + r13 + 48], rbx
	mov	qword ptr [r14 + r13 + 56], rdx
	add	r13, 64
	cmp	qword ptr [rsp + 16], r13
	jne	.LBB75_170
.LBB75_171:
	mov	r9, qword ptr [rsp + 40]
	test	r9, r9
	je	.LBB75_721
	movabs	rcx, 1152921504606846960
	add	rcx, 8
	mov	rax, qword ptr [rsp + 24]
	and	rax, rcx
	lea	rcx, [r15 + 8*rax]
	mov	rdx, qword ptr [rsp + 8]
	lea	rsi, [rdx + 8*rax]
	xor	r8d, r8d
	movabs	rdi, -4563226453097033507
	.p2align	4
.LBB75_173:
	mov	rax, qword ptr [rcx + 8*r8]
	xor	rax, qword ptr [rsi + 8*r8]
	xor	rax, r12
	mul	rdi
	xor	rdx, rax
	mov	qword ptr [rsi + 8*r8], rdx
	inc	r8
	cmp	r9, r8
	jne	.LBB75_173
	jmp	.LBB75_721
.LBB75_10:
	mov	rcx, qword ptr [rbx + 32]
	test	r12b, r12b
	je	.LBB75_11
	movabs	r12, 589684135938649225
	movabs	rsi, 1152921504606846960
	or	rsi, 8
	mov	rax, qword ptr [rsp + 24]
	and	rsi, rax
	mov	edx, eax
	and	edx, 7
	mov	qword ptr [rsp + 40], rdx
	test	rax, -8
	je	.LBB75_45
	xor	r13d, r13d
	movabs	rbp, -4563226453097033507
	mov	qword ptr [rsp + 16], rsi
	mov	r14, qword ptr [rsp + 8]
	.p2align	4
.LBB75_44:
	movzx	eax, byte ptr [rcx + r13]
	xor	rax, qword ptr [r14 + 8*r13]
	xor	rax, r12
	mul	rbp
	mov	r15, rdx
	xor	r15, rax
	movzx	eax, byte ptr [rcx + r13 + 1]
	xor	rax, qword ptr [r14 + 8*r13 + 8]
	xor	rax, r12
	mul	rbp
	mov	r8, rax
	mov	rsi, rdx
	movzx	eax, byte ptr [rcx + r13 + 2]
	xor	rax, qword ptr [r14 + 8*r13 + 16]
	xor	rax, r12
	mul	rbp
	mov	rdi, rdx
	xor	rsi, r8
	xor	rdi, rax
	movzx	eax, byte ptr [rcx + r13 + 3]
	xor	rax, qword ptr [r14 + 8*r13 + 24]
	xor	rax, r12
	mul	rbp
	mov	r10, rax
	mov	r8, rdx
	movzx	eax, byte ptr [rcx + r13 + 4]
	xor	rax, qword ptr [r14 + 8*r13 + 32]
	xor	rax, r12
	mul	rbp
	mov	r9, rdx
	xor	r8, r10
	xor	r9, rax
	movzx	eax, byte ptr [rcx + r13 + 5]
	xor	rax, qword ptr [r14 + 8*r13 + 40]
	xor	rax, r12
	mul	rbp
	mov	r11, rax
	mov	r10, rdx
	movzx	eax, byte ptr [rcx + r13 + 6]
	xor	rax, qword ptr [r14 + 8*r13 + 48]
	xor	rax, r12
	mul	rbp
	mov	rbx, rdx
	xor	r10, r11
	xor	rbx, rax
	movzx	eax, byte ptr [rcx + r13 + 7]
	xor	rax, qword ptr [r14 + 8*r13 + 56]
	xor	rax, r12
	mul	rbp
	xor	rdx, rax
	mov	qword ptr [r14 + 8*r13], r15
	mov	qword ptr [r14 + 8*r13 + 8], rsi
	mov	rsi, qword ptr [rsp + 16]
	mov	qword ptr [r14 + 8*r13 + 16], rdi
	mov	qword ptr [r14 + 8*r13 + 24], r8
	mov	qword ptr [r14 + 8*r13 + 32], r9
	mov	qword ptr [r14 + 8*r13 + 40], r10
	mov	qword ptr [r14 + 8*r13 + 48], rbx
	mov	qword ptr [r14 + 8*r13 + 56], rdx
	add	r13, 8
	cmp	rsi, r13
	jne	.LBB75_44
.LBB75_45:
	mov	r9, qword ptr [rsp + 40]
	test	r9, r9
	je	.LBB75_721
	add	rcx, rsi
	mov	rax, qword ptr [rsp + 8]
	lea	rsi, [rax + 8*rsi]
	xor	r8d, r8d
	movabs	rdi, -4563226453097033507
	.p2align	4
.LBB75_47:
	movzx	eax, byte ptr [rcx + r8]
	xor	rax, qword ptr [rsi + 8*r8]
	xor	rax, r12
	mul	rdi
	xor	rdx, rax
	mov	qword ptr [rsi + 8*r8], rdx
	inc	r8
	cmp	r9, r8
	jne	.LBB75_47
	jmp	.LBB75_721
.LBB75_94:
	mov	rcx, qword ptr [rbx + 32]
	test	r12b, r12b
	je	.LBB75_95
	movabs	r12, 589684135938649225
	mov	rdx, qword ptr [rsp + 24]
	mov	esi, edx
	and	esi, 7
	mov	qword ptr [rsp + 40], rsi
	shr	rax, 5
	shr	rdx, 3
	cmp	rax, rdx
	cmovb	rdx, rax
	mov	qword ptr [rsp + 16], rdx
	test	rdx, rdx
	je	.LBB75_129
	shl	qword ptr [rsp + 16], 5
	xor	r13d, r13d
	movabs	rbp, -4563226453097033507
	mov	r14, qword ptr [rsp + 8]
	.p2align	4
.LBB75_128:
	mov	eax, dword ptr [rcx + r13]
	mov	esi, dword ptr [rcx + r13 + 4]
	xor	rax, qword ptr [r14 + 2*r13]
	xor	rax, r12
	mul	rbp
	mov	r15, rdx
	xor	r15, rax
	xor	rsi, qword ptr [r14 + 2*r13 + 8]
	xor	rsi, r12
	mov	rax, rsi
	mul	rbp
	mov	r8, rax
	mov	rsi, rdx
	mov	eax, dword ptr [rcx + r13 + 8]
	xor	rax, qword ptr [r14 + 2*r13 + 16]
	xor	rax, r12
	mul	rbp
	mov	rdi, rdx
	xor	rsi, r8
	xor	rdi, rax
	mov	eax, dword ptr [rcx + r13 + 12]
	xor	rax, qword ptr [r14 + 2*r13 + 24]
	xor	rax, r12
	mul	rbp
	mov	r10, rax
	mov	r8, rdx
	mov	eax, dword ptr [rcx + r13 + 16]
	xor	rax, qword ptr [r14 + 2*r13 + 32]
	xor	rax, r12
	mul	rbp
	mov	r9, rdx
	xor	r8, r10
	xor	r9, rax
	mov	eax, dword ptr [rcx + r13 + 20]
	xor	rax, qword ptr [r14 + 2*r13 + 40]
	xor	rax, r12
	mul	rbp
	mov	r11, rax
	mov	r10, rdx
	mov	eax, dword ptr [rcx + r13 + 24]
	xor	rax, qword ptr [r14 + 2*r13 + 48]
	xor	rax, r12
	mul	rbp
	mov	rbx, rdx
	xor	r10, r11
	xor	rbx, rax
	mov	eax, dword ptr [rcx + r13 + 28]
	xor	rax, qword ptr [r14 + 2*r13 + 56]
	xor	rax, r12
	mul	rbp
	xor	rdx, rax
	mov	qword ptr [r14 + 2*r13], r15
	mov	qword ptr [r14 + 2*r13 + 8], rsi
	mov	qword ptr [r14 + 2*r13 + 16], rdi
	mov	qword ptr [r14 + 2*r13 + 24], r8
	mov	qword ptr [r14 + 2*r13 + 32], r9
	mov	qword ptr [r14 + 2*r13 + 40], r10
	mov	qword ptr [r14 + 2*r13 + 48], rbx
	mov	qword ptr [r14 + 2*r13 + 56], rdx
	add	r13, 32
	cmp	qword ptr [rsp + 16], r13
	jne	.LBB75_128
.LBB75_129:
	mov	r9, qword ptr [rsp + 40]
	test	r9, r9
	je	.LBB75_721
	movabs	rdx, 1152921504606846960
	add	rdx, 8
	mov	rax, qword ptr [rsp + 24]
	and	rax, rdx
	lea	rcx, [rcx + 4*rax]
	mov	rdx, qword ptr [rsp + 8]
	lea	rsi, [rdx + 8*rax]
	xor	r8d, r8d
	movabs	rdi, -4563226453097033507
	.p2align	4
.LBB75_131:
	mov	eax, dword ptr [rcx + 4*r8]
	xor	rax, qword ptr [rsi + 8*r8]
	xor	rax, r12
	mul	rdi
	xor	rdx, rax
	mov	qword ptr [rsi + 8*r8], rdx
	inc	r8
	cmp	r9, r8
	jne	.LBB75_131
	jmp	.LBB75_721
.LBB75_262:
	mov	rcx, qword ptr [rbx + 32]
	test	r12b, r12b
	je	.LBB75_263
	movabs	r12, 589684135938649225
	mov	rdx, qword ptr [rsp + 24]
	mov	esi, edx
	and	esi, 7
	mov	qword ptr [rsp + 40], rsi
	shr	rax, 5
	shr	rdx, 3
	cmp	rax, rdx
	cmovb	rdx, rax
	mov	qword ptr [rsp + 16], rdx
	test	rdx, rdx
	je	.LBB75_297
	shl	qword ptr [rsp + 16], 5
	xor	r13d, r13d
	movabs	rbp, -4563226453097033507
	mov	r14, qword ptr [rsp + 8]
	.p2align	4
.LBB75_296:
	mov	eax, dword ptr [rcx + r13]
	mov	esi, dword ptr [rcx + r13 + 4]
	xor	rax, qword ptr [r14 + 2*r13]
	xor	rax, r12
	mul	rbp
	mov	r15, rdx
	xor	r15, rax
	xor	rsi, qword ptr [r14 + 2*r13 + 8]
	xor	rsi, r12
	mov	rax, rsi
	mul	rbp
	mov	r8, rax
	mov	rsi, rdx
	mov	eax, dword ptr [rcx + r13 + 8]
	xor	rax, qword ptr [r14 + 2*r13 + 16]
	xor	rax, r12
	mul	rbp
	mov	rdi, rdx
	xor	rsi, r8
	xor	rdi, rax
	mov	eax, dword ptr [rcx + r13 + 12]
	xor	rax, qword ptr [r14 + 2*r13 + 24]
	xor	rax, r12
	mul	rbp
	mov	r10, rax
	mov	r8, rdx
	mov	eax, dword ptr [rcx + r13 + 16]
	xor	rax, qword ptr [r14 + 2*r13 + 32]
	xor	rax, r12
	mul	rbp
	mov	r9, rdx
	xor	r8, r10
	xor	r9, rax
	mov	eax, dword ptr [rcx + r13 + 20]
	xor	rax, qword ptr [r14 + 2*r13 + 40]
	xor	rax, r12
	mul	rbp
	mov	r11, rax
	mov	r10, rdx
	mov	eax, dword ptr [rcx + r13 + 24]
	xor	rax, qword ptr [r14 + 2*r13 + 48]
	xor	rax, r12
	mul	rbp
	mov	rbx, rdx
	xor	r10, r11
	xor	rbx, rax
	mov	eax, dword ptr [rcx + r13 + 28]
	xor	rax, qword ptr [r14 + 2*r13 + 56]
	xor	rax, r12
	mul	rbp
	xor	rdx, rax
	mov	qword ptr [r14 + 2*r13], r15
	mov	qword ptr [r14 + 2*r13 + 8], rsi
	mov	qword ptr [r14 + 2*r13 + 16], rdi
	mov	qword ptr [r14 + 2*r13 + 24], r8
	mov	qword ptr [r14 + 2*r13 + 32], r9
	mov	qword ptr [r14 + 2*r13 + 40], r10
	mov	qword ptr [r14 + 2*r13 + 48], rbx
	mov	qword ptr [r14 + 2*r13 + 56], rdx
	add	r13, 32
	cmp	qword ptr [rsp + 16], r13
	jne	.LBB75_296
.LBB75_297:
	mov	r9, qword ptr [rsp + 40]
	test	r9, r9
	je	.LBB75_721
	movabs	rdx, 1152921504606846960
	add	rdx, 8
	mov	rax, qword ptr [rsp + 24]
	and	rax, rdx
	lea	rcx, [rcx + 4*rax]
	mov	rdx, qword ptr [rsp + 8]
	lea	rsi, [rdx + 8*rax]
	xor	r8d, r8d
	movabs	rdi, -4563226453097033507
	.p2align	4
.LBB75_299:
	mov	eax, dword ptr [rcx + 4*r8]
	xor	rax, qword ptr [rsi + 8*r8]
	xor	rax, r12
	mul	rdi
	xor	rdx, rax
	mov	qword ptr [rsi + 8*r8], rdx
	inc	r8
	cmp	r9, r8
	jne	.LBB75_299
	jmp	.LBB75_721
.LBB75_304:
	mov	r15, qword ptr [rbx + 32]
	test	r12b, r12b
	je	.LBB75_305
	movabs	r12, 589684135938649225
	mov	rcx, qword ptr [rsp + 24]
	mov	edx, ecx
	and	edx, 7
	mov	qword ptr [rsp + 40], rdx
	shr	rax, 6
	shr	rcx, 3
	cmp	rax, rcx
	cmovb	rcx, rax
	mov	qword ptr [rsp + 16], rcx
	test	rcx, rcx
	je	.LBB75_339
	shl	qword ptr [rsp + 16], 6
	xor	r13d, r13d
	movabs	rbp, -4563226453097033507
	mov	r14, qword ptr [rsp + 8]
	.p2align	4
.LBB75_338:
	mov	rax, qword ptr [r14 + r13]
	mov	rsi, qword ptr [r14 + r13 + 8]
	xor	rax, qword ptr [r15 + r13]
	xor	rax, r12
	mul	rbp
	mov	rcx, rdx
	xor	rcx, rax
	xor	rsi, qword ptr [r15 + r13 + 8]
	xor	rsi, r12
	mov	rax, rsi
	mul	rbp
	mov	r8, rax
	mov	rsi, rdx
	mov	rax, qword ptr [r14 + r13 + 16]
	xor	rax, qword ptr [r15 + r13 + 16]
	xor	rax, r12
	mul	rbp
	mov	rdi, rdx
	xor	rsi, r8
	xor	rdi, rax
	mov	rax, qword ptr [r14 + r13 + 24]
	xor	rax, qword ptr [r15 + r13 + 24]
	xor	rax, r12
	mul	rbp
	mov	r10, rax
	mov	r8, rdx
	mov	rax, qword ptr [r14 + r13 + 32]
	xor	rax, qword ptr [r15 + r13 + 32]
	xor	rax, r12
	mul	rbp
	mov	r9, rdx
	xor	r8, r10
	xor	r9, rax
	mov	rax, qword ptr [r14 + r13 + 40]
	xor	rax, qword ptr [r15 + r13 + 40]
	xor	rax, r12
	mul	rbp
	mov	r11, rax
	mov	r10, rdx
	mov	rax, qword ptr [r14 + r13 + 48]
	xor	rax, qword ptr [r15 + r13 + 48]
	xor	rax, r12
	mul	rbp
	mov	rbx, rdx
	xor	r10, r11
	xor	rbx, rax
	mov	rax, qword ptr [r14 + r13 + 56]
	xor	rax, qword ptr [r15 + r13 + 56]
	xor	rax, r12
	mul	rbp
	xor	rdx, rax
	mov	qword ptr [r14 + r13], rcx
	mov	qword ptr [r14 + r13 + 8], rsi
	mov	qword ptr [r14 + r13 + 16], rdi
	mov	qword ptr [r14 + r13 + 24], r8
	mov	qword ptr [r14 + r13 + 32], r9
	mov	qword ptr [r14 + r13 + 40], r10
	mov	qword ptr [r14 + r13 + 48], rbx
	mov	qword ptr [r14 + r13 + 56], rdx
	add	r13, 64
	cmp	qword ptr [rsp + 16], r13
	jne	.LBB75_338
.LBB75_339:
	mov	r9, qword ptr [rsp + 40]
	test	r9, r9
	je	.LBB75_721
	movabs	rax, 1152921504606846960
	or	rax, 8
	mov	rdx, qword ptr [rsp + 24]
	and	rdx, rax
	lea	rcx, [r15 + 8*rdx]
	mov	rax, qword ptr [rsp + 8]
	lea	rsi, [rax + 8*rdx]
	xor	r8d, r8d
	movabs	rdi, -4563226453097033507
	.p2align	4
.LBB75_341:
	mov	rax, qword ptr [rcx + 8*r8]
	xor	rax, qword ptr [rsi + 8*r8]
	xor	rax, r12
	mul	rdi
	xor	rdx, rax
	mov	qword ptr [rsi + 8*r8], rdx
	inc	r8
	cmp	r9, r8
	jne	.LBB75_341
	jmp	.LBB75_721
.LBB75_518:
	mov	r15, qword ptr [rbx + 32]
	test	r12b, r12b
	je	.LBB75_519
	movabs	r12, 589684135938649225
	mov	rcx, qword ptr [rsp + 24]
	mov	edx, ecx
	and	edx, 7
	mov	qword ptr [rsp + 40], rdx
	shr	rax, 6
	shr	rcx, 3
	cmp	rax, rcx
	cmovb	rcx, rax
	mov	qword ptr [rsp + 16], rcx
	test	rcx, rcx
	je	.LBB75_553
	shl	qword ptr [rsp + 16], 6
	xor	r13d, r13d
	movabs	rbp, -4563226453097033507
	mov	r14, qword ptr [rsp + 8]
	.p2align	4
.LBB75_552:
	mov	rax, qword ptr [r14 + r13]
	mov	rsi, qword ptr [r14 + r13 + 8]
	xor	rax, qword ptr [r15 + r13]
	xor	rax, r12
	mul	rbp
	mov	rcx, rdx
	xor	rcx, rax
	xor	rsi, qword ptr [r15 + r13 + 8]
	xor	rsi, r12
	mov	rax, rsi
	mul	rbp
	mov	r8, rax
	mov	rsi, rdx
	mov	rax, qword ptr [r14 + r13 + 16]
	xor	rax, qword ptr [r15 + r13 + 16]
	xor	rax, r12
	mul	rbp
	mov	rdi, rdx
	xor	rsi, r8
	xor	rdi, rax
	mov	rax, qword ptr [r14 + r13 + 24]
	xor	rax, qword ptr [r15 + r13 + 24]
	xor	rax, r12
	mul	rbp
	mov	r10, rax
	mov	r8, rdx
	mov	rax, qword ptr [r14 + r13 + 32]
	xor	rax, qword ptr [r15 + r13 + 32]
	xor	rax, r12
	mul	rbp
	mov	r9, rdx
	xor	r8, r10
	xor	r9, rax
	mov	rax, qword ptr [r14 + r13 + 40]
	xor	rax, qword ptr [r15 + r13 + 40]
	xor	rax, r12
	mul	rbp
	mov	r11, rax
	mov	r10, rdx
	mov	rax, qword ptr [r14 + r13 + 48]
	xor	rax, qword ptr [r15 + r13 + 48]
	xor	rax, r12
	mul	rbp
	mov	rbx, rdx
	xor	r10, r11
	xor	rbx, rax
	mov	rax, qword ptr [r14 + r13 + 56]
	xor	rax, qword ptr [r15 + r13 + 56]
	xor	rax, r12
	mul	rbp
	xor	rdx, rax
	mov	qword ptr [r14 + r13], rcx
	mov	qword ptr [r14 + r13 + 8], rsi
	mov	qword ptr [r14 + r13 + 16], rdi
	mov	qword ptr [r14 + r13 + 24], r8
	mov	qword ptr [r14 + r13 + 32], r9
	mov	qword ptr [r14 + r13 + 40], r10
	mov	qword ptr [r14 + r13 + 48], rbx
	mov	qword ptr [r14 + r13 + 56], rdx
	add	r13, 64
	cmp	qword ptr [rsp + 16], r13
	jne	.LBB75_552
.LBB75_553:
	mov	r9, qword ptr [rsp + 40]
	test	r9, r9
	je	.LBB75_721
	movabs	rcx, 1152921504606846960
	add	rcx, 8
	mov	rax, qword ptr [rsp + 24]
	and	rax, rcx
	lea	rcx, [r15 + 8*rax]
	mov	rdx, qword ptr [rsp + 8]
	lea	rsi, [rdx + 8*rax]
	xor	r8d, r8d
	movabs	rdi, -4563226453097033507
	.p2align	4
.LBB75_555:
	mov	rax, qword ptr [rcx + 8*r8]
	xor	rax, qword ptr [rsi + 8*r8]
	xor	rax, r12
	mul	rdi
	xor	rdx, rax
	mov	qword ptr [rsi + 8*r8], rdx
	inc	r8
	cmp	r9, r8
	jne	.LBB75_555
	jmp	.LBB75_721
.LBB75_560:
	mov	r13, qword ptr [rbx + 32]
	test	r12b, r12b
	je	.LBB75_561
	movabs	rbx, -4563226453097033507
	mov	rbp, qword ptr [rsp + 24]
	movabs	r15, 589684135938649225
	mov	ecx, ebp
	and	ecx, 7
	mov	qword ptr [rsp + 16], rcx
	shr	rax, 7
	shr	rbp, 3
	cmp	rax, rbp
	cmovb	rbp, rax
	test	rbp, rbp
	je	.LBB75_595
	shl	rbp, 6
	xor	r12d, r12d
	mov	r14, qword ptr [rsp + 8]
	.p2align	4
.LBB75_594:
	mov	rcx, qword ptr [r14 + r12]
	mov	rsi, qword ptr [r14 + r12 + 8]
	xor	rcx, qword ptr [r13 + 2*r12]
	xor	rcx, r15
	mov	rax, qword ptr [r13 + 2*r12 + 8]
	xor	rax, rbx
	mul	rcx
	mov	rcx, rdx
	xor	rsi, qword ptr [r13 + 2*r12 + 16]
	xor	rcx, rax
	xor	rsi, r15
	mov	rax, qword ptr [r13 + 2*r12 + 24]
	xor	rax, rbx
	mul	rsi
	mov	rsi, rdx
	mov	rdx, qword ptr [r14 + r12 + 16]
	xor	rdx, qword ptr [r13 + 2*r12 + 32]
	xor	rsi, rax
	xor	rdx, r15
	mov	rax, qword ptr [r13 + 2*r12 + 40]
	xor	rax, rbx
	mul	rdx
	mov	rdi, rdx
	mov	rdx, qword ptr [r14 + r12 + 24]
	xor	rdx, qword ptr [r13 + 2*r12 + 48]
	xor	rdi, rax
	xor	rdx, r15
	mov	rax, qword ptr [r13 + 2*r12 + 56]
	xor	rax, rbx
	mul	rdx
	mov	r8, rdx
	mov	rdx, qword ptr [r14 + r12 + 32]
	xor	rdx, qword ptr [r13 + 2*r12 + 64]
	xor	r8, rax
	xor	rdx, r15
	mov	rax, qword ptr [r13 + 2*r12 + 72]
	xor	rax, rbx
	mul	rdx
	mov	r9, rdx
	mov	rdx, qword ptr [r14 + r12 + 40]
	xor	rdx, qword ptr [r13 + 2*r12 + 80]
	xor	r9, rax
	xor	rdx, r15
	mov	rax, qword ptr [r13 + 2*r12 + 88]
	xor	rax, rbx
	mul	rdx
	mov	r10, rdx
	mov	rdx, qword ptr [r14 + r12 + 48]
	xor	rdx, qword ptr [r13 + 2*r12 + 96]
	xor	r10, rax
	xor	rdx, r15
	mov	rax, qword ptr [r13 + 2*r12 + 104]
	xor	rax, rbx
	mul	rdx
	mov	r11, rdx
	mov	rdx, qword ptr [r14 + r12 + 56]
	xor	rdx, qword ptr [r13 + 2*r12 + 112]
	xor	r11, rax
	xor	rdx, r15
	mov	rax, qword ptr [r13 + 2*r12 + 120]
	xor	rax, rbx
	mul	rdx
	xor	rdx, rax
	mov	qword ptr [r14 + r12], rcx
	mov	qword ptr [r14 + r12 + 8], rsi
	mov	qword ptr [r14 + r12 + 16], rdi
	mov	qword ptr [r14 + r12 + 24], r8
	mov	qword ptr [r14 + r12 + 32], r9
	mov	qword ptr [r14 + r12 + 40], r10
	mov	qword ptr [r14 + r12 + 48], r11
	mov	qword ptr [r14 + r12 + 56], rdx
	add	r12, 64
	cmp	rbp, r12
	jne	.LBB75_594
.LBB75_595:
	mov	rdi, qword ptr [rsp + 16]
	test	rdi, rdi
	je	.LBB75_721
	movabs	rax, 576460752303423472
	or	rax, 8
	movabs	rcx, 1152921504606846960
	mov	rdx, qword ptr [rsp + 24]
	and	rax, rdx
	or	rcx, 8
	shl	rax, 4
	and	rcx, rdx
	add	r13, rax
	mov	rax, qword ptr [rsp + 8]
	lea	rcx, [rax + 8*rcx]
	shl	edi, 3
	xor	esi, esi
	.p2align	4
.LBB75_597:
	mov	rdx, qword ptr [rcx + rsi]
	xor	rdx, qword ptr [r13 + 2*rsi]
	xor	rdx, r15
	mov	rax, qword ptr [r13 + 2*rsi + 8]
	xor	rax, rbx
	mul	rdx
	xor	rdx, rax
	mov	qword ptr [rcx + rsi], rdx
	add	rsi, 8
	cmp	rdi, rsi
	jne	.LBB75_597
	jmp	.LBB75_721
.LBB75_388:
	mov	r13, qword ptr [rbx + 32]
	test	r12b, r12b
	je	.LBB75_389
	movabs	r12, 4577018097722394903
	mov	rcx, qword ptr [rsp + 24]
	movabs	r15, 589684135938649225
	mov	edx, ecx
	and	edx, 7
	mov	qword ptr [rsp + 48], rdx
	shr	rax, 5
	shr	rcx, 3
	cmp	rax, rcx
	cmovb	rcx, rax
	mov	qword ptr [rsp + 40], rcx
	test	rcx, rcx
	je	.LBB75_423
	shl	qword ptr [rsp + 40], 5
	xor	ebx, ebx
	mov	r14, qword ptr [rsp + 8]
	.p2align	4
.LBB75_422:
	mov	eax, dword ptr [r13 + rbx]
	mov	esi, dword ptr [r13 + rbx + 4]
	mov	rcx, qword ptr [r14 + 2*rbx]
	xor	rcx, r15
	rol	rcx, 60
	mov	edx, 2147483648
	cmp	rax, rdx
	mov	edi, 2147483648
	mov	ebp, 0
	cmove	rax, rbp
	xor	rcx, rax
	xor	rax, r12
	mul	rcx
	xor	rdx, rax
	mov	qword ptr [rsp + 16], rdx
	mov	rdx, qword ptr [r14 + 2*rbx + 8]
	xor	rdx, r15
	rol	rdx, 60
	cmp	rsi, rdi
	mov	ecx, 2147483648
	cmove	rsi, rbp
	xor	rdx, rsi
	xor	rsi, r12
	mov	rax, rsi
	mul	rdx
	mov	rsi, rdx
	xor	rsi, rax
	mov	eax, dword ptr [r13 + rbx + 8]
	mov	rdx, qword ptr [r14 + 2*rbx + 16]
	xor	rdx, r15
	rol	rdx, 60
	cmp	rax, rcx
	cmove	rax, rbp
	xor	rdx, rax
	xor	rax, r12
	mul	rdx
	mov	rdi, rdx
	xor	rdi, rax
	mov	eax, dword ptr [r13 + rbx + 12]
	mov	rdx, qword ptr [r14 + 2*rbx + 24]
	xor	rdx, r15
	rol	rdx, 60
	cmp	rax, rcx
	cmove	rax, rbp
	xor	rdx, rax
	xor	rax, r12
	mul	rdx
	mov	r8, rdx
	xor	r8, rax
	mov	eax, dword ptr [r13 + rbx + 16]
	mov	rdx, qword ptr [r14 + 2*rbx + 32]
	xor	rdx, r15
	rol	rdx, 60
	cmp	rax, rcx
	cmove	rax, rbp
	xor	rdx, rax
	xor	rax, r12
	mul	rdx
	mov	r9, rdx
	xor	r9, rax
	mov	eax, dword ptr [r13 + rbx + 20]
	mov	rdx, qword ptr [r14 + 2*rbx + 40]
	xor	rdx, r15
	rol	rdx, 60
	cmp	rax, rcx
	cmove	rax, rbp
	xor	rdx, rax
	xor	rax, r12
	mul	rdx
	mov	r10, rdx
	xor	r10, rax
	mov	eax, dword ptr [r13 + rbx + 24]
	mov	rdx, qword ptr [r14 + 2*rbx + 48]
	xor	rdx, r15
	rol	rdx, 60
	cmp	rax, rcx
	cmove	rax, rbp
	xor	rdx, rax
	xor	rax, r12
	mul	rdx
	mov	r11, rdx
	xor	r11, rax
	mov	eax, dword ptr [r13 + rbx + 28]
	mov	rdx, qword ptr [r14 + 2*rbx + 56]
	xor	rdx, r15
	rol	rdx, 60
	cmp	rax, rcx
	cmove	rax, rbp
	xor	rdx, rax
	xor	rax, r12
	mul	rdx
	xor	rdx, rax
	mov	rax, qword ptr [rsp + 16]
	mov	qword ptr [r14 + 2*rbx], rax
	mov	qword ptr [r14 + 2*rbx + 8], rsi
	mov	qword ptr [r14 + 2*rbx + 16], rdi
	mov	qword ptr [r14 + 2*rbx + 24], r8
	mov	qword ptr [r14 + 2*rbx + 32], r9
	mov	qword ptr [r14 + 2*rbx + 40], r10
	mov	qword ptr [r14 + 2*rbx + 48], r11
	mov	qword ptr [r14 + 2*rbx + 56], rdx
	add	rbx, 32
	cmp	qword ptr [rsp + 40], rbx
	jne	.LBB75_422
.LBB75_423:
	mov	r11, qword ptr [rsp + 48]
	test	r11, r11
	je	.LBB75_721
	movabs	rcx, 1152921504606846960
	add	rcx, 8
	mov	rax, qword ptr [rsp + 24]
	and	rax, rcx
	lea	rcx, [4*rax]
	add	rcx, r13
	mov	rdx, qword ptr [rsp + 8]
	lea	rsi, [rdx + 8*rax]
	xor	r8d, r8d
	mov	edi, 2147483648
	.p2align	4
.LBB75_425:
	mov	edx, dword ptr [rcx + 4*r8]
	mov	r9, qword ptr [rsi + 8*r8]
	xor	r9, r15
	rol	r9, 60
	cmp	rdx, rdi
	mov	eax, 0
	cmovne	rax, rdx
	xor	r9, rax
	xor	rax, r12
	mul	r9
	xor	rdx, rax
	mov	qword ptr [rsi + 8*r8], rdx
	inc	r8
	cmp	r11, r8
	jne	.LBB75_425
	jmp	.LBB75_721
.LBB75_346:
	mov	r8, qword ptr [rbx + 32]
	test	r12b, r12b
	je	.LBB75_347
	movabs	r13, 4577018097722394903
	movabs	r12, 589684135938649225
	mov	r15, qword ptr [rsp + 24]
	mov	ecx, r15d
	and	ecx, 7
	mov	qword ptr [rsp + 48], rcx
	shr	rax, 4
	shr	r15, 3
	cmp	rax, r15
	cmovb	r15, rax
	mov	qword ptr [rsp + 40], r15
	test	r15, r15
	je	.LBB75_381
	shl	qword ptr [rsp + 40], 4
	xor	ebp, ebp
	mov	r14, qword ptr [rsp + 8]
	.p2align	4
.LBB75_380:
	mov	rcx, qword ptr [r14 + 4*rbp]
	xor	rcx, r12
	movzx	eax, word ptr [r8 + rbp]
	cmp	eax, 32768
	mov	edx, 0
	cmove	eax, edx
	xor	edi, edi
	mov	edx, eax
	shr	edx, 8
	rol	rcx, 62
	movzx	esi, al
	xor	rsi, rcx
	and	eax, 65280
	or	eax, edx
	xor	rax, r13
	mul	rsi
	xor	rdx, rax
	mov	qword ptr [rsp + 16], rdx
	mov	rdx, qword ptr [r14 + 4*rbp + 8]
	xor	rdx, r12
	movzx	eax, word ptr [r8 + rbp + 2]
	cmp	eax, 32768
	cmove	eax, edi
	xor	ecx, ecx
	mov	esi, eax
	shr	esi, 8
	rol	rdx, 62
	movzx	edi, al
	xor	rdi, rdx
	and	eax, 65280
	or	eax, esi
	xor	rax, r13
	mul	rdi
	mov	rsi, rdx
	xor	rsi, rax
	mov	rdx, qword ptr [r14 + 4*rbp + 16]
	xor	rdx, r12
	movzx	eax, word ptr [r8 + rbp + 4]
	cmp	eax, 32768
	cmove	eax, ecx
	mov	edi, eax
	shr	edi, 8
	rol	rdx, 62
	movzx	r9d, al
	xor	r9, rdx
	and	eax, 65280
	or	eax, edi
	xor	rax, r13
	mul	r9
	mov	rdi, rdx
	xor	rdi, rax
	mov	rdx, qword ptr [r14 + 4*rbp + 24]
	xor	rdx, r12
	movzx	eax, word ptr [r8 + rbp + 6]
	cmp	eax, 32768
	cmove	eax, ecx
	mov	r9d, eax
	shr	r9d, 8
	rol	rdx, 62
	movzx	r10d, al
	xor	r10, rdx
	and	eax, 65280
	or	eax, r9d
	xor	rax, r13
	mul	r10
	mov	rbx, rdx
	xor	rbx, rax
	mov	rdx, qword ptr [r14 + 4*rbp + 32]
	xor	rdx, r12
	movzx	eax, word ptr [r8 + rbp + 8]
	cmp	eax, 32768
	cmove	eax, ecx
	mov	r9d, eax
	shr	r9d, 8
	rol	rdx, 62
	movzx	r10d, al
	xor	r10, rdx
	and	eax, 65280
	or	eax, r9d
	xor	rax, r13
	mul	r10
	mov	r9, rdx
	xor	r9, rax
	mov	rdx, qword ptr [r14 + 4*rbp + 40]
	xor	rdx, r12
	movzx	eax, word ptr [r8 + rbp + 10]
	cmp	eax, 32768
	cmove	eax, ecx
	mov	r10d, eax
	shr	r10d, 8
	rol	rdx, 62
	movzx	r11d, al
	xor	r11, rdx
	and	eax, 65280
	or	eax, r10d
	xor	rax, r13
	mul	r11
	mov	r10, rdx
	xor	r10, rax
	mov	rdx, qword ptr [r14 + 4*rbp + 48]
	xor	rdx, r12
	movzx	eax, word ptr [r8 + rbp + 12]
	cmp	eax, 32768
	cmove	eax, ecx
	mov	r11d, eax
	shr	r11d, 8
	rol	rdx, 62
	movzx	r15d, al
	xor	r15, rdx
	and	eax, 65280
	or	eax, r11d
	xor	rax, r13
	mul	r15
	mov	r11, rdx
	xor	r11, rax
	mov	rdx, qword ptr [r14 + 4*rbp + 56]
	xor	rdx, r12
	movzx	eax, word ptr [r8 + rbp + 14]
	cmp	eax, 32768
	cmove	eax, ecx
	rol	rdx, 62
	movzx	r15d, al
	xor	r15, rdx
	mov	edx, eax
	shr	edx, 8
	and	eax, 65280
	or	eax, edx
	xor	rax, r13
	mul	r15
	xor	rdx, rax
	mov	rax, qword ptr [rsp + 16]
	mov	qword ptr [r14 + 4*rbp], rax
	mov	qword ptr [r14 + 4*rbp + 8], rsi
	mov	qword ptr [r14 + 4*rbp + 16], rdi
	mov	qword ptr [r14 + 4*rbp + 24], rbx
	mov	qword ptr [r14 + 4*rbp + 32], r9
	mov	qword ptr [r14 + 4*rbp + 40], r10
	mov	qword ptr [r14 + 4*rbp + 48], r11
	mov	qword ptr [r14 + 4*rbp + 56], rdx
	add	rbp, 16
	cmp	qword ptr [rsp + 40], rbp
	jne	.LBB75_380
.LBB75_381:
	mov	rbx, qword ptr [rsp + 48]
	test	rbx, rbx
	je	.LBB75_721
	movabs	rcx, 1152921504606846960
	add	rcx, 8
	mov	rax, qword ptr [rsp + 24]
	and	rax, rcx
	lea	rcx, [r8 + 2*rax]
	mov	rdx, qword ptr [rsp + 8]
	lea	rsi, [rdx + 8*rax]
	xor	edi, edi
	xor	r8d, r8d
	.p2align	4
.LBB75_383:
	mov	rdx, qword ptr [rsi + 8*r8]
	xor	rdx, r12
	movzx	eax, word ptr [rcx + 2*r8]
	cmp	eax, 32768
	cmove	eax, edi
	mov	r9d, eax
	shr	r9d, 8
	rol	rdx, 62
	movzx	r10d, al
	xor	r10, rdx
	and	eax, 65280
	or	eax, r9d
	xor	rax, r13
	mul	r10
	xor	rdx, rax
	mov	qword ptr [rsi + 8*r8], rdx
	inc	r8
	cmp	rbx, r8
	jne	.LBB75_383
	jmp	.LBB75_721
.LBB75_603:
	mov	rbp, qword ptr [rbx + 32]
	test	r12b, r12b
	je	.LBB75_604
	movabs	r15, -4563226453097033507
	movabs	r12, 589684135938649225
	mov	rcx, qword ptr [rsp + 24]
	mov	edx, ecx
	and	edx, 7
	mov	qword ptr [rsp + 72], rdx
	shr	rax, 8
	shr	rcx, 3
	cmp	rax, rcx
	cmovb	rcx, rax
	mov	qword ptr [rsp + 48], rcx
	test	rcx, rcx
	je	.LBB75_640
	shl	qword ptr [rsp + 48], 6
	xor	esi, esi
	mov	r14, qword ptr [rsp + 8]
	.p2align	4
.LBB75_639:
	mov	rdx, qword ptr [r14 + rsi]
	mov	rdi, qword ptr [r14 + rsi + 8]
	xor	rdx, qword ptr [rbp + 4*rsi]
	xor	rdx, r12
	mov	rax, qword ptr [rbp + 4*rsi + 8]
	xor	rax, r15
	mul	rdx
	xor	rdx, qword ptr [rbp + 4*rsi + 16]
	xor	rax, rdx
	mov	rdx, qword ptr [rbp + 4*rsi + 24]
	xor	rdx, r15
	mul	rdx
	mov	qword ptr [rsp + 16], rax
	mov	rcx, rdx
	xor	rdi, qword ptr [rbp + 4*rsi + 32]
	xor	rdi, r12
	mov	rax, qword ptr [rbp + 4*rsi + 40]
	xor	rax, r15
	mul	rdi
	xor	rdx, qword ptr [rbp + 4*rsi + 48]
	xor	rax, rdx
	mov	rdx, qword ptr [rbp + 4*rsi + 56]
	xor	rdx, r15
	mul	rdx
	mov	r13, rax
	mov	r8, rdx
	mov	rdx, qword ptr [r14 + rsi + 16]
	xor	rdx, qword ptr [rbp + 4*rsi + 64]
	xor	rdx, r12
	mov	rax, qword ptr [rbp + 4*rsi + 72]
	xor	rax, r15
	mul	rdx
	xor	rdx, qword ptr [rbp + 4*rsi + 80]
	xor	rax, rdx
	mov	rdx, qword ptr [rbp + 4*rsi + 88]
	xor	rdx, r15
	mul	rdx
	mov	qword ptr [rsp + 40], rax
	mov	rdi, rdx
	mov	rdx, qword ptr [r14 + rsi + 24]
	xor	rdx, qword ptr [rbp + 4*rsi + 96]
	xor	rdx, r12
	mov	rax, qword ptr [rbp + 4*rsi + 104]
	xor	rax, r15
	mul	rdx
	xor	rdx, qword ptr [rbp + 4*rsi + 112]
	xor	rax, rdx
	mov	rdx, qword ptr [rbp + 4*rsi + 120]
	xor	rdx, r15
	mul	rdx
	mov	rbx, rax
	mov	r9, rdx
	mov	rdx, qword ptr [r14 + rsi + 32]
	xor	rdx, qword ptr [rbp + 4*rsi + 128]
	xor	rdx, r12
	mov	rax, qword ptr [rbp + 4*rsi + 136]
	xor	rax, r15
	mul	rdx
	xor	rdx, qword ptr [rbp + 4*rsi + 144]
	xor	rax, rdx
	mov	rdx, qword ptr [rbp + 4*rsi + 152]
	xor	rdx, r15
	mul	rdx
	mov	r10, rax
	mov	r11, rdx
	mov	rdx, qword ptr [r14 + rsi + 40]
	xor	rdx, qword ptr [rbp + 4*rsi + 160]
	xor	rdx, r12
	mov	rax, qword ptr [rbp + 4*rsi + 168]
	xor	rax, r15
	mul	rdx
	xor	rcx, qword ptr [rsp + 16]
	xor	rdx, qword ptr [rbp + 4*rsi + 176]
	xor	r8, r13
	mov	qword ptr [rsp + 16], r8
	xor	rax, rdx
	mov	rdx, qword ptr [rbp + 4*rsi + 184]
	xor	rdx, r15
	mul	rdx
	mov	r13, rax
	mov	r8, rdx
	xor	rdi, qword ptr [rsp + 40]
	mov	rdx, qword ptr [r14 + rsi + 48]
	xor	rdx, qword ptr [rbp + 4*rsi + 192]
	xor	rdx, r12
	mov	rax, qword ptr [rbp + 4*rsi + 200]
	xor	rax, r15
	mul	rdx
	xor	r9, rbx
	xor	rdx, qword ptr [rbp + 4*rsi + 208]
	xor	r11, r10
	xor	rax, rdx
	mov	rdx, qword ptr [rbp + 4*rsi + 216]
	xor	rdx, r15
	mul	rdx
	mov	rbx, rax
	mov	r10, rdx
	xor	r8, r13
	mov	rdx, qword ptr [r14 + rsi + 56]
	xor	rdx, qword ptr [rbp + 4*rsi + 224]
	xor	rdx, r12
	mov	rax, qword ptr [rbp + 4*rsi + 232]
	xor	rax, r15
	mul	rdx
	xor	r10, rbx
	xor	rdx, qword ptr [rbp + 4*rsi + 240]
	xor	rax, rdx
	mov	rdx, qword ptr [rbp + 4*rsi + 248]
	xor	rdx, r15
	mul	rdx
	xor	rdx, rax
	mov	qword ptr [r14 + rsi], rcx
	mov	rax, qword ptr [rsp + 16]
	mov	qword ptr [r14 + rsi + 8], rax
	mov	qword ptr [r14 + rsi + 16], rdi
	mov	qword ptr [r14 + rsi + 24], r9
	mov	qword ptr [r14 + rsi + 32], r11
	mov	qword ptr [r14 + rsi + 40], r8
	mov	qword ptr [r14 + rsi + 48], r10
	mov	qword ptr [r14 + rsi + 56], rdx
	add	rsi, 64
	cmp	qword ptr [rsp + 48], rsi
	jne	.LBB75_639
.LBB75_640:
	mov	r9, qword ptr [rsp + 72]
	test	r9, r9
	je	.LBB75_721
	movabs	rax, 1152921504606846960
	or	rax, 8
	mov	rdx, qword ptr [rsp + 24]
	and	rax, rdx
	mov	rcx, qword ptr [rsp + 8]
	lea	rsi, [rcx + 8*rax]
	shl	rdx, 5
	movabs	rax, 9223372036854775552
	and	rax, rdx
	lea	rdi, [rax + rbp]
	add	rdi, 16
	shl	r9d, 3
	xor	r8d, r8d
	.p2align	4
.LBB75_642:
	mov	rdx, qword ptr [rsi + r8]
	xor	rdx, qword ptr [rdi + 4*r8 - 16]
	xor	rdx, r12
	mov	rax, qword ptr [rdi + 4*r8 - 8]
	xor	rax, r15
	mul	rdx
	xor	rdx, qword ptr [rdi + 4*r8]
	xor	rax, rdx
	mov	rdx, qword ptr [rdi + 4*r8 + 8]
	xor	rdx, r15
	mul	rdx
	xor	rdx, rax
	mov	qword ptr [rsi + r8], rdx
	add	r8, 8
	cmp	r9, r8
	jne	.LBB75_642
	jmp	.LBB75_721
.LBB75_646:
	call	_RINvNtCs33DBFqlTBxO_11arrow_array4cast18as_primitive_arrayNtNtB4_5types21IntervalYearMonthTypeECsg1h6yN5JQqd_7hashasm
	movzx	r8d, r12b
	mov	rdi, rax
	mov	rsi, qword ptr [rsp + 16]
	mov	rdx, qword ptr [rsp + 8]
	mov	rcx, qword ptr [rsp + 24]
	call	_RINvNtCspC5jgAXZxx_17datafusion_common10hash_utils20hash_array_primitiveNtNtCs33DBFqlTBxO_11arrow_array5types21IntervalYearMonthTypeNtNtCsRVR2D38dYC_8foldhash4fast10FixedStateECsg1h6yN5JQqd_7hashasm
	jmp	.LBB75_721
.LBB75_647:
	call	_RINvNtCs33DBFqlTBxO_11arrow_array4cast18as_primitive_arrayNtNtB4_5types24IntervalMonthDayNanoTypeECsg1h6yN5JQqd_7hashasm
	movzx	r8d, r12b
	mov	rdi, rax
	mov	rsi, qword ptr [rsp + 16]
	mov	rdx, qword ptr [rsp + 8]
	mov	rcx, qword ptr [rsp + 24]
	call	_RINvNtCspC5jgAXZxx_17datafusion_common10hash_utils20hash_array_primitiveNtNtCs33DBFqlTBxO_11arrow_array5types24IntervalMonthDayNanoTypeNtNtCsRVR2D38dYC_8foldhash4fast10FixedStateECsg1h6yN5JQqd_7hashasm
	jmp	.LBB75_721
.LBB75_644:
	mov	rdi, r13
	call	_RINvNtCs33DBFqlTBxO_11arrow_array4cast18as_primitive_arrayNtNtB4_5types23DurationMicrosecondTypeECsg1h6yN5JQqd_7hashasm
	movzx	r8d, r12b
	mov	rdi, rax
	mov	rsi, qword ptr [rsp + 16]
	mov	rdx, qword ptr [rsp + 8]
	mov	rcx, qword ptr [rsp + 24]
	call	_RINvNtCspC5jgAXZxx_17datafusion_common10hash_utils20hash_array_primitiveNtNtCs33DBFqlTBxO_11arrow_array5types23DurationMicrosecondTypeNtNtCsRVR2D38dYC_8foldhash4fast10FixedStateECsg1h6yN5JQqd_7hashasm
	jmp	.LBB75_721
.LBB75_645:
	mov	rdi, r13
	call	_RINvNtCs33DBFqlTBxO_11arrow_array4cast18as_primitive_arrayNtNtB4_5types22DurationNanosecondTypeECsg1h6yN5JQqd_7hashasm
	movzx	r8d, r12b
	mov	rdi, rax
	mov	rsi, qword ptr [rsp + 16]
	mov	rdx, qword ptr [rsp + 8]
	mov	rcx, qword ptr [rsp + 24]
	call	_RINvNtCspC5jgAXZxx_17datafusion_common10hash_utils20hash_array_primitiveNtNtCs33DBFqlTBxO_11arrow_array5types22DurationNanosecondTypeNtNtCsRVR2D38dYC_8foldhash4fast10FixedStateECsg1h6yN5JQqd_7hashasm
	jmp	.LBB75_721
.LBB75_643:
	mov	rdi, r13
	call	_RINvNtCs33DBFqlTBxO_11arrow_array4cast18as_primitive_arrayNtNtB4_5types23DurationMillisecondTypeECsg1h6yN5JQqd_7hashasm
	movzx	r8d, r12b
	mov	rdi, rax
	mov	rsi, qword ptr [rsp + 16]
	mov	rdx, qword ptr [rsp + 8]
	mov	rcx, qword ptr [rsp + 24]
	call	_RINvNtCspC5jgAXZxx_17datafusion_common10hash_utils20hash_array_primitiveNtNtCs33DBFqlTBxO_11arrow_array5types23DurationMillisecondTypeNtNtCsRVR2D38dYC_8foldhash4fast10FixedStateECsg1h6yN5JQqd_7hashasm
	jmp	.LBB75_721
.LBB75_477:
	mov	rsi, qword ptr [rsp + 24]
	mov	edx, esi
	and	edx, 15
	mov	qword ptr [rsp + 120], rdx
	shr	rax, 6
	shr	rsi, 4
	cmp	rax, rsi
	cmovb	rsi, rax
	mov	qword ptr [rsp + 128], rsi
	test	rsi, rsi
	je	.LBB75_480
	mov	rax, qword ptr [rsp + 16]
	mov	r15, qword ptr [rax]
	shl	qword ptr [rsp + 128], 6
	xor	r12d, r12d
	.p2align	4
.LBB75_479:
	mov	eax, dword ptr [rcx + r12]
	mov	esi, dword ptr [rcx + r12 + 4]
	mov	edi, dword ptr [rcx + r12 + 8]
	mov	r9d, dword ptr [rcx + r12 + 12]
	mov	r11d, dword ptr [rcx + r12 + 16]
	mov	ebp, dword ptr [rcx + r12 + 20]
	mov	r13d, dword ptr [rcx + r12 + 24]
	mov	r10d, dword ptr [rcx + r12 + 28]
	mov	ebx, dword ptr [rcx + r12 + 32]
	mov	edx, dword ptr [rcx + r12 + 36]
	mov	qword ptr [rsp + 64], rdx
	mov	edx, dword ptr [rcx + r12 + 40]
	mov	qword ptr [rsp + 56], rdx
	mov	edx, dword ptr [rcx + r12 + 44]
	mov	qword ptr [rsp + 88], rdx
	mov	edx, dword ptr [rcx + r12 + 48]
	mov	qword ptr [rsp + 104], rdx
	xor	rax, r15
	movabs	r14, -4563226453097033507
	mul	r14
	xor	rdx, rax
	mov	qword ptr [rsp + 40], rdx
	xor	rsi, r15
	mov	rax, rsi
	mul	r14
	mov	r8, rax
	mov	rsi, rdx
	xor	rdi, r15
	mov	rax, rdi
	mul	r14
	xor	rsi, r8
	mov	qword ptr [rsp + 48], rsi
	xor	rdx, rax
	mov	qword ptr [rsp + 72], rdx
	xor	r9, r15
	mov	rax, r9
	mul	r14
	xor	rdx, rax
	mov	qword ptr [rsp + 96], rdx
	xor	r11, r15
	mov	rax, r11
	mul	r14
	xor	rdx, rax
	mov	qword ptr [rsp + 80], rdx
	xor	rbp, r15
	mov	rax, rbp
	mul	r14
	mov	rsi, rax
	mov	rdi, rdx
	xor	r13, r15
	mov	rax, r13
	mul	r14
	mov	rbp, rdx
	xor	rdi, rsi
	mov	qword ptr [rsp + 112], rdi
	xor	rbp, rax
	xor	r10, r15
	mov	rax, r10
	mul	r14
	mov	r11, rdx
	xor	r11, rax
	xor	rbx, r15
	mov	rax, rbx
	mul	r14
	mov	r9, rdx
	xor	r9, rax
	mov	rax, qword ptr [rsp + 64]
	xor	rax, r15
	mul	r14
	mov	rsi, rax
	mov	rbx, rdx
	mov	rax, qword ptr [rsp + 56]
	xor	rax, r15
	mul	r14
	mov	r13, rdx
	xor	rbx, rsi
	xor	r13, rax
	mov	rax, qword ptr [rsp + 88]
	xor	rax, r15
	mul	r14
	mov	r10, rdx
	mov	esi, dword ptr [rcx + r12 + 52]
	xor	r10, rax
	mov	rax, qword ptr [rsp + 104]
	xor	rax, r15
	mul	r14
	mov	r8, rdx
	xor	r8, rax
	xor	rsi, r15
	mov	rax, rsi
	mul	r14
	mov	rsi, rdx
	xor	rsi, rax
	mov	eax, dword ptr [rcx + r12 + 56]
	xor	rax, r15
	mul	r14
	mov	rdi, rdx
	xor	rdi, rax
	mov	eax, dword ptr [rcx + r12 + 60]
	xor	rax, r15
	mul	r14
	xor	rdx, rax
	mov	rax, qword ptr [rsp + 8]
	mov	r14, qword ptr [rsp + 40]
	mov	qword ptr [rax + 2*r12], r14
	mov	r14, qword ptr [rsp + 48]
	mov	qword ptr [rax + 2*r12 + 8], r14
	mov	r14, qword ptr [rsp + 72]
	mov	qword ptr [rax + 2*r12 + 16], r14
	mov	r14, qword ptr [rsp + 96]
	mov	qword ptr [rax + 2*r12 + 24], r14
	mov	r14, qword ptr [rsp + 80]
	mov	qword ptr [rax + 2*r12 + 32], r14
	mov	r14, qword ptr [rsp + 112]
	mov	qword ptr [rax + 2*r12 + 40], r14
	mov	qword ptr [rax + 2*r12 + 48], rbp
	mov	qword ptr [rax + 2*r12 + 56], r11
	mov	qword ptr [rax + 2*r12 + 64], r9
	mov	qword ptr [rax + 2*r12 + 72], rbx
	mov	qword ptr [rax + 2*r12 + 80], r13
	mov	qword ptr [rax + 2*r12 + 88], r10
	mov	qword ptr [rax + 2*r12 + 96], r8
	mov	qword ptr [rax + 2*r12 + 104], rsi
	mov	qword ptr [rax + 2*r12 + 112], rdi
	mov	qword ptr [rax + 2*r12 + 120], rdx
	add	r12, 64
	cmp	qword ptr [rsp + 128], r12
	jne	.LBB75_479
.LBB75_480:
	mov	r10, qword ptr [rsp + 120]
	test	r10, r10
	je	.LBB75_721
	movabs	rax, 1152921504606846960
	mov	rdx, qword ptr [rsp + 24]
	and	rdx, rax
	lea	rcx, [rcx + 4*rdx]
	mov	rax, qword ptr [rsp + 8]
	lea	rsi, [rax + 8*rdx]
	mov	rax, qword ptr [rsp + 16]
	mov	rdi, qword ptr [rax]
	xor	r9d, r9d
	movabs	r8, -4563226453097033507
	.p2align	4
.LBB75_482:
	mov	eax, dword ptr [rcx + 4*r9]
	xor	rax, rdi
	mul	r8
	xor	rdx, rax
	mov	qword ptr [rsi + 8*r9], rdx
	inc	r9
	cmp	r10, r9
	jne	.LBB75_482
	jmp	.LBB75_721
.LBB75_484:
	lea	rdi, [rsp + 144]
	call	qword ptr [rip + _RNvMNtNtCs7SjCkj1olcZ_12arrow_buffer6buffer4nullNtB2_10NullBuffer13valid_indices@GOTPCREL]
	mov	r9, qword ptr [rsp + 144]
	mov	rcx, qword ptr [rsp + 152]
	mov	r12, qword ptr [rsp + 160]
	mov	rsi, qword ptr [rsp + 168]
	mov	rdi, qword ptr [rsp + 176]
	mov	r8, qword ptr [rsp + 184]
	mov	r13, qword ptr [rsp + 192]
	mov	r10, qword ptr [rsp + 200]
	mov	r11, qword ptr [rbx + 32]
	mov	rax, qword ptr [rsp + 16]
	mov	rbx, qword ptr [rax]
	movabs	r15, -4563226453097033507
	jmp	.LBB75_485
	.p2align	4
.LBB75_488:
	mov	r13, rsi
	xor	r12d, r12d
	add	r10, 64
.LBB75_485:
	test	r13, r13
	jne	.LBB75_495
	cmp	r12, -2
	je	.LBB75_492
	cmp	r12, 1
	je	.LBB75_488
	test	rdi, rdi
	je	.LBB75_492
	cmp	rdi, r8
	je	.LBB75_492
	mov	r13, qword ptr [rdi]
	add	rdi, 8
	mov	r12, -1
	add	r10, 64
	jmp	.LBB75_485
	.p2align	4
.LBB75_492:
	dec	r9
	cmp	r9, -3
	ja	.LBB75_721
	mov	r12, -2
	mov	r13, rcx
	xor	r9d, r9d
	add	r10, 64
	jmp	.LBB75_485
.LBB75_495:
	rep		bsf	rbp, r13
	add	rbp, r10
	mov	eax, dword ptr [r11 + 4*rbp]
	xor	rax, rbx
	mul	r15
	xor	rdx, rax
	lea	rax, [r13 - 1]
	and	r13, rax
	mov	rax, qword ptr [rsp + 8]
	mov	qword ptr [rax + 8*rbp], rdx
	jmp	.LBB75_485
.LBB75_431:
	mov	r15, qword ptr [rsp + 24]
	mov	ecx, r15d
	and	ecx, 15
	mov	qword ptr [rsp + 64], rcx
	shr	rax, 7
	shr	r15, 4
	cmp	rax, r15
	cmovb	r15, rax
	mov	qword ptr [rsp + 56], r15
	test	r15, r15
	je	.LBB75_434
	mov	rax, qword ptr [rsp + 16]
	mov	r9, qword ptr [rax]
	shl	qword ptr [rsp + 56], 7
	xor	r10d, r10d
	movabs	r11, -4563226453097033507
	xor	r8d, r8d
	.p2align	4
.LBB75_433:
	mov	rax, qword ptr [r13 + r8]
	mov	rcx, qword ptr [r13 + r8 + 8]
	mov	rdx, rax
	neg	rdx
	cmovo	rax, r10
	xor	rax, r9
	mul	r11
	mov	r14, rdx
	mov	rsi, qword ptr [r13 + r8 + 16]
	xor	r14, rax
	mov	rax, rcx
	neg	rax
	cmovo	rcx, r10
	xor	rcx, r9
	mov	rax, rcx
	mul	r11
	mov	rcx, qword ptr [r13 + r8 + 24]
	xor	rdx, rax
	mov	qword ptr [rsp + 40], rdx
	mov	rax, rsi
	neg	rax
	cmovo	rsi, r10
	xor	rsi, r9
	mov	rax, rsi
	mul	r11
	mov	rsi, qword ptr [r13 + r8 + 32]
	xor	rdx, rax
	mov	qword ptr [rsp + 48], rdx
	mov	rax, rcx
	neg	rax
	cmovo	rcx, r10
	xor	rcx, r9
	mov	rax, rcx
	mul	r11
	mov	rcx, qword ptr [r13 + r8 + 40]
	xor	rdx, rax
	mov	qword ptr [rsp + 72], rdx
	mov	rax, rsi
	neg	rax
	cmovo	rsi, r10
	xor	rsi, r9
	mov	rax, rsi
	mul	r11
	mov	rsi, qword ptr [r13 + r8 + 48]
	xor	rdx, rax
	mov	qword ptr [rsp + 104], rdx
	mov	rax, rcx
	neg	rax
	cmovo	rcx, r10
	xor	rcx, r9
	mov	rax, rcx
	mul	r11
	mov	rcx, qword ptr [r13 + r8 + 56]
	xor	rdx, rax
	mov	qword ptr [rsp + 96], rdx
	mov	rax, rsi
	neg	rax
	cmovo	rsi, r10
	xor	rsi, r9
	mov	rax, rsi
	mul	r11
	mov	rsi, qword ptr [r13 + r8 + 64]
	xor	rdx, rax
	mov	qword ptr [rsp + 88], rdx
	mov	rax, rcx
	neg	rax
	cmovo	rcx, r10
	xor	rcx, r9
	mov	rax, rcx
	mul	r11
	mov	rcx, qword ptr [r13 + r8 + 72]
	xor	rdx, rax
	mov	qword ptr [rsp + 80], rdx
	mov	rax, rsi
	neg	rax
	cmovo	rsi, r10
	xor	rsi, r9
	mov	rax, rsi
	mul	r11
	mov	rbp, rdx
	mov	rsi, qword ptr [r13 + r8 + 80]
	xor	rbp, rax
	mov	rax, rcx
	neg	rax
	cmovo	rcx, r10
	xor	rcx, r9
	mov	rax, rcx
	mul	r11
	mov	rbx, rdx
	mov	rcx, qword ptr [r13 + r8 + 88]
	xor	rbx, rax
	mov	rax, rsi
	neg	rax
	cmovo	rsi, r10
	xor	rsi, r9
	mov	rax, rsi
	mul	r11
	mov	r15, rdx
	mov	rsi, qword ptr [r13 + r8 + 96]
	xor	r15, rax
	mov	rax, rcx
	neg	rax
	cmovo	rcx, r10
	xor	rcx, r9
	mov	rax, rcx
	mul	r11
	mov	r12, rdx
	mov	rdi, qword ptr [r13 + r8 + 104]
	xor	r12, rax
	mov	rax, rsi
	neg	rax
	cmovo	rsi, r10
	xor	rsi, r9
	mov	rax, rsi
	mul	r11
	mov	rcx, rdx
	xor	rcx, rax
	mov	rax, rdi
	neg	rax
	cmovo	rdi, r10
	xor	rdi, r9
	mov	rax, rdi
	mul	r11
	mov	rsi, rdx
	xor	rsi, rax
	mov	rax, qword ptr [r13 + r8 + 112]
	mov	rdx, rax
	neg	rdx
	cmovo	rax, r10
	xor	rax, r9
	mul	r11
	mov	rdi, rdx
	xor	rdi, rax
	mov	rax, qword ptr [r13 + r8 + 120]
	mov	rdx, rax
	neg	rdx
	cmovo	rax, r10
	xor	rax, r9
	mul	r11
	xor	rdx, rax
	mov	rax, qword ptr [rsp + 8]
	mov	qword ptr [rax + r8], r14
	mov	r14, qword ptr [rsp + 40]
	mov	qword ptr [rax + r8 + 8], r14
	mov	r14, qword ptr [rsp + 48]
	mov	qword ptr [rax + r8 + 16], r14
	mov	r14, qword ptr [rsp + 72]
	mov	qword ptr [rax + r8 + 24], r14
	mov	r14, qword ptr [rsp + 104]
	mov	qword ptr [rax + r8 + 32], r14
	mov	r14, qword ptr [rsp + 96]
	mov	qword ptr [rax + r8 + 40], r14
	mov	r14, qword ptr [rsp + 88]
	mov	qword ptr [rax + r8 + 48], r14
	mov	r14, qword ptr [rsp + 80]
	mov	qword ptr [rax + r8 + 56], r14
	mov	qword ptr [rax + r8 + 64], rbp
	mov	qword ptr [rax + r8 + 72], rbx
	mov	qword ptr [rax + r8 + 80], r15
	mov	qword ptr [rax + r8 + 88], r12
	mov	qword ptr [rax + r8 + 96], rcx
	mov	qword ptr [rax + r8 + 104], rsi
	mov	qword ptr [rax + r8 + 112], rdi
	mov	qword ptr [rax + r8 + 120], rdx
	sub	r8, -128
	cmp	qword ptr [rsp + 56], r8
	jne	.LBB75_433
.LBB75_434:
	mov	r10, qword ptr [rsp + 64]
	test	r10, r10
	je	.LBB75_721
	movabs	rax, 1152921504606846960
	mov	rdx, qword ptr [rsp + 24]
	and	rdx, rax
	lea	rcx, [8*rdx]
	add	rcx, r13
	mov	rax, qword ptr [rsp + 8]
	lea	rsi, [rax + 8*rdx]
	mov	rax, qword ptr [rsp + 16]
	mov	rdi, qword ptr [rax]
	xor	r9d, r9d
	movabs	r8, -4563226453097033507
	.p2align	4
.LBB75_436:
	mov	rdx, qword ptr [rcx + 8*r9]
	mov	rax, rdx
	neg	rax
	mov	eax, 0
	cmovno	rax, rdx
	xor	rax, rdi
	mul	r8
	xor	rdx, rax
	mov	qword ptr [rsi + 8*r9], rdx
	inc	r9
	cmp	r10, r9
	jne	.LBB75_436
	jmp	.LBB75_721
.LBB75_438:
	lea	rdi, [rsp + 144]
	call	qword ptr [rip + _RNvMNtNtCs7SjCkj1olcZ_12arrow_buffer6buffer4nullNtB2_10NullBuffer13valid_indices@GOTPCREL]
	mov	r9, qword ptr [rsp + 144]
	mov	rcx, qword ptr [rsp + 152]
	mov	r12, qword ptr [rsp + 160]
	mov	rsi, qword ptr [rsp + 168]
	mov	rdi, qword ptr [rsp + 176]
	mov	r8, qword ptr [rsp + 184]
	mov	r13, qword ptr [rsp + 192]
	mov	r10, qword ptr [rsp + 200]
	mov	r11, qword ptr [rbx + 32]
	mov	rax, qword ptr [rsp + 16]
	mov	rbx, qword ptr [rax]
	movabs	r15, -4563226453097033507
	jmp	.LBB75_439
	.p2align	4
.LBB75_442:
	mov	r13, rsi
	xor	r12d, r12d
	add	r10, 64
.LBB75_439:
	test	r13, r13
	jne	.LBB75_449
	cmp	r12, -2
	je	.LBB75_446
	cmp	r12, 1
	je	.LBB75_442
	test	rdi, rdi
	je	.LBB75_446
	cmp	rdi, r8
	je	.LBB75_446
	mov	r13, qword ptr [rdi]
	add	rdi, 8
	mov	r12, -1
	add	r10, 64
	jmp	.LBB75_439
	.p2align	4
.LBB75_446:
	dec	r9
	cmp	r9, -3
	ja	.LBB75_721
	mov	r12, -2
	mov	r13, rcx
	xor	r9d, r9d
	add	r10, 64
	jmp	.LBB75_439
.LBB75_449:
	rep		bsf	rbp, r13
	add	rbp, r10
	mov	rdx, qword ptr [r11 + 8*rbp]
	mov	rax, rdx
	neg	rax
	mov	eax, 0
	cmovno	rax, rdx
	xor	rax, rbx
	mul	r15
	xor	rdx, rax
	lea	rax, [r13 - 1]
	and	r13, rax
	mov	rax, qword ptr [rsp + 8]
	mov	qword ptr [rax + 8*rbp], rdx
	jmp	.LBB75_439
.LBB75_179:
	movabs	rsi, 1152921504606846960
	mov	rdx, qword ptr [rsp + 24]
	and	rsi, rdx
	mov	eax, edx
	and	eax, 15
	mov	qword ptr [rsp + 120], rax
	test	rdx, -16
	je	.LBB75_182
	mov	rax, qword ptr [rsp + 16]
	mov	rbp, qword ptr [rax]
	xor	r12d, r12d
	mov	qword ptr [rsp + 128], rsi
	.p2align	4
.LBB75_181:
	movzx	eax, byte ptr [rcx + r12]
	movzx	esi, byte ptr [rcx + r12 + 1]
	movzx	edi, byte ptr [rcx + r12 + 2]
	movzx	r9d, byte ptr [rcx + r12 + 3]
	movzx	r8d, byte ptr [rcx + r12 + 4]
	movzx	r13d, byte ptr [rcx + r12 + 5]
	movzx	r15d, byte ptr [rcx + r12 + 6]
	movzx	r10d, byte ptr [rcx + r12 + 7]
	movzx	ebx, byte ptr [rcx + r12 + 8]
	movzx	edx, byte ptr [rcx + r12 + 9]
	mov	qword ptr [rsp + 56], rdx
	movzx	edx, byte ptr [rcx + r12 + 10]
	mov	qword ptr [rsp + 80], rdx
	movzx	edx, byte ptr [rcx + r12 + 11]
	mov	qword ptr [rsp + 96], rdx
	movzx	edx, byte ptr [rcx + r12 + 12]
	mov	qword ptr [rsp + 72], rdx
	xor	rax, rbp
	movabs	r11, -4563226453097033507
	mul	r11
	xor	rdx, rax
	mov	qword ptr [rsp + 24], rdx
	xor	rsi, rbp
	mov	rax, rsi
	mul	r11
	mov	rsi, rax
	mov	r14, rdx
	xor	rdi, rbp
	mov	rax, rdi
	mul	r11
	xor	r14, rsi
	mov	qword ptr [rsp + 40], r14
	xor	rdx, rax
	mov	qword ptr [rsp + 48], rdx
	xor	r9, rbp
	mov	rax, r9
	mul	r11
	xor	rdx, rax
	mov	qword ptr [rsp + 104], rdx
	xor	r8, rbp
	mov	rax, r8
	mul	r11
	xor	rdx, rax
	mov	qword ptr [rsp + 88], rdx
	xor	r13, rbp
	mov	rax, r13
	mul	r11
	mov	rsi, rax
	mov	rdi, rdx
	xor	r15, rbp
	mov	rax, r15
	mul	r11
	xor	rdi, rsi
	mov	qword ptr [rsp + 64], rdi
	xor	rdx, rax
	mov	qword ptr [rsp + 112], rdx
	xor	r10, rbp
	mov	rax, r10
	mul	r11
	mov	r14, rdx
	xor	r14, rax
	xor	rbx, rbp
	mov	rax, rbx
	mul	r11
	mov	r9, rdx
	xor	r9, rax
	mov	rax, qword ptr [rsp + 56]
	xor	rax, rbp
	mul	r11
	mov	rsi, rax
	mov	rbx, rdx
	mov	rax, qword ptr [rsp + 80]
	xor	rax, rbp
	mul	r11
	mov	r15, rdx
	xor	rbx, rsi
	xor	r15, rax
	mov	rax, qword ptr [rsp + 96]
	xor	rax, rbp
	mul	r11
	mov	r10, rdx
	movzx	esi, byte ptr [rcx + r12 + 13]
	xor	r10, rax
	mov	rax, qword ptr [rsp + 72]
	xor	rax, rbp
	mul	r11
	mov	r8, rdx
	xor	r8, rax
	xor	rsi, rbp
	mov	rax, rsi
	mul	r11
	mov	rsi, rdx
	xor	rsi, rax
	movzx	eax, byte ptr [rcx + r12 + 14]
	xor	rax, rbp
	mul	r11
	mov	rdi, rdx
	xor	rdi, rax
	movzx	eax, byte ptr [rcx + r12 + 15]
	xor	rax, rbp
	mul	r11
	xor	rdx, rax
	mov	rax, qword ptr [rsp + 8]
	mov	r13, qword ptr [rsp + 24]
	mov	qword ptr [rax + 8*r12], r13
	mov	r11, qword ptr [rsp + 40]
	mov	qword ptr [rax + 8*r12 + 8], r11
	mov	r11, qword ptr [rsp + 48]
	mov	qword ptr [rax + 8*r12 + 16], r11
	mov	r11, qword ptr [rsp + 104]
	mov	qword ptr [rax + 8*r12 + 24], r11
	mov	r11, qword ptr [rsp + 88]
	mov	qword ptr [rax + 8*r12 + 32], r11
	mov	r11, qword ptr [rsp + 64]
	mov	qword ptr [rax + 8*r12 + 40], r11
	mov	r11, qword ptr [rsp + 112]
	mov	qword ptr [rax + 8*r12 + 48], r11
	mov	qword ptr [rax + 8*r12 + 56], r14
	mov	qword ptr [rax + 8*r12 + 64], r9
	mov	qword ptr [rax + 8*r12 + 72], rbx
	mov	qword ptr [rax + 8*r12 + 80], r15
	mov	qword ptr [rax + 8*r12 + 88], r10
	mov	qword ptr [rax + 8*r12 + 96], r8
	mov	qword ptr [rax + 8*r12 + 104], rsi
	mov	rsi, qword ptr [rsp + 128]
	mov	qword ptr [rax + 8*r12 + 112], rdi
	mov	qword ptr [rax + 8*r12 + 120], rdx
	add	r12, 16
	cmp	rsi, r12
	jne	.LBB75_181
.LBB75_182:
	mov	r10, qword ptr [rsp + 120]
	test	r10, r10
	je	.LBB75_721
	add	rcx, rsi
	mov	rax, qword ptr [rsp + 8]
	lea	rsi, [rax + 8*rsi]
	mov	rax, qword ptr [rsp + 16]
	mov	rdi, qword ptr [rax]
	xor	r9d, r9d
	movabs	r8, -4563226453097033507
	.p2align	4
.LBB75_184:
	movzx	eax, byte ptr [rcx + r9]
	xor	rax, rdi
	mul	r8
	xor	rdx, rax
	mov	qword ptr [rsi + 8*r9], rdx
	inc	r9
	cmp	r10, r9
	jne	.LBB75_184
	jmp	.LBB75_721
.LBB75_186:
	lea	rdi, [rsp + 144]
	call	qword ptr [rip + _RNvMNtNtCs7SjCkj1olcZ_12arrow_buffer6buffer4nullNtB2_10NullBuffer13valid_indices@GOTPCREL]
	mov	r9, qword ptr [rsp + 144]
	mov	rcx, qword ptr [rsp + 152]
	mov	r12, qword ptr [rsp + 160]
	mov	rsi, qword ptr [rsp + 168]
	mov	rdi, qword ptr [rsp + 176]
	mov	r8, qword ptr [rsp + 184]
	mov	r13, qword ptr [rsp + 192]
	mov	r10, qword ptr [rsp + 200]
	mov	r11, qword ptr [rbx + 32]
	mov	rax, qword ptr [rsp + 16]
	mov	rbx, qword ptr [rax]
	movabs	r15, -4563226453097033507
	jmp	.LBB75_187
	.p2align	4
.LBB75_190:
	mov	r13, rsi
	xor	r12d, r12d
	add	r10, 64
.LBB75_187:
	test	r13, r13
	jne	.LBB75_197
	cmp	r12, -2
	je	.LBB75_194
	cmp	r12, 1
	je	.LBB75_190
	test	rdi, rdi
	je	.LBB75_194
	cmp	rdi, r8
	je	.LBB75_194
	mov	r13, qword ptr [rdi]
	add	rdi, 8
	mov	r12, -1
	add	r10, 64
	jmp	.LBB75_187
	.p2align	4
.LBB75_194:
	dec	r9
	cmp	r9, -3
	ja	.LBB75_721
	mov	r12, -2
	mov	r13, rcx
	xor	r9d, r9d
	add	r10, 64
	jmp	.LBB75_187
.LBB75_197:
	rep		bsf	rbp, r13
	add	rbp, r10
	movzx	eax, byte ptr [r11 + rbp]
	xor	rax, rbx
	mul	r15
	xor	rdx, rax
	lea	rax, [r13 - 1]
	and	r13, rax
	mov	rax, qword ptr [rsp + 8]
	mov	qword ptr [rax + 8*rbp], rdx
	jmp	.LBB75_187
.LBB75_53:
	mov	rsi, qword ptr [rsp + 24]
	mov	edx, esi
	and	edx, 15
	mov	qword ptr [rsp + 136], rdx
	shr	rax, 5
	shr	rsi, 4
	cmp	rax, rsi
	cmovb	rsi, rax
	mov	qword ptr [rsp + 120], rsi
	test	rsi, rsi
	je	.LBB75_56
	mov	rax, qword ptr [rsp + 16]
	mov	r15, qword ptr [rax]
	shl	qword ptr [rsp + 120], 5
	xor	r12d, r12d
	.p2align	4
.LBB75_55:
	movzx	eax, word ptr [rcx + r12]
	movzx	esi, word ptr [rcx + r12 + 2]
	movzx	edi, word ptr [rcx + r12 + 4]
	movzx	r9d, word ptr [rcx + r12 + 6]
	movzx	r8d, word ptr [rcx + r12 + 8]
	movzx	ebp, word ptr [rcx + r12 + 10]
	movzx	r13d, word ptr [rcx + r12 + 12]
	movzx	r10d, word ptr [rcx + r12 + 14]
	movzx	ebx, word ptr [rcx + r12 + 16]
	movzx	edx, word ptr [rcx + r12 + 18]
	mov	qword ptr [rsp + 64], rdx
	movzx	edx, word ptr [rcx + r12 + 20]
	mov	qword ptr [rsp + 56], rdx
	movzx	edx, word ptr [rcx + r12 + 22]
	mov	qword ptr [rsp + 88], rdx
	movzx	edx, word ptr [rcx + r12 + 24]
	mov	qword ptr [rsp + 104], rdx
	xor	rax, r15
	movabs	r11, -4563226453097033507
	mul	r11
	xor	rdx, rax
	mov	qword ptr [rsp + 40], rdx
	xor	rsi, r15
	mov	rax, rsi
	mul	r11
	mov	rsi, rax
	mov	r14, rdx
	xor	rdi, r15
	mov	rax, rdi
	mul	r11
	xor	r14, rsi
	mov	qword ptr [rsp + 48], r14
	xor	rdx, rax
	mov	qword ptr [rsp + 72], rdx
	xor	r9, r15
	mov	rax, r9
	mul	r11
	xor	rdx, rax
	mov	qword ptr [rsp + 96], rdx
	xor	r8, r15
	mov	rax, r8
	mul	r11
	xor	rdx, rax
	mov	qword ptr [rsp + 80], rdx
	xor	rbp, r15
	mov	rax, rbp
	mul	r11
	mov	rsi, rax
	mov	rdi, rdx
	xor	r13, r15
	mov	rax, r13
	mul	r11
	xor	rdi, rsi
	mov	qword ptr [rsp + 112], rdi
	xor	rdx, rax
	mov	qword ptr [rsp + 128], rdx
	xor	r10, r15
	mov	rax, r10
	mul	r11
	mov	r14, rdx
	xor	r14, rax
	xor	rbx, r15
	mov	rax, rbx
	mul	r11
	mov	r9, rdx
	xor	r9, rax
	mov	rax, qword ptr [rsp + 64]
	xor	rax, r15
	mul	r11
	mov	rsi, rax
	mov	rbx, rdx
	mov	rax, qword ptr [rsp + 56]
	xor	rax, r15
	mul	r11
	mov	r13, rdx
	xor	rbx, rsi
	xor	r13, rax
	mov	rax, qword ptr [rsp + 88]
	xor	rax, r15
	mul	r11
	mov	r10, rdx
	movzx	esi, word ptr [rcx + r12 + 26]
	xor	r10, rax
	mov	rax, qword ptr [rsp + 104]
	xor	rax, r15
	mul	r11
	mov	r8, rdx
	xor	r8, rax
	xor	rsi, r15
	mov	rax, rsi
	mul	r11
	mov	rsi, rdx
	xor	rsi, rax
	movzx	eax, word ptr [rcx + r12 + 28]
	xor	rax, r15
	mul	r11
	mov	rdi, rdx
	xor	rdi, rax
	movzx	eax, word ptr [rcx + r12 + 30]
	xor	rax, r15
	mul	r11
	xor	rdx, rax
	mov	rax, qword ptr [rsp + 8]
	mov	rbp, qword ptr [rsp + 40]
	mov	qword ptr [rax + 4*r12], rbp
	mov	r11, qword ptr [rsp + 48]
	mov	qword ptr [rax + 4*r12 + 8], r11
	mov	r11, qword ptr [rsp + 72]
	mov	qword ptr [rax + 4*r12 + 16], r11
	mov	r11, qword ptr [rsp + 96]
	mov	qword ptr [rax + 4*r12 + 24], r11
	mov	r11, qword ptr [rsp + 80]
	mov	qword ptr [rax + 4*r12 + 32], r11
	mov	r11, qword ptr [rsp + 112]
	mov	qword ptr [rax + 4*r12 + 40], r11
	mov	r11, qword ptr [rsp + 128]
	mov	qword ptr [rax + 4*r12 + 48], r11
	mov	qword ptr [rax + 4*r12 + 56], r14
	mov	qword ptr [rax + 4*r12 + 64], r9
	mov	qword ptr [rax + 4*r12 + 72], rbx
	mov	qword ptr [rax + 4*r12 + 80], r13
	mov	qword ptr [rax + 4*r12 + 88], r10
	mov	qword ptr [rax + 4*r12 + 96], r8
	mov	qword ptr [rax + 4*r12 + 104], rsi
	mov	qword ptr [rax + 4*r12 + 112], rdi
	mov	qword ptr [rax + 4*r12 + 120], rdx
	add	r12, 32
	cmp	qword ptr [rsp + 120], r12
	jne	.LBB75_55
.LBB75_56:
	mov	r10, qword ptr [rsp + 136]
	test	r10, r10
	je	.LBB75_721
	movabs	rax, 1152921504606846960
	mov	rdx, qword ptr [rsp + 24]
	and	rdx, rax
	lea	rcx, [rcx + 2*rdx]
	mov	rax, qword ptr [rsp + 8]
	lea	rsi, [rax + 8*rdx]
	mov	rax, qword ptr [rsp + 16]
	mov	rdi, qword ptr [rax]
	xor	r9d, r9d
	movabs	r8, -4563226453097033507
	.p2align	4
.LBB75_58:
	movzx	eax, word ptr [rcx + 2*r9]
	xor	rax, rdi
	mul	r8
	xor	rdx, rax
	mov	qword ptr [rsi + 8*r9], rdx
	inc	r9
	cmp	r10, r9
	jne	.LBB75_58
	jmp	.LBB75_721
.LBB75_221:
	mov	rsi, qword ptr [rsp + 24]
	mov	edx, esi
	and	edx, 15
	mov	qword ptr [rsp + 136], rdx
	shr	rax, 5
	shr	rsi, 4
	cmp	rax, rsi
	cmovb	rsi, rax
	mov	qword ptr [rsp + 120], rsi
	test	rsi, rsi
	je	.LBB75_224
	mov	rax, qword ptr [rsp + 16]
	mov	r15, qword ptr [rax]
	shl	qword ptr [rsp + 120], 5
	xor	r12d, r12d
	.p2align	4
.LBB75_223:
	movzx	eax, word ptr [rcx + r12]
	movzx	esi, word ptr [rcx + r12 + 2]
	movzx	edi, word ptr [rcx + r12 + 4]
	movzx	r9d, word ptr [rcx + r12 + 6]
	movzx	r8d, word ptr [rcx + r12 + 8]
	movzx	ebp, word ptr [rcx + r12 + 10]
	movzx	r13d, word ptr [rcx + r12 + 12]
	movzx	r10d, word ptr [rcx + r12 + 14]
	movzx	ebx, word ptr [rcx + r12 + 16]
	movzx	edx, word ptr [rcx + r12 + 18]
	mov	qword ptr [rsp + 64], rdx
	movzx	edx, word ptr [rcx + r12 + 20]
	mov	qword ptr [rsp + 56], rdx
	movzx	edx, word ptr [rcx + r12 + 22]
	mov	qword ptr [rsp + 88], rdx
	movzx	edx, word ptr [rcx + r12 + 24]
	mov	qword ptr [rsp + 104], rdx
	xor	rax, r15
	movabs	r11, -4563226453097033507
	mul	r11
	xor	rdx, rax
	mov	qword ptr [rsp + 40], rdx
	xor	rsi, r15
	mov	rax, rsi
	mul	r11
	mov	rsi, rax
	mov	r14, rdx
	xor	rdi, r15
	mov	rax, rdi
	mul	r11
	xor	r14, rsi
	mov	qword ptr [rsp + 48], r14
	xor	rdx, rax
	mov	qword ptr [rsp + 72], rdx
	xor	r9, r15
	mov	rax, r9
	mul	r11
	xor	rdx, rax
	mov	qword ptr [rsp + 96], rdx
	xor	r8, r15
	mov	rax, r8
	mul	r11
	xor	rdx, rax
	mov	qword ptr [rsp + 80], rdx
	xor	rbp, r15
	mov	rax, rbp
	mul	r11
	mov	rsi, rax
	mov	rdi, rdx
	xor	r13, r15
	mov	rax, r13
	mul	r11
	xor	rdi, rsi
	mov	qword ptr [rsp + 112], rdi
	xor	rdx, rax
	mov	qword ptr [rsp + 128], rdx
	xor	r10, r15
	mov	rax, r10
	mul	r11
	mov	r14, rdx
	xor	r14, rax
	xor	rbx, r15
	mov	rax, rbx
	mul	r11
	mov	r9, rdx
	xor	r9, rax
	mov	rax, qword ptr [rsp + 64]
	xor	rax, r15
	mul	r11
	mov	rsi, rax
	mov	rbx, rdx
	mov	rax, qword ptr [rsp + 56]
	xor	rax, r15
	mul	r11
	mov	r13, rdx
	xor	rbx, rsi
	xor	r13, rax
	mov	rax, qword ptr [rsp + 88]
	xor	rax, r15
	mul	r11
	mov	r10, rdx
	movzx	esi, word ptr [rcx + r12 + 26]
	xor	r10, rax
	mov	rax, qword ptr [rsp + 104]
	xor	rax, r15
	mul	r11
	mov	r8, rdx
	xor	r8, rax
	xor	rsi, r15
	mov	rax, rsi
	mul	r11
	mov	rsi, rdx
	xor	rsi, rax
	movzx	eax, word ptr [rcx + r12 + 28]
	xor	rax, r15
	mul	r11
	mov	rdi, rdx
	xor	rdi, rax
	movzx	eax, word ptr [rcx + r12 + 30]
	xor	rax, r15
	mul	r11
	xor	rdx, rax
	mov	rax, qword ptr [rsp + 8]
	mov	rbp, qword ptr [rsp + 40]
	mov	qword ptr [rax + 4*r12], rbp
	mov	r11, qword ptr [rsp + 48]
	mov	qword ptr [rax + 4*r12 + 8], r11
	mov	r11, qword ptr [rsp + 72]
	mov	qword ptr [rax + 4*r12 + 16], r11
	mov	r11, qword ptr [rsp + 96]
	mov	qword ptr [rax + 4*r12 + 24], r11
	mov	r11, qword ptr [rsp + 80]
	mov	qword ptr [rax + 4*r12 + 32], r11
	mov	r11, qword ptr [rsp + 112]
	mov	qword ptr [rax + 4*r12 + 40], r11
	mov	r11, qword ptr [rsp + 128]
	mov	qword ptr [rax + 4*r12 + 48], r11
	mov	qword ptr [rax + 4*r12 + 56], r14
	mov	qword ptr [rax + 4*r12 + 64], r9
	mov	qword ptr [rax + 4*r12 + 72], rbx
	mov	qword ptr [rax + 4*r12 + 80], r13
	mov	qword ptr [rax + 4*r12 + 88], r10
	mov	qword ptr [rax + 4*r12 + 96], r8
	mov	qword ptr [rax + 4*r12 + 104], rsi
	mov	qword ptr [rax + 4*r12 + 112], rdi
	mov	qword ptr [rax + 4*r12 + 120], rdx
	add	r12, 32
	cmp	qword ptr [rsp + 120], r12
	jne	.LBB75_223
.LBB75_224:
	mov	r10, qword ptr [rsp + 136]
	test	r10, r10
	je	.LBB75_721
	movabs	rax, 1152921504606846960
	mov	rdx, qword ptr [rsp + 24]
	and	rdx, rax
	lea	rcx, [rcx + 2*rdx]
	mov	rax, qword ptr [rsp + 8]
	lea	rsi, [rax + 8*rdx]
	mov	rax, qword ptr [rsp + 16]
	mov	rdi, qword ptr [rax]
	xor	r9d, r9d
	movabs	r8, -4563226453097033507
	.p2align	4
.LBB75_226:
	movzx	eax, word ptr [rcx + 2*r9]
	xor	rax, rdi
	mul	r8
	xor	rdx, rax
	mov	qword ptr [rsi + 8*r9], rdx
	inc	r9
	cmp	r10, r9
	jne	.LBB75_226
	jmp	.LBB75_721
.LBB75_137:
	mov	rcx, qword ptr [rsp + 24]
	mov	edx, ecx
	and	edx, 15
	mov	qword ptr [rsp + 64], rdx
	shr	rax, 7
	shr	rcx, 4
	cmp	rax, rcx
	cmovb	rcx, rax
	mov	qword ptr [rsp + 56], rcx
	test	rcx, rcx
	je	.LBB75_140
	mov	rax, qword ptr [rsp + 16]
	mov	r9, qword ptr [rax]
	shl	qword ptr [rsp + 56], 7
	xor	r10d, r10d
	movabs	r11, -4563226453097033507
	.p2align	4
.LBB75_139:
	mov	rax, qword ptr [r15 + r10]
	xor	rax, r9
	mul	r11
	xor	rdx, rax
	mov	qword ptr [rsp + 40], rdx
	mov	rax, qword ptr [r15 + r10 + 8]
	xor	rax, r9
	mul	r11
	xor	rdx, rax
	mov	qword ptr [rsp + 48], rdx
	mov	rax, qword ptr [r15 + r10 + 16]
	xor	rax, r9
	mul	r11
	xor	rdx, rax
	mov	qword ptr [rsp + 72], rdx
	mov	rax, qword ptr [r15 + r10 + 24]
	xor	rax, r9
	mul	r11
	xor	rdx, rax
	mov	qword ptr [rsp + 104], rdx
	mov	rax, qword ptr [r15 + r10 + 32]
	xor	rax, r9
	mul	r11
	xor	rdx, rax
	mov	qword ptr [rsp + 96], rdx
	mov	rax, qword ptr [r15 + r10 + 40]
	xor	rax, r9
	mul	r11
	xor	rdx, rax
	mov	qword ptr [rsp + 88], rdx
	mov	rax, qword ptr [r15 + r10 + 48]
	xor	rax, r9
	mul	r11
	xor	rdx, rax
	mov	qword ptr [rsp + 80], rdx
	mov	rax, qword ptr [r15 + r10 + 56]
	xor	rax, r9
	mul	r11
	mov	rbx, rdx
	xor	rbx, rax
	mov	rax, qword ptr [r15 + r10 + 64]
	xor	rax, r9
	mul	r11
	mov	r8, rdx
	xor	r8, rax
	mov	rax, qword ptr [r15 + r10 + 72]
	xor	rax, r9
	mul	r11
	mov	r12, rdx
	xor	r12, rax
	mov	rax, qword ptr [r15 + r10 + 80]
	xor	rax, r9
	mul	r11
	mov	r13, rdx
	xor	r13, rax
	mov	rax, qword ptr [r15 + r10 + 88]
	xor	rax, r9
	mul	r11
	mov	rbp, rdx
	xor	rbp, rax
	mov	rax, qword ptr [r15 + r10 + 96]
	xor	rax, r9
	mul	r11
	mov	rcx, rdx
	xor	rcx, rax
	mov	rax, qword ptr [r15 + r10 + 104]
	xor	rax, r9
	mul	r11
	mov	rsi, rdx
	xor	rsi, rax
	mov	rax, qword ptr [r15 + r10 + 112]
	xor	rax, r9
	mul	r11
	mov	rdi, rdx
	xor	rdi, rax
	mov	rax, qword ptr [r15 + r10 + 120]
	xor	rax, r9
	mul	r11
	xor	rdx, rax
	mov	rax, qword ptr [rsp + 8]
	mov	r14, qword ptr [rsp + 40]
	mov	qword ptr [rax + r10], r14
	mov	r14, qword ptr [rsp + 48]
	mov	qword ptr [rax + r10 + 8], r14
	mov	r14, qword ptr [rsp + 72]
	mov	qword ptr [rax + r10 + 16], r14
	mov	r14, qword ptr [rsp + 104]
	mov	qword ptr [rax + r10 + 24], r14
	mov	r14, qword ptr [rsp + 96]
	mov	qword ptr [rax + r10 + 32], r14
	mov	r14, qword ptr [rsp + 88]
	mov	qword ptr [rax + r10 + 40], r14
	mov	r14, qword ptr [rsp + 80]
	mov	qword ptr [rax + r10 + 48], r14
	mov	qword ptr [rax + r10 + 56], rbx
	mov	qword ptr [rax + r10 + 64], r8
	mov	qword ptr [rax + r10 + 72], r12
	mov	qword ptr [rax + r10 + 80], r13
	mov	qword ptr [rax + r10 + 88], rbp
	mov	qword ptr [rax + r10 + 96], rcx
	mov	qword ptr [rax + r10 + 104], rsi
	mov	qword ptr [rax + r10 + 112], rdi
	mov	qword ptr [rax + r10 + 120], rdx
	sub	r10, -128
	cmp	qword ptr [rsp + 56], r10
	jne	.LBB75_139
.LBB75_140:
	mov	r10, qword ptr [rsp + 64]
	test	r10, r10
	je	.LBB75_721
	mov	rax, qword ptr [rsp + 24]
	movabs	rcx, 1152921504606846960
	and	rax, rcx
	lea	rcx, [r15 + 8*rax]
	mov	rdx, qword ptr [rsp + 8]
	lea	rsi, [rdx + 8*rax]
	mov	rax, qword ptr [rsp + 16]
	mov	rdi, qword ptr [rax]
	xor	r9d, r9d
	movabs	r8, -4563226453097033507
	.p2align	4
.LBB75_142:
	mov	rax, qword ptr [rcx + 8*r9]
	xor	rax, rdi
	mul	r8
	xor	rdx, rax
	mov	qword ptr [rsi + 8*r9], rdx
	inc	r9
	cmp	r10, r9
	jne	.LBB75_142
	jmp	.LBB75_721
.LBB75_60:
	lea	rdi, [rsp + 144]
	call	qword ptr [rip + _RNvMNtNtCs7SjCkj1olcZ_12arrow_buffer6buffer4nullNtB2_10NullBuffer13valid_indices@GOTPCREL]
	mov	r9, qword ptr [rsp + 144]
	mov	rcx, qword ptr [rsp + 152]
	mov	r12, qword ptr [rsp + 160]
	mov	rsi, qword ptr [rsp + 168]
	mov	rdi, qword ptr [rsp + 176]
	mov	r8, qword ptr [rsp + 184]
	mov	r13, qword ptr [rsp + 192]
	mov	r10, qword ptr [rsp + 200]
	mov	r11, qword ptr [rbx + 32]
	mov	rax, qword ptr [rsp + 16]
	mov	rbx, qword ptr [rax]
	movabs	r15, -4563226453097033507
	jmp	.LBB75_61
	.p2align	4
.LBB75_64:
	mov	r13, rsi
	xor	r12d, r12d
	add	r10, 64
.LBB75_61:
	test	r13, r13
	jne	.LBB75_71
	cmp	r12, -2
	je	.LBB75_68
	cmp	r12, 1
	je	.LBB75_64
	test	rdi, rdi
	je	.LBB75_68
	cmp	rdi, r8
	je	.LBB75_68
	mov	r13, qword ptr [rdi]
	add	rdi, 8
	mov	r12, -1
	add	r10, 64
	jmp	.LBB75_61
	.p2align	4
.LBB75_68:
	dec	r9
	cmp	r9, -3
	ja	.LBB75_721
	mov	r12, -2
	mov	r13, rcx
	xor	r9d, r9d
	add	r10, 64
	jmp	.LBB75_61
.LBB75_71:
	rep		bsf	rbp, r13
	add	rbp, r10
	movzx	eax, word ptr [r11 + 2*rbp]
	xor	rax, rbx
	mul	r15
	xor	rdx, rax
	lea	rax, [r13 - 1]
	and	r13, rax
	mov	rax, qword ptr [rsp + 8]
	mov	qword ptr [rax + 8*rbp], rdx
	jmp	.LBB75_61
.LBB75_228:
	lea	rdi, [rsp + 144]
	call	qword ptr [rip + _RNvMNtNtCs7SjCkj1olcZ_12arrow_buffer6buffer4nullNtB2_10NullBuffer13valid_indices@GOTPCREL]
	mov	r9, qword ptr [rsp + 144]
	mov	rcx, qword ptr [rsp + 152]
	mov	r12, qword ptr [rsp + 160]
	mov	rsi, qword ptr [rsp + 168]
	mov	rdi, qword ptr [rsp + 176]
	mov	r8, qword ptr [rsp + 184]
	mov	r13, qword ptr [rsp + 192]
	mov	r10, qword ptr [rsp + 200]
	mov	r11, qword ptr [rbx + 32]
	mov	rax, qword ptr [rsp + 16]
	mov	rbx, qword ptr [rax]
	movabs	r15, -4563226453097033507
	jmp	.LBB75_229
	.p2align	4
.LBB75_232:
	mov	r13, rsi
	xor	r12d, r12d
	add	r10, 64
.LBB75_229:
	test	r13, r13
	jne	.LBB75_239
	cmp	r12, -2
	je	.LBB75_236
	cmp	r12, 1
	je	.LBB75_232
	test	rdi, rdi
	je	.LBB75_236
	cmp	rdi, r8
	je	.LBB75_236
	mov	r13, qword ptr [rdi]
	add	rdi, 8
	mov	r12, -1
	add	r10, 64
	jmp	.LBB75_229
	.p2align	4
.LBB75_236:
	dec	r9
	cmp	r9, -3
	ja	.LBB75_721
	mov	r12, -2
	mov	r13, rcx
	xor	r9d, r9d
	add	r10, 64
	jmp	.LBB75_229
.LBB75_239:
	rep		bsf	rbp, r13
	add	rbp, r10
	movzx	eax, word ptr [r11 + 2*rbp]
	xor	rax, rbx
	mul	r15
	xor	rdx, rax
	lea	rax, [r13 - 1]
	and	r13, rax
	mov	rax, qword ptr [rsp + 8]
	mov	qword ptr [rax + 8*rbp], rdx
	jmp	.LBB75_229
.LBB75_144:
	lea	rdi, [rsp + 144]
	call	qword ptr [rip + _RNvMNtNtCs7SjCkj1olcZ_12arrow_buffer6buffer4nullNtB2_10NullBuffer13valid_indices@GOTPCREL]
	mov	r9, qword ptr [rsp + 144]
	mov	rcx, qword ptr [rsp + 152]
	mov	r12, qword ptr [rsp + 160]
	mov	rsi, qword ptr [rsp + 168]
	mov	rdi, qword ptr [rsp + 176]
	mov	r8, qword ptr [rsp + 184]
	mov	r13, qword ptr [rsp + 192]
	mov	r10, qword ptr [rsp + 200]
	mov	r11, qword ptr [rbx + 32]
	mov	rax, qword ptr [rsp + 16]
	mov	rbx, qword ptr [rax]
	movabs	r15, -4563226453097033507
	jmp	.LBB75_145
	.p2align	4
.LBB75_148:
	mov	r13, rsi
	xor	r12d, r12d
	add	r10, 64
.LBB75_145:
	test	r13, r13
	jne	.LBB75_155
	cmp	r12, -2
	je	.LBB75_152
	cmp	r12, 1
	je	.LBB75_148
	test	rdi, rdi
	je	.LBB75_152
	cmp	rdi, r8
	je	.LBB75_152
	mov	r13, qword ptr [rdi]
	add	rdi, 8
	mov	r12, -1
	add	r10, 64
	jmp	.LBB75_145
	.p2align	4
.LBB75_152:
	dec	r9
	cmp	r9, -3
	ja	.LBB75_721
	mov	r12, -2
	mov	r13, rcx
	xor	r9d, r9d
	add	r10, 64
	jmp	.LBB75_145
.LBB75_155:
	rep		bsf	rbp, r13
	add	rbp, r10
	mov	rax, qword ptr [r11 + 8*rbp]
	xor	rax, rbx
	mul	r15
	xor	rdx, rax
	lea	rax, [r13 - 1]
	and	r13, rax
	mov	rax, qword ptr [rsp + 8]
	mov	qword ptr [rax + 8*rbp], rdx
	jmp	.LBB75_145
.LBB75_11:
	movabs	rsi, 1152921504606846960
	mov	rdx, qword ptr [rsp + 24]
	and	rsi, rdx
	mov	eax, edx
	and	eax, 15
	mov	qword ptr [rsp + 120], rax
	test	rdx, -16
	je	.LBB75_14
	mov	rax, qword ptr [rsp + 16]
	mov	rbp, qword ptr [rax]
	xor	r12d, r12d
	mov	qword ptr [rsp + 128], rsi
	.p2align	4
.LBB75_13:
	movzx	eax, byte ptr [rcx + r12]
	movzx	esi, byte ptr [rcx + r12 + 1]
	movzx	edi, byte ptr [rcx + r12 + 2]
	movzx	r9d, byte ptr [rcx + r12 + 3]
	movzx	r8d, byte ptr [rcx + r12 + 4]
	movzx	r13d, byte ptr [rcx + r12 + 5]
	movzx	r15d, byte ptr [rcx + r12 + 6]
	movzx	r10d, byte ptr [rcx + r12 + 7]
	movzx	ebx, byte ptr [rcx + r12 + 8]
	movzx	edx, byte ptr [rcx + r12 + 9]
	mov	qword ptr [rsp + 56], rdx
	movzx	edx, byte ptr [rcx + r12 + 10]
	mov	qword ptr [rsp + 80], rdx
	movzx	edx, byte ptr [rcx + r12 + 11]
	mov	qword ptr [rsp + 96], rdx
	movzx	edx, byte ptr [rcx + r12 + 12]
	mov	qword ptr [rsp + 72], rdx
	xor	rax, rbp
	movabs	r11, -4563226453097033507
	mul	r11
	xor	rdx, rax
	mov	qword ptr [rsp + 24], rdx
	xor	rsi, rbp
	mov	rax, rsi
	mul	r11
	mov	rsi, rax
	mov	r14, rdx
	xor	rdi, rbp
	mov	rax, rdi
	mul	r11
	xor	r14, rsi
	mov	qword ptr [rsp + 40], r14
	xor	rdx, rax
	mov	qword ptr [rsp + 48], rdx
	xor	r9, rbp
	mov	rax, r9
	mul	r11
	xor	rdx, rax
	mov	qword ptr [rsp + 104], rdx
	xor	r8, rbp
	mov	rax, r8
	mul	r11
	xor	rdx, rax
	mov	qword ptr [rsp + 88], rdx
	xor	r13, rbp
	mov	rax, r13
	mul	r11
	mov	rsi, rax
	mov	rdi, rdx
	xor	r15, rbp
	mov	rax, r15
	mul	r11
	xor	rdi, rsi
	mov	qword ptr [rsp + 64], rdi
	xor	rdx, rax
	mov	qword ptr [rsp + 112], rdx
	xor	r10, rbp
	mov	rax, r10
	mul	r11
	mov	r14, rdx
	xor	r14, rax
	xor	rbx, rbp
	mov	rax, rbx
	mul	r11
	mov	r9, rdx
	xor	r9, rax
	mov	rax, qword ptr [rsp + 56]
	xor	rax, rbp
	mul	r11
	mov	rsi, rax
	mov	rbx, rdx
	mov	rax, qword ptr [rsp + 80]
	xor	rax, rbp
	mul	r11
	mov	r15, rdx
	xor	rbx, rsi
	xor	r15, rax
	mov	rax, qword ptr [rsp + 96]
	xor	rax, rbp
	mul	r11
	mov	r10, rdx
	movzx	esi, byte ptr [rcx + r12 + 13]
	xor	r10, rax
	mov	rax, qword ptr [rsp + 72]
	xor	rax, rbp
	mul	r11
	mov	r8, rdx
	xor	r8, rax
	xor	rsi, rbp
	mov	rax, rsi
	mul	r11
	mov	rsi, rdx
	xor	rsi, rax
	movzx	eax, byte ptr [rcx + r12 + 14]
	xor	rax, rbp
	mul	r11
	mov	rdi, rdx
	xor	rdi, rax
	movzx	eax, byte ptr [rcx + r12 + 15]
	xor	rax, rbp
	mul	r11
	xor	rdx, rax
	mov	rax, qword ptr [rsp + 8]
	mov	r13, qword ptr [rsp + 24]
	mov	qword ptr [rax + 8*r12], r13
	mov	r11, qword ptr [rsp + 40]
	mov	qword ptr [rax + 8*r12 + 8], r11
	mov	r11, qword ptr [rsp + 48]
	mov	qword ptr [rax + 8*r12 + 16], r11
	mov	r11, qword ptr [rsp + 104]
	mov	qword ptr [rax + 8*r12 + 24], r11
	mov	r11, qword ptr [rsp + 88]
	mov	qword ptr [rax + 8*r12 + 32], r11
	mov	r11, qword ptr [rsp + 64]
	mov	qword ptr [rax + 8*r12 + 40], r11
	mov	r11, qword ptr [rsp + 112]
	mov	qword ptr [rax + 8*r12 + 48], r11
	mov	qword ptr [rax + 8*r12 + 56], r14
	mov	qword ptr [rax + 8*r12 + 64], r9
	mov	qword ptr [rax + 8*r12 + 72], rbx
	mov	qword ptr [rax + 8*r12 + 80], r15
	mov	qword ptr [rax + 8*r12 + 88], r10
	mov	qword ptr [rax + 8*r12 + 96], r8
	mov	qword ptr [rax + 8*r12 + 104], rsi
	mov	rsi, qword ptr [rsp + 128]
	mov	qword ptr [rax + 8*r12 + 112], rdi
	mov	qword ptr [rax + 8*r12 + 120], rdx
	add	r12, 16
	cmp	rsi, r12
	jne	.LBB75_13
.LBB75_14:
	mov	r10, qword ptr [rsp + 120]
	test	r10, r10
	je	.LBB75_721
	add	rcx, rsi
	mov	rax, qword ptr [rsp + 8]
	lea	rsi, [rax + 8*rsi]
	mov	rax, qword ptr [rsp + 16]
	mov	rdi, qword ptr [rax]
	xor	r9d, r9d
	movabs	r8, -4563226453097033507
	.p2align	4
.LBB75_16:
	movzx	eax, byte ptr [rcx + r9]
	xor	rax, rdi
	mul	r8
	xor	rdx, rax
	mov	qword ptr [rsi + 8*r9], rdx
	inc	r9
	cmp	r10, r9
	jne	.LBB75_16
	jmp	.LBB75_721
.LBB75_95:
	mov	rsi, qword ptr [rsp + 24]
	mov	edx, esi
	and	edx, 15
	mov	qword ptr [rsp + 136], rdx
	shr	rax, 6
	shr	rsi, 4
	cmp	rax, rsi
	cmovb	rsi, rax
	mov	qword ptr [rsp + 120], rsi
	test	rsi, rsi
	je	.LBB75_98
	mov	rax, qword ptr [rsp + 16]
	mov	r15, qword ptr [rax]
	shl	qword ptr [rsp + 120], 6
	xor	r12d, r12d
	.p2align	4
.LBB75_97:
	mov	eax, dword ptr [rcx + r12]
	mov	esi, dword ptr [rcx + r12 + 4]
	mov	edi, dword ptr [rcx + r12 + 8]
	mov	r9d, dword ptr [rcx + r12 + 12]
	mov	r8d, dword ptr [rcx + r12 + 16]
	mov	ebp, dword ptr [rcx + r12 + 20]
	mov	r13d, dword ptr [rcx + r12 + 24]
	mov	r10d, dword ptr [rcx + r12 + 28]
	mov	ebx, dword ptr [rcx + r12 + 32]
	mov	edx, dword ptr [rcx + r12 + 36]
	mov	qword ptr [rsp + 64], rdx
	mov	edx, dword ptr [rcx + r12 + 40]
	mov	qword ptr [rsp + 56], rdx
	mov	edx, dword ptr [rcx + r12 + 44]
	mov	qword ptr [rsp + 88], rdx
	mov	edx, dword ptr [rcx + r12 + 48]
	mov	qword ptr [rsp + 104], rdx
	xor	rax, r15
	movabs	r11, -4563226453097033507
	mul	r11
	xor	rdx, rax
	mov	qword ptr [rsp + 40], rdx
	xor	rsi, r15
	mov	rax, rsi
	mul	r11
	mov	rsi, rax
	mov	r14, rdx
	xor	rdi, r15
	mov	rax, rdi
	mul	r11
	xor	r14, rsi
	mov	qword ptr [rsp + 48], r14
	xor	rdx, rax
	mov	qword ptr [rsp + 72], rdx
	xor	r9, r15
	mov	rax, r9
	mul	r11
	xor	rdx, rax
	mov	qword ptr [rsp + 96], rdx
	xor	r8, r15
	mov	rax, r8
	mul	r11
	xor	rdx, rax
	mov	qword ptr [rsp + 80], rdx
	xor	rbp, r15
	mov	rax, rbp
	mul	r11
	mov	rsi, rax
	mov	rdi, rdx
	xor	r13, r15
	mov	rax, r13
	mul	r11
	xor	rdi, rsi
	mov	qword ptr [rsp + 112], rdi
	xor	rdx, rax
	mov	qword ptr [rsp + 128], rdx
	xor	r10, r15
	mov	rax, r10
	mul	r11
	mov	r14, rdx
	xor	r14, rax
	xor	rbx, r15
	mov	rax, rbx
	mul	r11
	mov	r9, rdx
	xor	r9, rax
	mov	rax, qword ptr [rsp + 64]
	xor	rax, r15
	mul	r11
	mov	rsi, rax
	mov	rbx, rdx
	mov	rax, qword ptr [rsp + 56]
	xor	rax, r15
	mul	r11
	mov	r13, rdx
	xor	rbx, rsi
	xor	r13, rax
	mov	rax, qword ptr [rsp + 88]
	xor	rax, r15
	mul	r11
	mov	r10, rdx
	mov	esi, dword ptr [rcx + r12 + 52]
	xor	r10, rax
	mov	rax, qword ptr [rsp + 104]
	xor	rax, r15
	mul	r11
	mov	r8, rdx
	xor	r8, rax
	xor	rsi, r15
	mov	rax, rsi
	mul	r11
	mov	rsi, rdx
	xor	rsi, rax
	mov	eax, dword ptr [rcx + r12 + 56]
	xor	rax, r15
	mul	r11
	mov	rdi, rdx
	xor	rdi, rax
	mov	eax, dword ptr [rcx + r12 + 60]
	xor	rax, r15
	mul	r11
	xor	rdx, rax
	mov	rax, qword ptr [rsp + 8]
	mov	rbp, qword ptr [rsp + 40]
	mov	qword ptr [rax + 2*r12], rbp
	mov	r11, qword ptr [rsp + 48]
	mov	qword ptr [rax + 2*r12 + 8], r11
	mov	r11, qword ptr [rsp + 72]
	mov	qword ptr [rax + 2*r12 + 16], r11
	mov	r11, qword ptr [rsp + 96]
	mov	qword ptr [rax + 2*r12 + 24], r11
	mov	r11, qword ptr [rsp + 80]
	mov	qword ptr [rax + 2*r12 + 32], r11
	mov	r11, qword ptr [rsp + 112]
	mov	qword ptr [rax + 2*r12 + 40], r11
	mov	r11, qword ptr [rsp + 128]
	mov	qword ptr [rax + 2*r12 + 48], r11
	mov	qword ptr [rax + 2*r12 + 56], r14
	mov	qword ptr [rax + 2*r12 + 64], r9
	mov	qword ptr [rax + 2*r12 + 72], rbx
	mov	qword ptr [rax + 2*r12 + 80], r13
	mov	qword ptr [rax + 2*r12 + 88], r10
	mov	qword ptr [rax + 2*r12 + 96], r8
	mov	qword ptr [rax + 2*r12 + 104], rsi
	mov	qword ptr [rax + 2*r12 + 112], rdi
	mov	qword ptr [rax + 2*r12 + 120], rdx
	add	r12, 64
	cmp	qword ptr [rsp + 120], r12
	jne	.LBB75_97
.LBB75_98:
	mov	r10, qword ptr [rsp + 136]
	test	r10, r10
	je	.LBB75_721
	mov	rax, qword ptr [rsp + 24]
	movabs	rdx, 1152921504606846960
	and	rax, rdx
	lea	rcx, [rcx + 4*rax]
	mov	rdx, qword ptr [rsp + 8]
	lea	rsi, [rdx + 8*rax]
	mov	rax, qword ptr [rsp + 16]
	mov	rdi, qword ptr [rax]
	xor	r9d, r9d
	movabs	r8, -4563226453097033507
	.p2align	4
.LBB75_100:
	mov	eax, dword ptr [rcx + 4*r9]
	xor	rax, rdi
	mul	r8
	xor	rdx, rax
	mov	qword ptr [rsi + 8*r9], rdx
	inc	r9
	cmp	r10, r9
	jne	.LBB75_100
	jmp	.LBB75_721
.LBB75_263:
	mov	rsi, qword ptr [rsp + 24]
	mov	edx, esi
	and	edx, 15
	mov	qword ptr [rsp + 136], rdx
	shr	rax, 6
	shr	rsi, 4
	cmp	rax, rsi
	cmovb	rsi, rax
	mov	qword ptr [rsp + 120], rsi
	test	rsi, rsi
	je	.LBB75_266
	mov	rax, qword ptr [rsp + 16]
	mov	r15, qword ptr [rax]
	shl	qword ptr [rsp + 120], 6
	xor	r12d, r12d
	.p2align	4
.LBB75_265:
	mov	eax, dword ptr [rcx + r12]
	mov	esi, dword ptr [rcx + r12 + 4]
	mov	edi, dword ptr [rcx + r12 + 8]
	mov	r9d, dword ptr [rcx + r12 + 12]
	mov	r8d, dword ptr [rcx + r12 + 16]
	mov	ebp, dword ptr [rcx + r12 + 20]
	mov	r13d, dword ptr [rcx + r12 + 24]
	mov	r10d, dword ptr [rcx + r12 + 28]
	mov	ebx, dword ptr [rcx + r12 + 32]
	mov	edx, dword ptr [rcx + r12 + 36]
	mov	qword ptr [rsp + 64], rdx
	mov	edx, dword ptr [rcx + r12 + 40]
	mov	qword ptr [rsp + 56], rdx
	mov	edx, dword ptr [rcx + r12 + 44]
	mov	qword ptr [rsp + 88], rdx
	mov	edx, dword ptr [rcx + r12 + 48]
	mov	qword ptr [rsp + 104], rdx
	xor	rax, r15
	movabs	r11, -4563226453097033507
	mul	r11
	xor	rdx, rax
	mov	qword ptr [rsp + 40], rdx
	xor	rsi, r15
	mov	rax, rsi
	mul	r11
	mov	rsi, rax
	mov	r14, rdx
	xor	rdi, r15
	mov	rax, rdi
	mul	r11
	xor	r14, rsi
	mov	qword ptr [rsp + 48], r14
	xor	rdx, rax
	mov	qword ptr [rsp + 72], rdx
	xor	r9, r15
	mov	rax, r9
	mul	r11
	xor	rdx, rax
	mov	qword ptr [rsp + 96], rdx
	xor	r8, r15
	mov	rax, r8
	mul	r11
	xor	rdx, rax
	mov	qword ptr [rsp + 80], rdx
	xor	rbp, r15
	mov	rax, rbp
	mul	r11
	mov	rsi, rax
	mov	rdi, rdx
	xor	r13, r15
	mov	rax, r13
	mul	r11
	xor	rdi, rsi
	mov	qword ptr [rsp + 112], rdi
	xor	rdx, rax
	mov	qword ptr [rsp + 128], rdx
	xor	r10, r15
	mov	rax, r10
	mul	r11
	mov	r14, rdx
	xor	r14, rax
	xor	rbx, r15
	mov	rax, rbx
	mul	r11
	mov	r9, rdx
	xor	r9, rax
	mov	rax, qword ptr [rsp + 64]
	xor	rax, r15
	mul	r11
	mov	rsi, rax
	mov	rbx, rdx
	mov	rax, qword ptr [rsp + 56]
	xor	rax, r15
	mul	r11
	mov	r13, rdx
	xor	rbx, rsi
	xor	r13, rax
	mov	rax, qword ptr [rsp + 88]
	xor	rax, r15
	mul	r11
	mov	r10, rdx
	mov	esi, dword ptr [rcx + r12 + 52]
	xor	r10, rax
	mov	rax, qword ptr [rsp + 104]
	xor	rax, r15
	mul	r11
	mov	r8, rdx
	xor	r8, rax
	xor	rsi, r15
	mov	rax, rsi
	mul	r11
	mov	rsi, rdx
	xor	rsi, rax
	mov	eax, dword ptr [rcx + r12 + 56]
	xor	rax, r15
	mul	r11
	mov	rdi, rdx
	xor	rdi, rax
	mov	eax, dword ptr [rcx + r12 + 60]
	xor	rax, r15
	mul	r11
	xor	rdx, rax
	mov	rax, qword ptr [rsp + 8]
	mov	rbp, qword ptr [rsp + 40]
	mov	qword ptr [rax + 2*r12], rbp
	mov	r11, qword ptr [rsp + 48]
	mov	qword ptr [rax + 2*r12 + 8], r11
	mov	r11, qword ptr [rsp + 72]
	mov	qword ptr [rax + 2*r12 + 16], r11
	mov	r11, qword ptr [rsp + 96]
	mov	qword ptr [rax + 2*r12 + 24], r11
	mov	r11, qword ptr [rsp + 80]
	mov	qword ptr [rax + 2*r12 + 32], r11
	mov	r11, qword ptr [rsp + 112]
	mov	qword ptr [rax + 2*r12 + 40], r11
	mov	r11, qword ptr [rsp + 128]
	mov	qword ptr [rax + 2*r12 + 48], r11
	mov	qword ptr [rax + 2*r12 + 56], r14
	mov	qword ptr [rax + 2*r12 + 64], r9
	mov	qword ptr [rax + 2*r12 + 72], rbx
	mov	qword ptr [rax + 2*r12 + 80], r13
	mov	qword ptr [rax + 2*r12 + 88], r10
	mov	qword ptr [rax + 2*r12 + 96], r8
	mov	qword ptr [rax + 2*r12 + 104], rsi
	mov	qword ptr [rax + 2*r12 + 112], rdi
	mov	qword ptr [rax + 2*r12 + 120], rdx
	add	r12, 64
	cmp	qword ptr [rsp + 120], r12
	jne	.LBB75_265
.LBB75_266:
	mov	r10, qword ptr [rsp + 136]
	test	r10, r10
	je	.LBB75_721
	mov	rax, qword ptr [rsp + 24]
	movabs	rdx, 1152921504606846960
	and	rax, rdx
	lea	rcx, [rcx + 4*rax]
	mov	rdx, qword ptr [rsp + 8]
	lea	rsi, [rdx + 8*rax]
	mov	rax, qword ptr [rsp + 16]
	mov	rdi, qword ptr [rax]
	xor	r9d, r9d
	movabs	r8, -4563226453097033507
	.p2align	4
.LBB75_268:
	mov	eax, dword ptr [rcx + 4*r9]
	xor	rax, rdi
	mul	r8
	xor	rdx, rax
	mov	qword ptr [rsi + 8*r9], rdx
	inc	r9
	cmp	r10, r9
	jne	.LBB75_268
	jmp	.LBB75_721
.LBB75_305:
	mov	rcx, qword ptr [rsp + 24]
	mov	edx, ecx
	and	edx, 15
	mov	qword ptr [rsp + 64], rdx
	shr	rax, 7
	shr	rcx, 4
	cmp	rax, rcx
	cmovb	rcx, rax
	mov	qword ptr [rsp + 56], rcx
	test	rcx, rcx
	je	.LBB75_308
	mov	rax, qword ptr [rsp + 16]
	mov	r9, qword ptr [rax]
	shl	qword ptr [rsp + 56], 7
	xor	r10d, r10d
	movabs	r11, -4563226453097033507
	.p2align	4
.LBB75_307:
	mov	rax, qword ptr [r15 + r10]
	xor	rax, r9
	mul	r11
	xor	rdx, rax
	mov	qword ptr [rsp + 40], rdx
	mov	rax, qword ptr [r15 + r10 + 8]
	xor	rax, r9
	mul	r11
	xor	rdx, rax
	mov	qword ptr [rsp + 48], rdx
	mov	rax, qword ptr [r15 + r10 + 16]
	xor	rax, r9
	mul	r11
	xor	rdx, rax
	mov	qword ptr [rsp + 72], rdx
	mov	rax, qword ptr [r15 + r10 + 24]
	xor	rax, r9
	mul	r11
	xor	rdx, rax
	mov	qword ptr [rsp + 104], rdx
	mov	rax, qword ptr [r15 + r10 + 32]
	xor	rax, r9
	mul	r11
	xor	rdx, rax
	mov	qword ptr [rsp + 96], rdx
	mov	rax, qword ptr [r15 + r10 + 40]
	xor	rax, r9
	mul	r11
	xor	rdx, rax
	mov	qword ptr [rsp + 88], rdx
	mov	rax, qword ptr [r15 + r10 + 48]
	xor	rax, r9
	mul	r11
	xor	rdx, rax
	mov	qword ptr [rsp + 80], rdx
	mov	rax, qword ptr [r15 + r10 + 56]
	xor	rax, r9
	mul	r11
	mov	rbx, rdx
	xor	rbx, rax
	mov	rax, qword ptr [r15 + r10 + 64]
	xor	rax, r9
	mul	r11
	mov	r8, rdx
	xor	r8, rax
	mov	rax, qword ptr [r15 + r10 + 72]
	xor	rax, r9
	mul	r11
	mov	r12, rdx
	xor	r12, rax
	mov	rax, qword ptr [r15 + r10 + 80]
	xor	rax, r9
	mul	r11
	mov	r13, rdx
	xor	r13, rax
	mov	rax, qword ptr [r15 + r10 + 88]
	xor	rax, r9
	mul	r11
	mov	rbp, rdx
	xor	rbp, rax
	mov	rax, qword ptr [r15 + r10 + 96]
	xor	rax, r9
	mul	r11
	mov	rcx, rdx
	xor	rcx, rax
	mov	rax, qword ptr [r15 + r10 + 104]
	xor	rax, r9
	mul	r11
	mov	rsi, rdx
	xor	rsi, rax
	mov	rax, qword ptr [r15 + r10 + 112]
	xor	rax, r9
	mul	r11
	mov	rdi, rdx
	xor	rdi, rax
	mov	rax, qword ptr [r15 + r10 + 120]
	xor	rax, r9
	mul	r11
	xor	rdx, rax
	mov	rax, qword ptr [rsp + 8]
	mov	r14, qword ptr [rsp + 40]
	mov	qword ptr [rax + r10], r14
	mov	r14, qword ptr [rsp + 48]
	mov	qword ptr [rax + r10 + 8], r14
	mov	r14, qword ptr [rsp + 72]
	mov	qword ptr [rax + r10 + 16], r14
	mov	r14, qword ptr [rsp + 104]
	mov	qword ptr [rax + r10 + 24], r14
	mov	r14, qword ptr [rsp + 96]
	mov	qword ptr [rax + r10 + 32], r14
	mov	r14, qword ptr [rsp + 88]
	mov	qword ptr [rax + r10 + 40], r14
	mov	r14, qword ptr [rsp + 80]
	mov	qword ptr [rax + r10 + 48], r14
	mov	qword ptr [rax + r10 + 56], rbx
	mov	qword ptr [rax + r10 + 64], r8
	mov	qword ptr [rax + r10 + 72], r12
	mov	qword ptr [rax + r10 + 80], r13
	mov	qword ptr [rax + r10 + 88], rbp
	mov	qword ptr [rax + r10 + 96], rcx
	mov	qword ptr [rax + r10 + 104], rsi
	mov	qword ptr [rax + r10 + 112], rdi
	mov	qword ptr [rax + r10 + 120], rdx
	sub	r10, -128
	cmp	qword ptr [rsp + 56], r10
	jne	.LBB75_307
.LBB75_308:
	mov	r10, qword ptr [rsp + 64]
	test	r10, r10
	je	.LBB75_721
	movabs	rax, 1152921504606846960
	mov	rdx, qword ptr [rsp + 24]
	and	rdx, rax
	lea	rcx, [r15 + 8*rdx]
	mov	rax, qword ptr [rsp + 8]
	lea	rsi, [rax + 8*rdx]
	mov	rax, qword ptr [rsp + 16]
	mov	rdi, qword ptr [rax]
	xor	r9d, r9d
	movabs	r8, -4563226453097033507
	.p2align	4
.LBB75_310:
	mov	rax, qword ptr [rcx + 8*r9]
	xor	rax, rdi
	mul	r8
	xor	rdx, rax
	mov	qword ptr [rsi + 8*r9], rdx
	inc	r9
	cmp	r10, r9
	jne	.LBB75_310
	jmp	.LBB75_721
.LBB75_18:
	lea	rdi, [rsp + 144]
	call	qword ptr [rip + _RNvMNtNtCs7SjCkj1olcZ_12arrow_buffer6buffer4nullNtB2_10NullBuffer13valid_indices@GOTPCREL]
	mov	r9, qword ptr [rsp + 144]
	mov	rcx, qword ptr [rsp + 152]
	mov	r12, qword ptr [rsp + 160]
	mov	rsi, qword ptr [rsp + 168]
	mov	rdi, qword ptr [rsp + 176]
	mov	r8, qword ptr [rsp + 184]
	mov	r13, qword ptr [rsp + 192]
	mov	r10, qword ptr [rsp + 200]
	mov	r11, qword ptr [rbx + 32]
	mov	rax, qword ptr [rsp + 16]
	mov	rbx, qword ptr [rax]
	movabs	r15, -4563226453097033507
	jmp	.LBB75_19
	.p2align	4
.LBB75_22:
	mov	r13, rsi
	xor	r12d, r12d
	add	r10, 64
.LBB75_19:
	test	r13, r13
	jne	.LBB75_29
	cmp	r12, -2
	je	.LBB75_26
	cmp	r12, 1
	je	.LBB75_22
	test	rdi, rdi
	je	.LBB75_26
	cmp	rdi, r8
	je	.LBB75_26
	mov	r13, qword ptr [rdi]
	add	rdi, 8
	mov	r12, -1
	add	r10, 64
	jmp	.LBB75_19
	.p2align	4
.LBB75_26:
	dec	r9
	cmp	r9, -3
	ja	.LBB75_721
	mov	r12, -2
	mov	r13, rcx
	xor	r9d, r9d
	add	r10, 64
	jmp	.LBB75_19
.LBB75_29:
	rep		bsf	rbp, r13
	add	rbp, r10
	movzx	eax, byte ptr [r11 + rbp]
	xor	rax, rbx
	mul	r15
	xor	rdx, rax
	lea	rax, [r13 - 1]
	and	r13, rax
	mov	rax, qword ptr [rsp + 8]
	mov	qword ptr [rax + 8*rbp], rdx
	jmp	.LBB75_19
.LBB75_102:
	lea	rdi, [rsp + 144]
	call	qword ptr [rip + _RNvMNtNtCs7SjCkj1olcZ_12arrow_buffer6buffer4nullNtB2_10NullBuffer13valid_indices@GOTPCREL]
	mov	r9, qword ptr [rsp + 144]
	mov	rcx, qword ptr [rsp + 152]
	mov	r12, qword ptr [rsp + 160]
	mov	rsi, qword ptr [rsp + 168]
	mov	rdi, qword ptr [rsp + 176]
	mov	r8, qword ptr [rsp + 184]
	mov	r13, qword ptr [rsp + 192]
	mov	r10, qword ptr [rsp + 200]
	mov	r11, qword ptr [rbx + 32]
	mov	rax, qword ptr [rsp + 16]
	mov	rbx, qword ptr [rax]
	movabs	r15, -4563226453097033507
	jmp	.LBB75_103
	.p2align	4
.LBB75_106:
	mov	r13, rsi
	xor	r12d, r12d
	add	r10, 64
.LBB75_103:
	test	r13, r13
	jne	.LBB75_113
	cmp	r12, -2
	je	.LBB75_110
	cmp	r12, 1
	je	.LBB75_106
	test	rdi, rdi
	je	.LBB75_110
	cmp	rdi, r8
	je	.LBB75_110
	mov	r13, qword ptr [rdi]
	add	rdi, 8
	mov	r12, -1
	add	r10, 64
	jmp	.LBB75_103
	.p2align	4
.LBB75_110:
	dec	r9
	cmp	r9, -3
	ja	.LBB75_721
	mov	r12, -2
	mov	r13, rcx
	xor	r9d, r9d
	add	r10, 64
	jmp	.LBB75_103
.LBB75_113:
	rep		bsf	rbp, r13
	add	rbp, r10
	mov	eax, dword ptr [r11 + 4*rbp]
	xor	rax, rbx
	mul	r15
	xor	rdx, rax
	lea	rax, [r13 - 1]
	and	r13, rax
	mov	rax, qword ptr [rsp + 8]
	mov	qword ptr [rax + 8*rbp], rdx
	jmp	.LBB75_103
.LBB75_519:
	mov	rcx, qword ptr [rsp + 24]
	mov	edx, ecx
	and	edx, 15
	mov	qword ptr [rsp + 64], rdx
	shr	rax, 7
	shr	rcx, 4
	cmp	rax, rcx
	cmovb	rcx, rax
	mov	qword ptr [rsp + 56], rcx
	test	rcx, rcx
	je	.LBB75_522
	mov	rax, qword ptr [rsp + 16]
	mov	r9, qword ptr [rax]
	shl	qword ptr [rsp + 56], 7
	xor	r10d, r10d
	movabs	r11, -4563226453097033507
	.p2align	4
.LBB75_521:
	mov	rax, qword ptr [r15 + r10]
	xor	rax, r9
	mul	r11
	xor	rdx, rax
	mov	qword ptr [rsp + 40], rdx
	mov	rax, qword ptr [r15 + r10 + 8]
	xor	rax, r9
	mul	r11
	xor	rdx, rax
	mov	qword ptr [rsp + 48], rdx
	mov	rax, qword ptr [r15 + r10 + 16]
	xor	rax, r9
	mul	r11
	xor	rdx, rax
	mov	qword ptr [rsp + 72], rdx
	mov	rax, qword ptr [r15 + r10 + 24]
	xor	rax, r9
	mul	r11
	xor	rdx, rax
	mov	qword ptr [rsp + 104], rdx
	mov	rax, qword ptr [r15 + r10 + 32]
	xor	rax, r9
	mul	r11
	xor	rdx, rax
	mov	qword ptr [rsp + 96], rdx
	mov	rax, qword ptr [r15 + r10 + 40]
	xor	rax, r9
	mul	r11
	xor	rdx, rax
	mov	qword ptr [rsp + 88], rdx
	mov	rax, qword ptr [r15 + r10 + 48]
	xor	rax, r9
	mul	r11
	xor	rdx, rax
	mov	qword ptr [rsp + 80], rdx
	mov	rax, qword ptr [r15 + r10 + 56]
	xor	rax, r9
	mul	r11
	mov	rbx, rdx
	xor	rbx, rax
	mov	rax, qword ptr [r15 + r10 + 64]
	xor	rax, r9
	mul	r11
	mov	r8, rdx
	xor	r8, rax
	mov	rax, qword ptr [r15 + r10 + 72]
	xor	rax, r9
	mul	r11
	mov	r12, rdx
	xor	r12, rax
	mov	rax, qword ptr [r15 + r10 + 80]
	xor	rax, r9
	mul	r11
	mov	r13, rdx
	xor	r13, rax
	mov	rax, qword ptr [r15 + r10 + 88]
	xor	rax, r9
	mul	r11
	mov	rbp, rdx
	xor	rbp, rax
	mov	rax, qword ptr [r15 + r10 + 96]
	xor	rax, r9
	mul	r11
	mov	rcx, rdx
	xor	rcx, rax
	mov	rax, qword ptr [r15 + r10 + 104]
	xor	rax, r9
	mul	r11
	mov	rsi, rdx
	xor	rsi, rax
	mov	rax, qword ptr [r15 + r10 + 112]
	xor	rax, r9
	mul	r11
	mov	rdi, rdx
	xor	rdi, rax
	mov	rax, qword ptr [r15 + r10 + 120]
	xor	rax, r9
	mul	r11
	xor	rdx, rax
	mov	rax, qword ptr [rsp + 8]
	mov	r14, qword ptr [rsp + 40]
	mov	qword ptr [rax + r10], r14
	mov	r14, qword ptr [rsp + 48]
	mov	qword ptr [rax + r10 + 8], r14
	mov	r14, qword ptr [rsp + 72]
	mov	qword ptr [rax + r10 + 16], r14
	mov	r14, qword ptr [rsp + 104]
	mov	qword ptr [rax + r10 + 24], r14
	mov	r14, qword ptr [rsp + 96]
	mov	qword ptr [rax + r10 + 32], r14
	mov	r14, qword ptr [rsp + 88]
	mov	qword ptr [rax + r10 + 40], r14
	mov	r14, qword ptr [rsp + 80]
	mov	qword ptr [rax + r10 + 48], r14
	mov	qword ptr [rax + r10 + 56], rbx
	mov	qword ptr [rax + r10 + 64], r8
	mov	qword ptr [rax + r10 + 72], r12
	mov	qword ptr [rax + r10 + 80], r13
	mov	qword ptr [rax + r10 + 88], rbp
	mov	qword ptr [rax + r10 + 96], rcx
	mov	qword ptr [rax + r10 + 104], rsi
	mov	qword ptr [rax + r10 + 112], rdi
	mov	qword ptr [rax + r10 + 120], rdx
	sub	r10, -128
	cmp	qword ptr [rsp + 56], r10
	jne	.LBB75_521
.LBB75_522:
	mov	r10, qword ptr [rsp + 64]
	test	r10, r10
	je	.LBB75_721
	mov	rax, qword ptr [rsp + 24]
	movabs	rcx, 1152921504606846960
	and	rax, rcx
	lea	rcx, [r15 + 8*rax]
	mov	rdx, qword ptr [rsp + 8]
	lea	rsi, [rdx + 8*rax]
	mov	rax, qword ptr [rsp + 16]
	mov	rdi, qword ptr [rax]
	xor	r9d, r9d
	movabs	r8, -4563226453097033507
	.p2align	4
.LBB75_524:
	mov	rax, qword ptr [rcx + 8*r9]
	xor	rax, rdi
	mul	r8
	xor	rdx, rax
	mov	qword ptr [rsi + 8*r9], rdx
	inc	r9
	cmp	r10, r9
	jne	.LBB75_524
	jmp	.LBB75_721
.LBB75_270:
	lea	rdi, [rsp + 144]
	call	qword ptr [rip + _RNvMNtNtCs7SjCkj1olcZ_12arrow_buffer6buffer4nullNtB2_10NullBuffer13valid_indices@GOTPCREL]
	mov	r9, qword ptr [rsp + 144]
	mov	rcx, qword ptr [rsp + 152]
	mov	r12, qword ptr [rsp + 160]
	mov	rsi, qword ptr [rsp + 168]
	mov	rdi, qword ptr [rsp + 176]
	mov	r8, qword ptr [rsp + 184]
	mov	r13, qword ptr [rsp + 192]
	mov	r10, qword ptr [rsp + 200]
	mov	r11, qword ptr [rbx + 32]
	mov	rax, qword ptr [rsp + 16]
	mov	rbx, qword ptr [rax]
	movabs	r15, -4563226453097033507
	jmp	.LBB75_271
	.p2align	4
.LBB75_274:
	mov	r13, rsi
	xor	r12d, r12d
	add	r10, 64
.LBB75_271:
	test	r13, r13
	jne	.LBB75_281
	cmp	r12, -2
	je	.LBB75_278
	cmp	r12, 1
	je	.LBB75_274
	test	rdi, rdi
	je	.LBB75_278
	cmp	rdi, r8
	je	.LBB75_278
	mov	r13, qword ptr [rdi]
	add	rdi, 8
	mov	r12, -1
	add	r10, 64
	jmp	.LBB75_271
	.p2align	4
.LBB75_278:
	dec	r9
	cmp	r9, -3
	ja	.LBB75_721
	mov	r12, -2
	mov	r13, rcx
	xor	r9d, r9d
	add	r10, 64
	jmp	.LBB75_271
.LBB75_281:
	rep		bsf	rbp, r13
	add	rbp, r10
	mov	eax, dword ptr [r11 + 4*rbp]
	xor	rax, rbx
	mul	r15
	xor	rdx, rax
	lea	rax, [r13 - 1]
	and	r13, rax
	mov	rax, qword ptr [rsp + 8]
	mov	qword ptr [rax + 8*rbp], rdx
	jmp	.LBB75_271
.LBB75_561:
	movabs	r8, -4563226453097033507
	mov	r15, qword ptr [rsp + 24]
	mov	ecx, r15d
	and	ecx, 15
	mov	qword ptr [rsp + 112], rcx
	shr	rax, 8
	shr	r15, 4
	cmp	rax, r15
	cmovb	r15, rax
	mov	qword ptr [rsp + 64], r15
	test	r15, r15
	je	.LBB75_564
	mov	rax, qword ptr [rsp + 16]
	mov	r11, qword ptr [rax]
	shl	qword ptr [rsp + 64], 7
	xor	r10d, r10d
	mov	r9, qword ptr [rsp + 8]
	.p2align	4
.LBB75_563:
	mov	rcx, qword ptr [r13 + 2*r10]
	xor	rcx, r11
	mov	rax, qword ptr [r13 + 2*r10 + 8]
	xor	rax, r8
	mul	rcx
	xor	rdx, rax
	mov	qword ptr [rsp + 40], rdx
	mov	rcx, qword ptr [r13 + 2*r10 + 16]
	xor	rcx, r11
	mov	rax, qword ptr [r13 + 2*r10 + 24]
	xor	rax, r8
	mul	rcx
	mov	rcx, rax
	mov	rsi, rdx
	mov	rdx, qword ptr [r13 + 2*r10 + 32]
	xor	rdx, r11
	mov	rax, qword ptr [r13 + 2*r10 + 40]
	xor	rax, r8
	mul	rdx
	xor	rsi, rcx
	mov	qword ptr [rsp + 48], rsi
	xor	rdx, rax
	mov	qword ptr [rsp + 72], rdx
	mov	rcx, qword ptr [r13 + 2*r10 + 48]
	xor	rcx, r11
	mov	rax, qword ptr [r13 + 2*r10 + 56]
	xor	rax, r8
	mul	rcx
	mov	rcx, rax
	mov	rsi, rdx
	mov	rdx, qword ptr [r13 + 2*r10 + 64]
	xor	rdx, r11
	mov	rax, qword ptr [r13 + 2*r10 + 72]
	xor	rax, r8
	mul	rdx
	xor	rsi, rcx
	mov	qword ptr [rsp + 104], rsi
	xor	rdx, rax
	mov	qword ptr [rsp + 96], rdx
	mov	rcx, qword ptr [r13 + 2*r10 + 80]
	xor	rcx, r11
	mov	rax, qword ptr [r13 + 2*r10 + 88]
	xor	rax, r8
	mul	rcx
	mov	rcx, rax
	mov	rsi, rdx
	mov	rdx, qword ptr [r13 + 2*r10 + 96]
	xor	rdx, r11
	mov	rax, qword ptr [r13 + 2*r10 + 104]
	xor	rax, r8
	mul	rdx
	xor	rsi, rcx
	mov	qword ptr [rsp + 88], rsi
	xor	rdx, rax
	mov	qword ptr [rsp + 80], rdx
	mov	rcx, qword ptr [r13 + 2*r10 + 112]
	xor	rcx, r11
	mov	rax, qword ptr [r13 + 2*r10 + 120]
	xor	rax, r8
	mul	rcx
	mov	rcx, rax
	mov	rsi, rdx
	mov	rdx, qword ptr [r13 + 2*r10 + 128]
	xor	rdx, r11
	mov	rax, qword ptr [r13 + 2*r10 + 136]
	xor	rax, r8
	mul	rdx
	mov	rbp, rdx
	xor	rsi, rcx
	mov	qword ptr [rsp + 56], rsi
	xor	rbp, rax
	mov	rcx, qword ptr [r13 + 2*r10 + 144]
	xor	rcx, r11
	mov	rax, qword ptr [r13 + 2*r10 + 152]
	xor	rax, r8
	mul	rcx
	mov	rcx, rax
	mov	rbx, rdx
	mov	rdx, qword ptr [r13 + 2*r10 + 160]
	xor	rdx, r11
	mov	rax, qword ptr [r13 + 2*r10 + 168]
	xor	rax, r8
	mul	rdx
	mov	r15, rdx
	xor	rbx, rcx
	xor	r15, rax
	mov	rcx, qword ptr [r13 + 2*r10 + 176]
	xor	rcx, r11
	mov	rax, qword ptr [r13 + 2*r10 + 184]
	xor	rax, r8
	mul	rcx
	mov	rsi, rax
	mov	r12, rdx
	mov	rcx, qword ptr [r13 + 2*r10 + 192]
	xor	rcx, r11
	mov	rax, qword ptr [r13 + 2*r10 + 200]
	xor	rax, r8
	mul	rcx
	mov	rcx, rdx
	xor	r12, rsi
	xor	rcx, rax
	mov	rdx, qword ptr [r13 + 2*r10 + 208]
	xor	rdx, r11
	mov	rax, qword ptr [r13 + 2*r10 + 216]
	xor	rax, r8
	mul	rdx
	mov	r14, rax
	mov	rsi, rdx
	mov	rdx, qword ptr [r13 + 2*r10 + 224]
	xor	rdx, r11
	mov	rax, qword ptr [r13 + 2*r10 + 232]
	xor	rax, r8
	mul	rdx
	mov	rdi, rdx
	xor	rsi, r14
	xor	rdi, rax
	mov	rdx, qword ptr [r13 + 2*r10 + 240]
	xor	rdx, r11
	mov	rax, qword ptr [r13 + 2*r10 + 248]
	xor	rax, r8
	mul	rdx
	xor	rdx, rax
	mov	rax, qword ptr [rsp + 40]
	mov	qword ptr [r9 + r10], rax
	mov	rax, qword ptr [rsp + 48]
	mov	qword ptr [r9 + r10 + 8], rax
	mov	rax, qword ptr [rsp + 72]
	mov	qword ptr [r9 + r10 + 16], rax
	mov	rax, qword ptr [rsp + 104]
	mov	qword ptr [r9 + r10 + 24], rax
	mov	rax, qword ptr [rsp + 96]
	mov	qword ptr [r9 + r10 + 32], rax
	mov	rax, qword ptr [rsp + 88]
	mov	qword ptr [r9 + r10 + 40], rax
	mov	rax, qword ptr [rsp + 80]
	mov	qword ptr [r9 + r10 + 48], rax
	mov	rax, qword ptr [rsp + 56]
	mov	qword ptr [r9 + r10 + 56], rax
	mov	qword ptr [r9 + r10 + 64], rbp
	mov	qword ptr [r9 + r10 + 72], rbx
	mov	qword ptr [r9 + r10 + 80], r15
	mov	qword ptr [r9 + r10 + 88], r12
	mov	qword ptr [r9 + r10 + 96], rcx
	mov	qword ptr [r9 + r10 + 104], rsi
	mov	qword ptr [r9 + r10 + 112], rdi
	mov	qword ptr [r9 + r10 + 120], rdx
	sub	r10, -128
	cmp	qword ptr [rsp + 64], r10
	jne	.LBB75_563
.LBB75_564:
	mov	r9, qword ptr [rsp + 112]
	test	r9, r9
	je	.LBB75_721
	movabs	rax, 576460752303423472
	mov	rdx, qword ptr [rsp + 24]
	and	rax, rdx
	movabs	rcx, 1152921504606846960
	shl	rax, 4
	and	rcx, rdx
	add	r13, rax
	mov	rax, qword ptr [rsp + 8]
	lea	rcx, [rax + 8*rcx]
	mov	rax, qword ptr [rsp + 16]
	mov	rsi, qword ptr [rax]
	shl	r9d, 3
	xor	edi, edi
	.p2align	4
.LBB75_566:
	mov	rdx, qword ptr [r13 + 2*rdi]
	xor	rdx, rsi
	mov	rax, qword ptr [r13 + 2*rdi + 8]
	xor	rax, r8
	mul	rdx
	xor	rdx, rax
	mov	qword ptr [rcx + rdi], rdx
	add	rdi, 8
	cmp	r9, rdi
	jne	.LBB75_566
	jmp	.LBB75_721
.LBB75_312:
	lea	rdi, [rsp + 144]
	call	qword ptr [rip + _RNvMNtNtCs7SjCkj1olcZ_12arrow_buffer6buffer4nullNtB2_10NullBuffer13valid_indices@GOTPCREL]
	mov	r9, qword ptr [rsp + 144]
	mov	rcx, qword ptr [rsp + 152]
	mov	r12, qword ptr [rsp + 160]
	mov	rsi, qword ptr [rsp + 168]
	mov	rdi, qword ptr [rsp + 176]
	mov	r8, qword ptr [rsp + 184]
	mov	r13, qword ptr [rsp + 192]
	mov	r10, qword ptr [rsp + 200]
	mov	r11, qword ptr [rbx + 32]
	mov	rax, qword ptr [rsp + 16]
	mov	rbx, qword ptr [rax]
	movabs	r15, -4563226453097033507
	jmp	.LBB75_313
	.p2align	4
.LBB75_316:
	mov	r13, rsi
	xor	r12d, r12d
	add	r10, 64
.LBB75_313:
	test	r13, r13
	jne	.LBB75_323
	cmp	r12, -2
	je	.LBB75_320
	cmp	r12, 1
	je	.LBB75_316
	test	rdi, rdi
	je	.LBB75_320
	cmp	rdi, r8
	je	.LBB75_320
	mov	r13, qword ptr [rdi]
	add	rdi, 8
	mov	r12, -1
	add	r10, 64
	jmp	.LBB75_313
	.p2align	4
.LBB75_320:
	dec	r9
	cmp	r9, -3
	ja	.LBB75_721
	mov	r12, -2
	mov	r13, rcx
	xor	r9d, r9d
	add	r10, 64
	jmp	.LBB75_313
.LBB75_323:
	rep		bsf	rbp, r13
	add	rbp, r10
	mov	rax, qword ptr [r11 + 8*rbp]
	xor	rax, rbx
	mul	r15
	xor	rdx, rax
	lea	rax, [r13 - 1]
	and	r13, rax
	mov	rax, qword ptr [rsp + 8]
	mov	qword ptr [rax + 8*rbp], rdx
	jmp	.LBB75_313
.LBB75_526:
	lea	rdi, [rsp + 144]
	call	qword ptr [rip + _RNvMNtNtCs7SjCkj1olcZ_12arrow_buffer6buffer4nullNtB2_10NullBuffer13valid_indices@GOTPCREL]
	mov	r9, qword ptr [rsp + 144]
	mov	rcx, qword ptr [rsp + 152]
	mov	r12, qword ptr [rsp + 160]
	mov	rsi, qword ptr [rsp + 168]
	mov	rdi, qword ptr [rsp + 176]
	mov	r8, qword ptr [rsp + 184]
	mov	r13, qword ptr [rsp + 192]
	mov	r10, qword ptr [rsp + 200]
	mov	r11, qword ptr [rbx + 32]
	mov	rax, qword ptr [rsp + 16]
	mov	rbx, qword ptr [rax]
	movabs	r15, -4563226453097033507
	jmp	.LBB75_527
	.p2align	4
.LBB75_530:
	mov	r13, rsi
	xor	r12d, r12d
	add	r10, 64
.LBB75_527:
	test	r13, r13
	jne	.LBB75_537
	cmp	r12, -2
	je	.LBB75_534
	cmp	r12, 1
	je	.LBB75_530
	test	rdi, rdi
	je	.LBB75_534
	cmp	rdi, r8
	je	.LBB75_534
	mov	r13, qword ptr [rdi]
	add	rdi, 8
	mov	r12, -1
	add	r10, 64
	jmp	.LBB75_527
	.p2align	4
.LBB75_534:
	dec	r9
	cmp	r9, -3
	ja	.LBB75_721
	mov	r12, -2
	mov	r13, rcx
	xor	r9d, r9d
	add	r10, 64
	jmp	.LBB75_527
.LBB75_537:
	rep		bsf	rbp, r13
	add	rbp, r10
	mov	rax, qword ptr [r11 + 8*rbp]
	xor	rax, rbx
	mul	r15
	xor	rdx, rax
	lea	rax, [r13 - 1]
	and	r13, rax
	mov	rax, qword ptr [rsp + 8]
	mov	qword ptr [rax + 8*rbp], rdx
	jmp	.LBB75_527
.LBB75_568:
	lea	rdi, [rsp + 144]
	call	qword ptr [rip + _RNvMNtNtCs7SjCkj1olcZ_12arrow_buffer6buffer4nullNtB2_10NullBuffer13valid_indices@GOTPCREL]
	mov	r9, qword ptr [rsp + 144]
	mov	rcx, qword ptr [rsp + 152]
	mov	r12, qword ptr [rsp + 160]
	mov	rsi, qword ptr [rsp + 168]
	mov	rdi, qword ptr [rsp + 176]
	mov	r8, qword ptr [rsp + 184]
	mov	r13, qword ptr [rsp + 192]
	mov	r10, qword ptr [rsp + 200]
	mov	r11, qword ptr [rbx + 32]
	mov	rax, qword ptr [rsp + 16]
	mov	rbx, qword ptr [rax]
	jmp	.LBB75_569
	.p2align	4
.LBB75_572:
	mov	r13, rsi
	xor	r12d, r12d
	add	r10, 64
.LBB75_569:
	test	r13, r13
	jne	.LBB75_579
	cmp	r12, -2
	je	.LBB75_576
	cmp	r12, 1
	je	.LBB75_572
	test	rdi, rdi
	je	.LBB75_576
	cmp	rdi, r8
	je	.LBB75_576
	mov	r13, qword ptr [rdi]
	add	rdi, 8
	mov	r12, -1
	add	r10, 64
	jmp	.LBB75_569
	.p2align	4
.LBB75_576:
	dec	r9
	cmp	r9, -3
	ja	.LBB75_721
	mov	r12, -2
	mov	r13, rcx
	xor	r9d, r9d
	add	r10, 64
	jmp	.LBB75_569
.LBB75_579:
	rep		bsf	rbp, r13
	add	rbp, r10
	mov	rdx, rbp
	shl	rdx, 4
	mov	rax, qword ptr [r11 + rdx + 8]
	xor	rax, r15
	mov	rdx, qword ptr [r11 + rdx]
	xor	rdx, rbx
	mul	rdx
	xor	rdx, rax
	lea	rax, [r13 - 1]
	and	r13, rax
	mov	rax, qword ptr [rsp + 8]
	mov	qword ptr [rax + 8*rbp], rdx
	jmp	.LBB75_569
.LBB75_389:
	mov	r15, qword ptr [rsp + 24]
	mov	ecx, r15d
	and	ecx, 15
	mov	qword ptr [rsp + 112], rcx
	shr	rax, 6
	shr	r15, 4
	cmp	rax, r15
	cmovb	r15, rax
	mov	qword ptr [rsp + 64], r15
	test	r15, r15
	je	.LBB75_392
	mov	rax, qword ptr [rsp + 16]
	mov	r9, qword ptr [rax]
	shl	qword ptr [rsp + 64], 6
	xor	r10d, r10d
	mov	ebx, 2147483648
	.p2align	4
.LBB75_391:
	mov	eax, dword ptr [r13 + r10]
	mov	ecx, dword ptr [r13 + r10 + 4]
	cmp	rax, rbx
	mov	r11d, 0
	cmove	rax, r11
	xor	rax, r9
	movabs	r8, -4563226453097033507
	mul	r8
	xor	rdx, rax
	mov	qword ptr [rsp + 40], rdx
	cmp	rcx, rbx
	cmove	rcx, r11
	xor	rcx, r9
	mov	rax, rcx
	mul	r8
	mov	ecx, dword ptr [r13 + r10 + 8]
	xor	rdx, rax
	mov	qword ptr [rsp + 48], rdx
	cmp	rcx, rbx
	mov	esi, dword ptr [r13 + r10 + 12]
	cmove	rcx, r11
	xor	rcx, r9
	mov	rax, rcx
	mul	r8
	xor	rdx, rax
	mov	qword ptr [rsp + 72], rdx
	cmp	rsi, rbx
	mov	ecx, dword ptr [r13 + r10 + 16]
	cmove	rsi, r11
	xor	rsi, r9
	mov	rax, rsi
	mul	r8
	xor	rdx, rax
	mov	qword ptr [rsp + 104], rdx
	cmp	rcx, rbx
	mov	esi, dword ptr [r13 + r10 + 20]
	cmove	rcx, r11
	xor	rcx, r9
	mov	rax, rcx
	mul	r8
	xor	rdx, rax
	mov	qword ptr [rsp + 96], rdx
	cmp	rsi, rbx
	cmove	rsi, r11
	xor	rsi, r9
	mov	rax, rsi
	mul	r8
	mov	ecx, dword ptr [r13 + r10 + 24]
	xor	rdx, rax
	mov	qword ptr [rsp + 88], rdx
	cmp	rcx, rbx
	mov	esi, dword ptr [r13 + r10 + 28]
	cmove	rcx, r11
	xor	rcx, r9
	mov	rax, rcx
	mul	r8
	xor	rdx, rax
	mov	qword ptr [rsp + 80], rdx
	cmp	rsi, rbx
	mov	ecx, dword ptr [r13 + r10 + 32]
	cmove	rsi, r11
	xor	rsi, r9
	mov	rax, rsi
	mul	r8
	xor	rdx, rax
	mov	qword ptr [rsp + 56], rdx
	cmp	rcx, rbx
	mov	esi, dword ptr [r13 + r10 + 36]
	cmove	rcx, r11
	xor	rcx, r9
	mov	rax, rcx
	mul	r8
	mov	r14, rdx
	xor	r14, rax
	cmp	rsi, rbx
	cmove	rsi, r11
	xor	rsi, r9
	mov	rax, rsi
	mul	r8
	mov	rbp, rdx
	mov	ecx, dword ptr [r13 + r10 + 40]
	xor	rbp, rax
	cmp	rcx, rbx
	mov	esi, dword ptr [r13 + r10 + 44]
	cmove	rcx, r11
	xor	rcx, r9
	mov	rax, rcx
	mul	r8
	mov	r15, rdx
	xor	r15, rax
	cmp	rsi, rbx
	mov	ecx, dword ptr [r13 + r10 + 48]
	cmove	rsi, r11
	xor	rsi, r9
	mov	rax, rsi
	mul	r8
	mov	r12, rdx
	xor	r12, rax
	cmp	rcx, rbx
	mov	esi, dword ptr [r13 + r10 + 52]
	cmove	rcx, r11
	xor	rcx, r9
	mov	rax, rcx
	mul	r8
	mov	rcx, rdx
	xor	rcx, rax
	cmp	rsi, rbx
	cmove	rsi, r11
	xor	rsi, r9
	mov	rax, rsi
	mul	r8
	mov	rsi, rdx
	xor	rsi, rax
	mov	eax, dword ptr [r13 + r10 + 56]
	cmp	rax, rbx
	cmove	rax, r11
	xor	rax, r9
	mul	r8
	mov	rdi, rdx
	xor	rdi, rax
	mov	eax, dword ptr [r13 + r10 + 60]
	cmp	rax, rbx
	cmove	rax, r11
	xor	rax, r9
	mul	r8
	xor	rdx, rax
	mov	rax, qword ptr [rsp + 8]
	mov	r8, qword ptr [rsp + 40]
	mov	qword ptr [rax + 2*r10], r8
	mov	r8, qword ptr [rsp + 48]
	mov	qword ptr [rax + 2*r10 + 8], r8
	mov	r8, qword ptr [rsp + 72]
	mov	qword ptr [rax + 2*r10 + 16], r8
	mov	r8, qword ptr [rsp + 104]
	mov	qword ptr [rax + 2*r10 + 24], r8
	mov	r8, qword ptr [rsp + 96]
	mov	qword ptr [rax + 2*r10 + 32], r8
	mov	r8, qword ptr [rsp + 88]
	mov	qword ptr [rax + 2*r10 + 40], r8
	mov	r8, qword ptr [rsp + 80]
	mov	qword ptr [rax + 2*r10 + 48], r8
	mov	r8, qword ptr [rsp + 56]
	mov	qword ptr [rax + 2*r10 + 56], r8
	mov	qword ptr [rax + 2*r10 + 64], r14
	mov	qword ptr [rax + 2*r10 + 72], rbp
	mov	qword ptr [rax + 2*r10 + 80], r15
	mov	qword ptr [rax + 2*r10 + 88], r12
	mov	qword ptr [rax + 2*r10 + 96], rcx
	mov	qword ptr [rax + 2*r10 + 104], rsi
	mov	qword ptr [rax + 2*r10 + 112], rdi
	mov	qword ptr [rax + 2*r10 + 120], rdx
	add	r10, 64
	cmp	qword ptr [rsp + 64], r10
	jne	.LBB75_391
.LBB75_392:
	mov	r11, qword ptr [rsp + 112]
	test	r11, r11
	je	.LBB75_721
	mov	rax, qword ptr [rsp + 24]
	movabs	rcx, 1152921504606846960
	and	rax, rcx
	lea	rcx, [4*rax]
	add	rcx, r13
	mov	rdx, qword ptr [rsp + 8]
	lea	rsi, [rdx + 8*rax]
	mov	rax, qword ptr [rsp + 16]
	mov	rdi, qword ptr [rax]
	xor	r9d, r9d
	mov	r8d, 2147483648
	movabs	r10, -4563226453097033507
	.p2align	4
.LBB75_394:
	mov	edx, dword ptr [rcx + 4*r9]
	cmp	rdx, r8
	mov	eax, 0
	cmovne	rax, rdx
	xor	rax, rdi
	mul	r10
	xor	rdx, rax
	mov	qword ptr [rsi + 8*r9], rdx
	inc	r9
	cmp	r11, r9
	jne	.LBB75_394
	jmp	.LBB75_721
.LBB75_347:
	mov	r15, qword ptr [rsp + 24]
	mov	ecx, r15d
	and	ecx, 15
	mov	qword ptr [rsp + 112], rcx
	shr	rax, 5
	shr	r15, 4
	cmp	rax, r15
	cmovb	r15, rax
	mov	qword ptr [rsp + 64], r15
	test	r15, r15
	je	.LBB75_350
	mov	rax, qword ptr [rsp + 16]
	mov	r9, qword ptr [rax]
	shl	qword ptr [rsp + 64], 5
	xor	r10d, r10d
	movabs	r11, -4563226453097033507
	xor	r13d, r13d
	.p2align	4
.LBB75_349:
	movzx	eax, word ptr [r8 + r13]
	cmp	eax, 32768
	cmove	eax, r10d
	xor	rax, r9
	mul	r11
	xor	rdx, rax
	mov	qword ptr [rsp + 40], rdx
	movzx	eax, word ptr [r8 + r13 + 2]
	cmp	eax, 32768
	cmove	eax, r10d
	xor	rax, r9
	mul	r11
	xor	rdx, rax
	mov	qword ptr [rsp + 48], rdx
	movzx	eax, word ptr [r8 + r13 + 4]
	cmp	eax, 32768
	cmove	eax, r10d
	xor	rax, r9
	mul	r11
	xor	rdx, rax
	mov	qword ptr [rsp + 72], rdx
	movzx	eax, word ptr [r8 + r13 + 6]
	cmp	eax, 32768
	cmove	eax, r10d
	xor	rax, r9
	mul	r11
	xor	rdx, rax
	mov	qword ptr [rsp + 104], rdx
	movzx	eax, word ptr [r8 + r13 + 8]
	cmp	eax, 32768
	cmove	eax, r10d
	xor	rax, r9
	mul	r11
	xor	rdx, rax
	mov	qword ptr [rsp + 96], rdx
	movzx	eax, word ptr [r8 + r13 + 10]
	cmp	eax, 32768
	cmove	eax, r10d
	xor	rax, r9
	mul	r11
	xor	rdx, rax
	mov	qword ptr [rsp + 88], rdx
	movzx	eax, word ptr [r8 + r13 + 12]
	cmp	eax, 32768
	cmove	eax, r10d
	xor	rax, r9
	mul	r11
	xor	rdx, rax
	mov	qword ptr [rsp + 80], rdx
	movzx	eax, word ptr [r8 + r13 + 14]
	cmp	eax, 32768
	cmove	eax, r10d
	xor	rax, r9
	mul	r11
	xor	rdx, rax
	mov	qword ptr [rsp + 56], rdx
	movzx	eax, word ptr [r8 + r13 + 16]
	cmp	eax, 32768
	cmove	eax, r10d
	xor	rax, r9
	mul	r11
	mov	rbp, rdx
	xor	rbp, rax
	movzx	eax, word ptr [r8 + r13 + 18]
	cmp	eax, 32768
	cmove	eax, r10d
	xor	rax, r9
	mul	r11
	mov	rbx, rdx
	xor	rbx, rax
	movzx	eax, word ptr [r8 + r13 + 20]
	cmp	eax, 32768
	cmove	eax, r10d
	xor	rax, r9
	mul	r11
	mov	r15, rdx
	xor	r15, rax
	movzx	eax, word ptr [r8 + r13 + 22]
	cmp	eax, 32768
	cmove	eax, r10d
	xor	rax, r9
	mul	r11
	mov	r12, rdx
	xor	r12, rax
	movzx	eax, word ptr [r8 + r13 + 24]
	cmp	eax, 32768
	cmove	eax, r10d
	xor	rax, r9
	mul	r11
	mov	rcx, rdx
	xor	rcx, rax
	movzx	eax, word ptr [r8 + r13 + 26]
	cmp	eax, 32768
	cmove	eax, r10d
	xor	rax, r9
	mul	r11
	mov	rsi, rdx
	xor	rsi, rax
	movzx	eax, word ptr [r8 + r13 + 28]
	cmp	eax, 32768
	cmove	eax, r10d
	xor	rax, r9
	mul	r11
	mov	rdi, rdx
	xor	rdi, rax
	movzx	eax, word ptr [r8 + r13 + 30]
	cmp	eax, 32768
	cmove	eax, r10d
	xor	rax, r9
	mul	r11
	xor	rdx, rax
	mov	rax, qword ptr [rsp + 8]
	mov	r14, qword ptr [rsp + 40]
	mov	qword ptr [rax + 4*r13], r14
	mov	r14, qword ptr [rsp + 48]
	mov	qword ptr [rax + 4*r13 + 8], r14
	mov	r14, qword ptr [rsp + 72]
	mov	qword ptr [rax + 4*r13 + 16], r14
	mov	r14, qword ptr [rsp + 104]
	mov	qword ptr [rax + 4*r13 + 24], r14
	mov	r14, qword ptr [rsp + 96]
	mov	qword ptr [rax + 4*r13 + 32], r14
	mov	r14, qword ptr [rsp + 88]
	mov	qword ptr [rax + 4*r13 + 40], r14
	mov	r14, qword ptr [rsp + 80]
	mov	qword ptr [rax + 4*r13 + 48], r14
	mov	r14, qword ptr [rsp + 56]
	mov	qword ptr [rax + 4*r13 + 56], r14
	mov	qword ptr [rax + 4*r13 + 64], rbp
	mov	qword ptr [rax + 4*r13 + 72], rbx
	mov	qword ptr [rax + 4*r13 + 80], r15
	mov	qword ptr [rax + 4*r13 + 88], r12
	mov	qword ptr [rax + 4*r13 + 96], rcx
	mov	qword ptr [rax + 4*r13 + 104], rsi
	mov	qword ptr [rax + 4*r13 + 112], rdi
	mov	qword ptr [rax + 4*r13 + 120], rdx
	add	r13, 32
	cmp	qword ptr [rsp + 64], r13
	jne	.LBB75_349
.LBB75_350:
	mov	r11, qword ptr [rsp + 112]
	test	r11, r11
	je	.LBB75_721
	mov	rax, qword ptr [rsp + 24]
	movabs	rcx, 1152921504606846960
	and	rax, rcx
	lea	rcx, [r8 + 2*rax]
	mov	rdx, qword ptr [rsp + 8]
	lea	rsi, [rdx + 8*rax]
	mov	rax, qword ptr [rsp + 16]
	mov	rdi, qword ptr [rax]
	xor	r8d, r8d
	movabs	r9, -4563226453097033507
	xor	r10d, r10d
	.p2align	4
.LBB75_352:
	movzx	eax, word ptr [rcx + 2*r10]
	cmp	eax, 32768
	cmove	eax, r8d
	xor	rax, rdi
	mul	r9
	xor	rdx, rax
	mov	qword ptr [rsi + 8*r10], rdx
	inc	r10
	cmp	r11, r10
	jne	.LBB75_352
	jmp	.LBB75_721
.LBB75_396:
	call	qword ptr [rip + _RNvMNtNtCs7SjCkj1olcZ_12arrow_buffer6buffer4nullNtB2_10NullBuffer13valid_indices@GOTPCREL]
	mov	r9, qword ptr [rsp + 144]
	mov	rcx, qword ptr [rsp + 152]
	mov	r13, qword ptr [rsp + 160]
	mov	rsi, qword ptr [rsp + 168]
	mov	rdi, qword ptr [rsp + 176]
	mov	r8, qword ptr [rsp + 184]
	mov	rbp, qword ptr [rsp + 192]
	mov	r10, qword ptr [rsp + 200]
	mov	r11, qword ptr [rbx + 32]
	mov	rax, qword ptr [rsp + 16]
	mov	rbx, qword ptr [rax]
	mov	r14d, 2147483648
	movabs	r12, -4563226453097033507
	jmp	.LBB75_397
	.p2align	4
.LBB75_400:
	mov	rbp, rsi
	xor	r13d, r13d
	add	r10, 64
.LBB75_397:
	test	rbp, rbp
	jne	.LBB75_407
	cmp	r13, -2
	je	.LBB75_404
	cmp	r13, 1
	je	.LBB75_400
	test	rdi, rdi
	je	.LBB75_404
	cmp	rdi, r8
	je	.LBB75_404
	mov	rbp, qword ptr [rdi]
	add	rdi, 8
	mov	r13, -1
	add	r10, 64
	jmp	.LBB75_397
	.p2align	4
.LBB75_404:
	dec	r9
	cmp	r9, -3
	ja	.LBB75_721
	mov	r13, -2
	mov	rbp, rcx
	xor	r9d, r9d
	add	r10, 64
	jmp	.LBB75_397
.LBB75_407:
	rep		bsf	r15, rbp
	add	r15, r10
	mov	edx, dword ptr [r11 + 4*r15]
	cmp	rdx, r14
	mov	eax, 0
	cmovne	rax, rdx
	xor	rax, rbx
	mul	r12
	xor	rdx, rax
	lea	rax, [rbp - 1]
	and	rbp, rax
	mov	rax, qword ptr [rsp + 8]
	mov	qword ptr [rax + 8*r15], rdx
	jmp	.LBB75_397
.LBB75_354:
	call	qword ptr [rip + _RNvMNtNtCs7SjCkj1olcZ_12arrow_buffer6buffer4nullNtB2_10NullBuffer13valid_indices@GOTPCREL]
	mov	r9, qword ptr [rsp + 144]
	mov	rcx, qword ptr [rsp + 152]
	mov	r13, qword ptr [rsp + 160]
	mov	rsi, qword ptr [rsp + 168]
	mov	rdi, qword ptr [rsp + 176]
	mov	r8, qword ptr [rsp + 184]
	mov	rbp, qword ptr [rsp + 192]
	mov	r10, qword ptr [rsp + 200]
	mov	r11, qword ptr [rbx + 32]
	mov	rax, qword ptr [rsp + 16]
	mov	rbx, qword ptr [rax]
	xor	r14d, r14d
	movabs	r12, -4563226453097033507
	jmp	.LBB75_355
	.p2align	4
.LBB75_358:
	mov	rbp, rsi
	xor	r13d, r13d
	add	r10, 64
.LBB75_355:
	test	rbp, rbp
	jne	.LBB75_365
	cmp	r13, -2
	je	.LBB75_362
	cmp	r13, 1
	je	.LBB75_358
	test	rdi, rdi
	je	.LBB75_362
	cmp	rdi, r8
	je	.LBB75_362
	mov	rbp, qword ptr [rdi]
	add	rdi, 8
	mov	r13, -1
	add	r10, 64
	jmp	.LBB75_355
	.p2align	4
.LBB75_362:
	dec	r9
	cmp	r9, -3
	ja	.LBB75_721
	mov	r13, -2
	mov	rbp, rcx
	xor	r9d, r9d
	add	r10, 64
	jmp	.LBB75_355
.LBB75_365:
	rep		bsf	r15, rbp
	add	r15, r10
	movzx	eax, word ptr [r11 + 2*r15]
	cmp	eax, 32768
	cmove	eax, r14d
	xor	rax, rbx
	mul	r12
	xor	rdx, rax
	lea	rax, [rbp - 1]
	and	rbp, rax
	mov	rax, qword ptr [rsp + 8]
	mov	qword ptr [rax + 8*r15], rdx
	jmp	.LBB75_355
.LBB75_604:
	mov	r13, qword ptr [rsp + 24]
	movabs	r15, -4563226453097033507
	mov	r14d, r13d
	and	r14d, 15
	shr	rax, 9
	shr	r13, 4
	cmp	rax, r13
	cmovb	r13, rax
	test	r13, r13
	je	.LBB75_609
	mov	rax, qword ptr [rsp + 16]
	mov	rbx, qword ptr [rax]
	xor	r12d, r12d
	.p2align	4
.LBB75_606:
	mov	rsi, r12
	shl	rsi, 9
	add	rsi, rbp
	mov	edx, 512
	lea	rdi, [rsp + 144]
	call	qword ptr [rip + memcpy@GOTPCREL]
	xor	ecx, ecx
	.p2align	4
.LBB75_607:
	mov	rdx, qword ptr [rsp + 4*rcx + 144]
	xor	rdx, rbx
	mov	rax, qword ptr [rsp + 4*rcx + 152]
	xor	rax, r15
	mul	rdx
	xor	rdx, qword ptr [rsp + 4*rcx + 160]
	xor	rax, rdx
	mov	rdx, qword ptr [rsp + 4*rcx + 168]
	xor	rdx, r15
	mul	rdx
	xor	rdx, rax
	mov	qword ptr [rsp + rcx + 656], rdx
	add	rcx, 8
	cmp	rcx, 128
	jne	.LBB75_607
	lea	rax, [r12 + 1]
	shl	r12, 7
	movups	xmm0, xmmword ptr [rsp + 768]
	movaps	xmmword ptr [rsp + 896], xmm0
	movups	xmm0, xmmword ptr [rsp + 752]
	movaps	xmmword ptr [rsp + 880], xmm0
	movups	xmm1, xmmword ptr [rsp + 736]
	movaps	xmmword ptr [rsp + 864], xmm1
	movups	xmm2, xmmword ptr [rsp + 720]
	movaps	xmmword ptr [rsp + 848], xmm2
	movups	xmm3, xmmword ptr [rsp + 656]
	movups	xmm4, xmmword ptr [rsp + 672]
	movups	xmm5, xmmword ptr [rsp + 688]
	movups	xmm6, xmmword ptr [rsp + 704]
	movaps	xmmword ptr [rsp + 832], xmm6
	movaps	xmmword ptr [rsp + 816], xmm5
	movaps	xmmword ptr [rsp + 800], xmm4
	movaps	xmmword ptr [rsp + 784], xmm3
	mov	rcx, qword ptr [rsp + 8]
	movups	xmmword ptr [rcx + r12 + 96], xmm0
	movups	xmmword ptr [rcx + r12 + 80], xmm1
	movups	xmmword ptr [rcx + r12 + 64], xmm2
	movups	xmmword ptr [rcx + r12 + 48], xmm6
	movups	xmmword ptr [rcx + r12 + 32], xmm5
	movups	xmmword ptr [rcx + r12 + 16], xmm4
	movups	xmmword ptr [rcx + r12], xmm3
	movdqa	xmm0, xmmword ptr [rsp + 896]
	movdqu	xmmword ptr [rcx + r12 + 112], xmm0
	mov	r12, rax
	cmp	rax, r13
	jne	.LBB75_606
.LBB75_609:
	test	r14, r14
	je	.LBB75_721
	movabs	rax, 1152921504606846960
	mov	rdx, qword ptr [rsp + 24]
	and	rax, rdx
	mov	rcx, qword ptr [rsp + 8]
	lea	rcx, [rcx + 8*rax]
	mov	rax, qword ptr [rsp + 16]
	mov	rsi, qword ptr [rax]
	shl	rdx, 5
	movabs	rax, 9223372036854775296
	and	rax, rdx
	lea	rdi, [rax + rbp]
	add	rdi, 16
	shl	r14d, 3
	xor	r8d, r8d
	.p2align	4
.LBB75_611:
	mov	rdx, qword ptr [rdi + 4*r8 - 16]
	xor	rdx, rsi
	mov	rax, qword ptr [rdi + 4*r8 - 8]
	xor	rax, r15
	mul	rdx
	xor	rdx, qword ptr [rdi + 4*r8]
	xor	rax, rdx
	mov	rdx, qword ptr [rdi + 4*r8 + 8]
	xor	rdx, r15
	mul	rdx
	xor	rdx, rax
	mov	qword ptr [rcx + r8], rdx
	add	r8, 8
	cmp	r14, r8
	jne	.LBB75_611
	jmp	.LBB75_721
.LBB75_613:
	movabs	r14, -4563226453097033507
	lea	rdi, [rsp + 144]
	call	qword ptr [rip + _RNvMNtNtCs7SjCkj1olcZ_12arrow_buffer6buffer4nullNtB2_10NullBuffer13valid_indices@GOTPCREL]
	mov	r9, qword ptr [rsp + 144]
	mov	rcx, qword ptr [rsp + 152]
	mov	r12, qword ptr [rsp + 160]
	mov	rsi, qword ptr [rsp + 168]
	mov	rdi, qword ptr [rsp + 176]
	mov	r8, qword ptr [rsp + 184]
	mov	r13, qword ptr [rsp + 192]
	mov	r10, qword ptr [rsp + 200]
	mov	r11, qword ptr [rbx + 32]
	mov	rax, qword ptr [rsp + 16]
	mov	rbx, qword ptr [rax]
	jmp	.LBB75_614
	.p2align	4
.LBB75_617:
	mov	r13, rsi
	xor	r12d, r12d
	add	r10, 64
.LBB75_614:
	test	r13, r13
	jne	.LBB75_624
	cmp	r12, -2
	je	.LBB75_621
	cmp	r12, 1
	je	.LBB75_617
	test	rdi, rdi
	je	.LBB75_621
	cmp	rdi, r8
	je	.LBB75_621
	mov	r13, qword ptr [rdi]
	add	rdi, 8
	mov	r12, -1
	add	r10, 64
	jmp	.LBB75_614
	.p2align	4
.LBB75_621:
	dec	r9
	cmp	r9, -3
	ja	.LBB75_721
	mov	r12, -2
	mov	r13, rcx
	xor	r9d, r9d
	add	r10, 64
	jmp	.LBB75_614
.LBB75_624:
	rep		bsf	r15, r13
	add	r15, r10
	mov	rbp, r15
	shl	rbp, 5
	mov	rax, qword ptr [r11 + rbp + 8]
	xor	rax, r14
	mov	rdx, qword ptr [r11 + rbp]
	xor	rdx, rbx
	mul	rdx
	xor	rdx, qword ptr [r11 + rbp + 16]
	xor	rax, rdx
	mov	rdx, qword ptr [r11 + rbp + 24]
	xor	rdx, r14
	mul	rdx
	xor	rdx, rax
	lea	rax, [r13 - 1]
	and	r13, rax
	mov	rax, qword ptr [rsp + 8]
	mov	qword ptr [rax + 8*r15], rdx
	jmp	.LBB75_614
.LBB75_690:
	lea	rdi, [rsp + 144]
	mov	rsi, r13
	mov	rdx, rbp
	call	qword ptr [rip + _RNvNtCspC5jgAXZxx_17datafusion_common4cast24as_large_list_view_array@GOTPCREL]
	mov	rax, qword ptr [rsp + 144]
	cmp	rax, -1
	jne	.LBB75_660
	mov	rsi, qword ptr [rsp + 152]
	mov	rax, qword ptr [rsp + 16]
	mov	qword ptr [rsp + 656], rax
	lea	rdi, [rsp + 144]
	lea	rdx, [rsp + 656]
	mov	rcx, qword ptr [rsp + 8]
	mov	r8, qword ptr [rsp + 24]
	call	_RINvNtCspC5jgAXZxx_17datafusion_common10hash_utils20hash_list_view_arrayxINtB2_21HashStateChildHashingNtNtCsRVR2D38dYC_8foldhash4fast10FixedStateEECsg1h6yN5JQqd_7hashasm
	jmp	.LBB75_682
.LBB75_698:
	mov	rdi, r13
	call	rbx
	cmp	byte ptr [rax], 34
	jne	.LBB75_699
	mov	rcx, qword ptr [rax + 8]
	movzx	eax, byte ptr [rcx]
	add	eax, -2
	cmp	eax, 7
	ja	.LBB75_700
	lea	rcx, [rip + .LJTI75_3]
	movsxd	rax, dword ptr [rcx + 4*rax]
	add	rax, rcx
	jmp	rax
.LBB75_701:
	mov	rsi, qword ptr [rbp + 32]
	mov	rdi, r13
	call	_RINvNtCs33DBFqlTBxO_11arrow_array4cast19as_dictionary_arrayNtNtB4_5types8Int8TypeECsg1h6yN5JQqd_7hashasm
	movzx	r9d, r12b
	lea	rdi, [rsp + 144]
	mov	rsi, rax
	mov	rdx, qword ptr [rsp + 16]
	mov	rcx, qword ptr [rsp + 8]
	mov	r8, qword ptr [rsp + 24]
	call	_RINvNtCspC5jgAXZxx_17datafusion_common10hash_utils15hash_dictionaryNtNtCs33DBFqlTBxO_11arrow_array5types8Int8TypeNtNtCsRVR2D38dYC_8foldhash4fast10FixedStateECsg1h6yN5JQqd_7hashasm
	jmp	.LBB75_682
.LBB75_678:
	lea	rdi, [rsp + 144]
	mov	rsi, r13
	mov	rdx, rbp
	call	qword ptr [rip + _RNvNtCspC5jgAXZxx_17datafusion_common4cast20as_string_view_array@GOTPCREL]
	mov	rax, qword ptr [rsp + 144]
	cmp	rax, -1
	jne	.LBB75_660
	mov	rdi, qword ptr [rsp + 152]
	movzx	r8d, r12b
	mov	rsi, qword ptr [rsp + 16]
	mov	rdx, qword ptr [rsp + 8]
	mov	rcx, qword ptr [rsp + 24]
	call	_RINvNtCspC5jgAXZxx_17datafusion_common10hash_utils28hash_generic_byte_view_arrayNtNtCs33DBFqlTBxO_11arrow_array5types14StringViewTypeNtNtCsRVR2D38dYC_8foldhash4fast10FixedStateECsg1h6yN5JQqd_7hashasm
	jmp	.LBB75_721
.LBB75_715:
	mov	rdi, r13
	call	rbx
	cmp	byte ptr [rax], 40
	jne	.LBB75_716
	mov	rax, qword ptr [rax + 8]
	movzx	ecx, byte ptr [rax + 40]
	cmp	ecx, 3
	je	.LBB75_717
	cmp	ecx, 4
	je	.LBB75_718
	cmp	ecx, 5
	jne	.LBB75_714
	mov	rsi, qword ptr [rbp + 32]
	mov	rdi, r13
	call	_RINvNtCs33DBFqlTBxO_11arrow_array4cast12as_run_arrayNtNtB4_5types9Int64TypeECsg1h6yN5JQqd_7hashasm
	mov	rcx, qword ptr [rsp + 16]
	mov	qword ptr [rsp + 656], rcx
	movzx	r9d, r12b
	lea	rdi, [rsp + 144]
	lea	rdx, [rsp + 656]
	mov	rsi, rax
	mov	rcx, qword ptr [rsp + 8]
	mov	r8, qword ptr [rsp + 24]
	call	_RINvNtCspC5jgAXZxx_17datafusion_common10hash_utils14hash_run_arrayNtNtCs33DBFqlTBxO_11arrow_array5types9Int64TypeINtB2_21HashStateChildHashingNtNtCsRVR2D38dYC_8foldhash4fast10FixedStateEECsg1h6yN5JQqd_7hashasm
	jmp	.LBB75_682
.LBB75_674:
	lea	rdi, [rsp + 144]
	mov	rsi, r13
	mov	rdx, rbp
	call	qword ptr [rip + _RNvNtCspC5jgAXZxx_17datafusion_common4cast20as_binary_view_array@GOTPCREL]
	mov	rax, qword ptr [rsp + 144]
	cmp	rax, -1
	jne	.LBB75_660
	mov	rdi, qword ptr [rsp + 152]
	movzx	r8d, r12b
	mov	rsi, qword ptr [rsp + 16]
	mov	rdx, qword ptr [rsp + 8]
	mov	rcx, qword ptr [rsp + 24]
	call	_RINvNtCspC5jgAXZxx_17datafusion_common10hash_utils28hash_generic_byte_view_arrayNtNtCs33DBFqlTBxO_11arrow_array5types14BinaryViewTypeNtNtCsRVR2D38dYC_8foldhash4fast10FixedStateECsg1h6yN5JQqd_7hashasm
	jmp	.LBB75_721
.LBB75_709:
	lea	rdi, [rsp + 144]
	mov	rsi, r13
	mov	rdx, rbp
	call	qword ptr [rip + _RNvNtCspC5jgAXZxx_17datafusion_common4cast12as_map_array@GOTPCREL]
	mov	rax, qword ptr [rsp + 144]
	cmp	rax, -1
	jne	.LBB75_660
	mov	rsi, qword ptr [rsp + 152]
	mov	rax, qword ptr [rsp + 16]
	mov	qword ptr [rsp + 656], rax
	lea	rdi, [rsp + 144]
	lea	rdx, [rsp + 656]
	mov	rcx, qword ptr [rsp + 8]
	mov	r8, qword ptr [rsp + 24]
	call	_RINvNtCspC5jgAXZxx_17datafusion_common10hash_utils14hash_map_arrayINtB2_21HashStateChildHashingNtNtCsRVR2D38dYC_8foldhash4fast10FixedStateEECsg1h6yN5JQqd_7hashasm
	jmp	.LBB75_682
.LBB75_658:
	mov	rdi, r13
	mov	rsi, rbp
	call	qword ptr [rip + _RNvNtCs33DBFqlTBxO_11arrow_array4cast20as_largestring_array@GOTPCREL]
	mov	rcx, qword ptr [rsp + 16]
	mov	rsi, qword ptr [rcx]
	movzx	r8d, r12b
	mov	rdi, rax
	mov	rdx, qword ptr [rsp + 8]
	mov	rcx, qword ptr [rsp + 24]
	call	_RINvNtCspC5jgAXZxx_17datafusion_common10hash_utils10hash_arrayRINtNtNtCs33DBFqlTBxO_11arrow_array5array10byte_array16GenericByteArrayINtNtB16_5types17GenericStringTypexEENtNtCsRVR2D38dYC_8foldhash4fast10FixedStateECsg1h6yN5JQqd_7hashasm
	jmp	.LBB75_721
.LBB75_692:
	lea	rdi, [rsp + 144]
	mov	rsi, r13
	mov	rdx, rbp
	call	qword ptr [rip + _RNvNtCspC5jgAXZxx_17datafusion_common4cast15as_struct_array@GOTPCREL]
	mov	rax, qword ptr [rsp + 144]
	cmp	rax, -1
	jne	.LBB75_660
	mov	rsi, qword ptr [rsp + 152]
	mov	rax, qword ptr [rsp + 16]
	mov	qword ptr [rsp + 656], rax
	lea	rdi, [rsp + 144]
	lea	rdx, [rsp + 656]
	mov	rcx, qword ptr [rsp + 8]
	mov	r8, qword ptr [rsp + 24]
	call	_RINvNtCspC5jgAXZxx_17datafusion_common10hash_utils17hash_struct_arrayINtB2_21HashStateChildHashingNtNtCsRVR2D38dYC_8foldhash4fast10FixedStateEECsg1h6yN5JQqd_7hashasm
	jmp	.LBB75_682
.LBB75_649:
	movzx	eax, byte ptr [rax + 1]
	mov	rsi, qword ptr [rbp + 32]
	lea	rcx, [rip + .LJTI75_4]
	movsxd	rax, dword ptr [rcx + 4*rax]
	add	rax, rcx
	jmp	rax
.LBB75_662:
	mov	rdi, r13
	call	_RINvNtCs33DBFqlTBxO_11arrow_array4cast18as_primitive_arrayNtNtB4_5types19TimestampSecondTypeECsg1h6yN5JQqd_7hashasm
	movzx	r8d, r12b
	mov	rdi, rax
	mov	rsi, qword ptr [rsp + 16]
	mov	rdx, qword ptr [rsp + 8]
	mov	rcx, qword ptr [rsp + 24]
	call	_RINvNtCspC5jgAXZxx_17datafusion_common10hash_utils20hash_array_primitiveNtNtCs33DBFqlTBxO_11arrow_array5types19TimestampSecondTypeNtNtCsRVR2D38dYC_8foldhash4fast10FixedStateECsg1h6yN5JQqd_7hashasm
	jmp	.LBB75_721
.LBB75_652:
	movzx	eax, byte ptr [rax + 1]
	test	eax, eax
	je	.LBB75_666
	cmp	eax, 1
	jne	.LBB75_728
	mov	rsi, qword ptr [rbp + 32]
	mov	rdi, r13
	call	_RINvNtCs33DBFqlTBxO_11arrow_array4cast18as_primitive_arrayNtNtB4_5types21Time32MillisecondTypeECsg1h6yN5JQqd_7hashasm
	movzx	r8d, r12b
	mov	rdi, rax
	mov	rsi, qword ptr [rsp + 16]
	mov	rdx, qword ptr [rsp + 8]
	mov	rcx, qword ptr [rsp + 24]
	call	_RINvNtCspC5jgAXZxx_17datafusion_common10hash_utils20hash_array_primitiveNtNtCs33DBFqlTBxO_11arrow_array5types21Time32MillisecondTypeNtNtCsRVR2D38dYC_8foldhash4fast10FixedStateECsg1h6yN5JQqd_7hashasm
	jmp	.LBB75_721
.LBB75_676:
	lea	rdi, [rsp + 144]
	mov	rsi, r13
	mov	rdx, rbp
	call	qword ptr [rip + _RNvNtCspC5jgAXZxx_17datafusion_common4cast15as_string_array@GOTPCREL]
	mov	rax, qword ptr [rsp + 144]
	cmp	rax, -1
	jne	.LBB75_660
	mov	rdi, qword ptr [rsp + 152]
	mov	rax, qword ptr [rsp + 16]
	mov	rsi, qword ptr [rax]
	movzx	r8d, r12b
	mov	rdx, qword ptr [rsp + 8]
	mov	rcx, qword ptr [rsp + 24]
	call	_RINvNtCspC5jgAXZxx_17datafusion_common10hash_utils10hash_arrayRINtNtNtCs33DBFqlTBxO_11arrow_array5array10byte_array16GenericByteArrayINtNtB16_5types17GenericStringTypelEENtNtCsRVR2D38dYC_8foldhash4fast10FixedStateECsg1h6yN5JQqd_7hashasm
	jmp	.LBB75_721
.LBB75_659:
	lea	rdi, [rsp + 144]
	mov	rsi, r13
	mov	rdx, rbp
	call	qword ptr [rip + _RNvNtCspC5jgAXZxx_17datafusion_common4cast16as_boolean_array@GOTPCREL]
	mov	rax, qword ptr [rsp + 144]
	cmp	rax, -1
	jne	.LBB75_660
	mov	rdi, qword ptr [rsp + 152]
	mov	rax, qword ptr [rsp + 16]
	mov	rsi, qword ptr [rax]
	movzx	r8d, r12b
	mov	rdx, qword ptr [rsp + 8]
	mov	rcx, qword ptr [rsp + 24]
	call	_RINvNtCspC5jgAXZxx_17datafusion_common10hash_utils10hash_arrayRNtNtNtCs33DBFqlTBxO_11arrow_array5array13boolean_array12BooleanArrayNtNtCsRVR2D38dYC_8foldhash4fast10FixedStateECsg1h6yN5JQqd_7hashasm
	jmp	.LBB75_721
.LBB75_655:
	movzx	eax, byte ptr [rax + 1]
	cmp	eax, 2
	je	.LBB75_667
	cmp	eax, 3
	jne	.LBB75_728
	mov	rsi, qword ptr [rbp + 32]
	mov	rdi, r13
	call	_RINvNtCs33DBFqlTBxO_11arrow_array4cast18as_primitive_arrayNtNtB4_5types20Time64NanosecondTypeECsg1h6yN5JQqd_7hashasm
	movzx	r8d, r12b
	mov	rdi, rax
	mov	rsi, qword ptr [rsp + 16]
	mov	rdx, qword ptr [rsp + 8]
	mov	rcx, qword ptr [rsp + 24]
	call	_RINvNtCspC5jgAXZxx_17datafusion_common10hash_utils20hash_array_primitiveNtNtCs33DBFqlTBxO_11arrow_array5types20Time64NanosecondTypeNtNtCsRVR2D38dYC_8foldhash4fast10FixedStateECsg1h6yN5JQqd_7hashasm
	jmp	.LBB75_721
.LBB75_728:
	mov	rdi, r13
	call	rbx
	mov	qword ptr [rsp + 656], rax
	lea	r14, [rsp + 656]
	mov	qword ptr [rsp + 144], r14
	lea	rax, [rip + _RNvXs1i_NtCscI6d9CVNmLh_4core3fmtRNtNtCsk9P1WA8sPmy_12arrow_schema8datatype8DataTypeNtB6_7Display3fmtCsg1h6yN5JQqd_7hashasm]
	mov	qword ptr [rsp + 152], rax
	lea	rsi, [rip + .Lanon.98d58ce4ceb595e8c03795bebe6d48b9.37]
	mov	r15, qword ptr [rip + _RNvNvNtCs40k4W9msRzi_5alloc3fmt6format12format_inner@GOTPCREL]
	lea	rbx, [rsp + 784]
	lea	rdx, [rsp + 144]
	mov	rdi, rbx
	call	r15
	mov	qword ptr [rsp + 656], 0
	mov	qword ptr [rsp + 664], 1
	mov	qword ptr [rsp + 672], 0
	mov	qword ptr [rsp + 144], rbx
	lea	rax, [rip + _RNvXsq_NtCs40k4W9msRzi_5alloc6stringNtB5_6StringNtNtCscI6d9CVNmLh_4core3fmt7Display3fmt]
	mov	qword ptr [rsp + 152], rax
	mov	qword ptr [rsp + 160], r14
	mov	qword ptr [rsp + 168], rax
.Ltmp473:
	lea	rsi, [rip + .Lanon.98d58ce4ceb595e8c03795bebe6d48b9.38]
	lea	rdi, [rsp + 928]
	lea	rdx, [rsp + 144]
	call	r15
.Ltmp474:
	mov	rsi, qword ptr [rsp + 656]
	test	rsi, rsi
	je	.LBB75_731
	mov	rdi, qword ptr [rsp + 664]
	mov	edx, 1
	call	qword ptr [rip + _RNvCs9hJ03s5DiqP_7___rustc14___rust_dealloc@GOTPCREL]
.LBB75_731:
	mov	rsi, qword ptr [rsp + 784]
	test	rsi, rsi
	je	.LBB75_733
	mov	rdi, qword ptr [rsp + 792]
	mov	edx, 1
	call	qword ptr [rip + _RNvCs9hJ03s5DiqP_7___rustc14___rust_dealloc@GOTPCREL]
.LBB75_733:
	mov	rax, qword ptr [rsp + 944]
	mov	rcx, qword ptr [rsp + 32]
	mov	qword ptr [rcx + 24], rax
	movdqu	xmm0, xmmword ptr [rsp + 928]
	movdqu	xmmword ptr [rcx + 8], xmm0
	mov	qword ptr [rcx], 3
	jmp	.LBB75_722
.LBB75_668:
	mov	rdx, qword ptr [rbp + 32]
	lea	rdi, [rsp + 144]
	mov	rsi, r13
	mov	rcx, rbx
	call	_RINvNtCspC5jgAXZxx_17datafusion_common4cast23as_generic_binary_arraylECsg1h6yN5JQqd_7hashasm
	mov	rax, qword ptr [rsp + 144]
	cmp	rax, -1
	jne	.LBB75_660
	mov	rdi, qword ptr [rsp + 152]
	mov	rax, qword ptr [rsp + 16]
	mov	rsi, qword ptr [rax]
	movzx	r8d, r12b
	mov	rdx, qword ptr [rsp + 8]
	mov	rcx, qword ptr [rsp + 24]
	call	_RINvNtCspC5jgAXZxx_17datafusion_common10hash_utils10hash_arrayRINtNtNtCs33DBFqlTBxO_11arrow_array5array10byte_array16GenericByteArrayINtNtB16_5types17GenericBinaryTypelEENtNtCsRVR2D38dYC_8foldhash4fast10FixedStateECsg1h6yN5JQqd_7hashasm
	jmp	.LBB75_721
.LBB75_680:
	lea	rdi, [rsp + 144]
	mov	rsi, r13
	mov	rdx, rbp
	call	qword ptr [rip + _RNvNtCspC5jgAXZxx_17datafusion_common4cast13as_list_array@GOTPCREL]
	mov	rax, qword ptr [rsp + 144]
	cmp	rax, -1
	jne	.LBB75_660
	mov	rsi, qword ptr [rsp + 152]
	mov	rax, qword ptr [rsp + 16]
	mov	qword ptr [rsp + 656], rax
	lea	rdi, [rsp + 144]
	lea	rdx, [rsp + 656]
	mov	rcx, qword ptr [rsp + 8]
	mov	r8, qword ptr [rsp + 24]
	call	_RINvNtCspC5jgAXZxx_17datafusion_common10hash_utils15hash_list_arraylINtB2_21HashStateChildHashingNtNtCsRVR2D38dYC_8foldhash4fast10FixedStateEECsg1h6yN5JQqd_7hashasm
	jmp	.LBB75_682
.LBB75_684:
	lea	rdi, [rsp + 144]
	mov	rsi, r13
	mov	rdx, rbp
	call	qword ptr [rip + _RNvNtCspC5jgAXZxx_17datafusion_common4cast18as_list_view_array@GOTPCREL]
	mov	rax, qword ptr [rsp + 144]
	cmp	rax, -1
	jne	.LBB75_660
	mov	rsi, qword ptr [rsp + 152]
	mov	rax, qword ptr [rsp + 16]
	mov	qword ptr [rsp + 656], rax
	lea	rdi, [rsp + 144]
	lea	rdx, [rsp + 656]
	mov	rcx, qword ptr [rsp + 8]
	mov	r8, qword ptr [rsp + 24]
	call	_RINvNtCspC5jgAXZxx_17datafusion_common10hash_utils20hash_list_view_arraylINtB2_21HashStateChildHashingNtNtCsRVR2D38dYC_8foldhash4fast10FixedStateEECsg1h6yN5JQqd_7hashasm
	jmp	.LBB75_682
.LBB75_650:
	mov	rsi, qword ptr [rbp + 32]
	mov	rdi, r13
	call	_RINvNtCs33DBFqlTBxO_11arrow_array4cast18as_primitive_arrayNtNtB4_5types10Date32TypeECsg1h6yN5JQqd_7hashasm
	movzx	r8d, r12b
	mov	rdi, rax
	mov	rsi, qword ptr [rsp + 16]
	mov	rdx, qword ptr [rsp + 8]
	mov	rcx, qword ptr [rsp + 24]
	call	_RINvNtCspC5jgAXZxx_17datafusion_common10hash_utils20hash_array_primitiveNtNtCs33DBFqlTBxO_11arrow_array5types10Date32TypeNtNtCsRVR2D38dYC_8foldhash4fast10FixedStateECsg1h6yN5JQqd_7hashasm
	jmp	.LBB75_721
.LBB75_651:
	mov	rsi, qword ptr [rbp + 32]
	mov	rdi, r13
	call	_RINvNtCs33DBFqlTBxO_11arrow_array4cast18as_primitive_arrayNtNtB4_5types10Date64TypeECsg1h6yN5JQqd_7hashasm
	movzx	r8d, r12b
	mov	rdi, rax
	mov	rsi, qword ptr [rsp + 16]
	mov	rdx, qword ptr [rsp + 8]
	mov	rcx, qword ptr [rsp + 24]
	call	_RINvNtCspC5jgAXZxx_17datafusion_common10hash_utils20hash_array_primitiveNtNtCs33DBFqlTBxO_11arrow_array5types10Date64TypeNtNtCsRVR2D38dYC_8foldhash4fast10FixedStateECsg1h6yN5JQqd_7hashasm
	jmp	.LBB75_721
.LBB75_672:
	mov	rdx, qword ptr [rbp + 32]
	lea	rdi, [rsp + 144]
	mov	rsi, r13
	mov	rcx, rbx
	call	_RINvNtCspC5jgAXZxx_17datafusion_common4cast23as_generic_binary_arrayxECsg1h6yN5JQqd_7hashasm
	mov	rax, qword ptr [rsp + 144]
	cmp	rax, -1
	jne	.LBB75_660
	mov	rdi, qword ptr [rsp + 152]
	mov	rax, qword ptr [rsp + 16]
	mov	rsi, qword ptr [rax]
	movzx	r8d, r12b
	mov	rdx, qword ptr [rsp + 8]
	mov	rcx, qword ptr [rsp + 24]
	call	_RINvNtCspC5jgAXZxx_17datafusion_common10hash_utils10hash_arrayRINtNtNtCs33DBFqlTBxO_11arrow_array5array10byte_array16GenericByteArrayINtNtB16_5types17GenericBinaryTypexEENtNtCsRVR2D38dYC_8foldhash4fast10FixedStateECsg1h6yN5JQqd_7hashasm
	jmp	.LBB75_721
.LBB75_670:
	mov	rdi, r13
	call	qword ptr [rbp + 32]
	mov	rbx, rax
	lea	rdi, [rsp + 144]
	mov	rsi, rax
	call	qword ptr [rdx + 24]
	movdqu	xmm0, xmmword ptr [rsp + 144]
	pcmpeqb	xmm0, xmmword ptr [rip + .LCPI75_15]
	pmovmskb	eax, xmm0
	cmp	eax, 65535
	jne	.LBB75_735
	mov	rax, qword ptr [rsp + 16]
	mov	rsi, qword ptr [rax]
	movzx	r8d, r12b
	mov	rdi, rbx
	mov	rdx, qword ptr [rsp + 8]
	mov	rcx, qword ptr [rsp + 24]
	call	_RINvNtCspC5jgAXZxx_17datafusion_common10hash_utils10hash_arrayRNtNtNtCs33DBFqlTBxO_11arrow_array5array23fixed_size_binary_array20FixedSizeBinaryArrayNtNtCsRVR2D38dYC_8foldhash4fast10FixedStateECsg1h6yN5JQqd_7hashasm
	jmp	.LBB75_721
.LBB75_688:
	lea	rdi, [rsp + 144]
	mov	rsi, r13
	mov	rdx, rbp
	call	qword ptr [rip + _RNvNtCspC5jgAXZxx_17datafusion_common4cast19as_large_list_array@GOTPCREL]
	mov	rax, qword ptr [rsp + 144]
	cmp	rax, -1
	jne	.LBB75_660
	mov	rsi, qword ptr [rsp + 152]
	mov	rax, qword ptr [rsp + 16]
	mov	qword ptr [rsp + 656], rax
	lea	rdi, [rsp + 144]
	lea	rdx, [rsp + 656]
	mov	rcx, qword ptr [rsp + 8]
	mov	r8, qword ptr [rsp + 24]
	call	_RINvNtCspC5jgAXZxx_17datafusion_common10hash_utils15hash_list_arrayxINtB2_21HashStateChildHashingNtNtCsRVR2D38dYC_8foldhash4fast10FixedStateEECsg1h6yN5JQqd_7hashasm
	jmp	.LBB75_682
.LBB75_686:
	lea	rdi, [rsp + 144]
	mov	rsi, r13
	mov	rdx, rbp
	call	qword ptr [rip + _RNvNtCspC5jgAXZxx_17datafusion_common4cast24as_fixed_size_list_array@GOTPCREL]
	mov	rax, qword ptr [rsp + 144]
	cmp	rax, -1
	jne	.LBB75_660
	mov	rsi, qword ptr [rsp + 152]
	mov	rax, qword ptr [rsp + 16]
	mov	qword ptr [rsp + 656], rax
	lea	rdi, [rsp + 144]
	lea	rdx, [rsp + 656]
	mov	rcx, qword ptr [rsp + 8]
	mov	r8, qword ptr [rsp + 24]
	call	_RINvNtCspC5jgAXZxx_17datafusion_common10hash_utils21hash_fixed_list_arrayINtB2_21HashStateChildHashingNtNtCsRVR2D38dYC_8foldhash4fast10FixedStateEECsg1h6yN5JQqd_7hashasm
	jmp	.LBB75_682
.LBB75_694:
	lea	rdi, [rsp + 144]
	mov	rsi, r13
	mov	rdx, rbp
	call	qword ptr [rip + _RNvNtCspC5jgAXZxx_17datafusion_common4cast14as_union_array@GOTPCREL]
	mov	rax, qword ptr [rsp + 144]
	cmp	rax, -1
	je	.LBB75_695
.LBB75_660:
	mov	rcx, qword ptr [rsp + 152]
	mov	rdx, qword ptr [rsp + 176]
	mov	rsi, qword ptr [rsp + 32]
	mov	qword ptr [rsi + 32], rdx
	movdqu	xmm0, xmmword ptr [rsp + 160]
	movdqu	xmmword ptr [rsi + 16], xmm0
	mov	qword ptr [rsi], rax
	mov	qword ptr [rsi + 8], rcx
	jmp	.LBB75_722
.LBB75_695:
	mov	rsi, qword ptr [rsp + 152]
	mov	rax, qword ptr [rsp + 16]
	mov	qword ptr [rsp + 656], rax
	lea	rdi, [rsp + 144]
	lea	rdx, [rsp + 656]
	mov	rcx, qword ptr [rsp + 8]
	mov	r8, qword ptr [rsp + 24]
	call	_RINvNtCspC5jgAXZxx_17datafusion_common10hash_utils16hash_union_arrayINtB2_21HashStateChildHashingNtNtCsRVR2D38dYC_8foldhash4fast10FixedStateEECsg1h6yN5JQqd_7hashasm
	jmp	.LBB75_682
.LBB75_666:
	mov	rsi, qword ptr [rbp + 32]
	mov	rdi, r13
	call	_RINvNtCs33DBFqlTBxO_11arrow_array4cast18as_primitive_arrayNtNtB4_5types16Time32SecondTypeECsg1h6yN5JQqd_7hashasm
	movzx	r8d, r12b
	mov	rdi, rax
	mov	rsi, qword ptr [rsp + 16]
	mov	rdx, qword ptr [rsp + 8]
	mov	rcx, qword ptr [rsp + 24]
	call	_RINvNtCspC5jgAXZxx_17datafusion_common10hash_utils20hash_array_primitiveNtNtCs33DBFqlTBxO_11arrow_array5types16Time32SecondTypeNtNtCsRVR2D38dYC_8foldhash4fast10FixedStateECsg1h6yN5JQqd_7hashasm
	jmp	.LBB75_721
.LBB75_667:
	mov	rsi, qword ptr [rbp + 32]
	mov	rdi, r13
	call	_RINvNtCs33DBFqlTBxO_11arrow_array4cast18as_primitive_arrayNtNtB4_5types21Time64MicrosecondTypeECsg1h6yN5JQqd_7hashasm
	movzx	r8d, r12b
	mov	rdi, rax
	mov	rsi, qword ptr [rsp + 16]
	mov	rdx, qword ptr [rsp + 8]
	mov	rcx, qword ptr [rsp + 24]
	call	_RINvNtCspC5jgAXZxx_17datafusion_common10hash_utils20hash_array_primitiveNtNtCs33DBFqlTBxO_11arrow_array5types21Time64MicrosecondTypeNtNtCsRVR2D38dYC_8foldhash4fast10FixedStateECsg1h6yN5JQqd_7hashasm
	jmp	.LBB75_721
.LBB75_718:
	mov	rsi, qword ptr [rbp + 32]
	mov	rdi, r13
	call	_RINvNtCs33DBFqlTBxO_11arrow_array4cast12as_run_arrayNtNtB4_5types9Int32TypeECsg1h6yN5JQqd_7hashasm
	mov	rcx, qword ptr [rsp + 16]
	mov	qword ptr [rsp + 656], rcx
	movzx	r9d, r12b
	lea	rdi, [rsp + 144]
	lea	rdx, [rsp + 656]
	mov	rsi, rax
	mov	rcx, qword ptr [rsp + 8]
	mov	r8, qword ptr [rsp + 24]
	call	_RINvNtCspC5jgAXZxx_17datafusion_common10hash_utils14hash_run_arrayNtNtCs33DBFqlTBxO_11arrow_array5types9Int32TypeINtB2_21HashStateChildHashingNtNtCsRVR2D38dYC_8foldhash4fast10FixedStateEECsg1h6yN5JQqd_7hashasm
	jmp	.LBB75_682
.LBB75_717:
	mov	rsi, qword ptr [rbp + 32]
	mov	rdi, r13
	call	_RINvNtCs33DBFqlTBxO_11arrow_array4cast12as_run_arrayNtNtB4_5types9Int16TypeECsg1h6yN5JQqd_7hashasm
	mov	rcx, qword ptr [rsp + 16]
	mov	qword ptr [rsp + 656], rcx
	movzx	r9d, r12b
	lea	rdi, [rsp + 144]
	lea	rdx, [rsp + 656]
	mov	rsi, rax
	mov	rcx, qword ptr [rsp + 8]
	mov	r8, qword ptr [rsp + 24]
	call	_RINvNtCspC5jgAXZxx_17datafusion_common10hash_utils14hash_run_arrayNtNtCs33DBFqlTBxO_11arrow_array5types9Int16TypeINtB2_21HashStateChildHashingNtNtCsRVR2D38dYC_8foldhash4fast10FixedStateEECsg1h6yN5JQqd_7hashasm
	jmp	.LBB75_682
.LBB75_664:
	mov	rdi, r13
	call	_RINvNtCs33DBFqlTBxO_11arrow_array4cast18as_primitive_arrayNtNtB4_5types24TimestampMicrosecondTypeECsg1h6yN5JQqd_7hashasm
	movzx	r8d, r12b
	mov	rdi, rax
	mov	rsi, qword ptr [rsp + 16]
	mov	rdx, qword ptr [rsp + 8]
	mov	rcx, qword ptr [rsp + 24]
	call	_RINvNtCspC5jgAXZxx_17datafusion_common10hash_utils20hash_array_primitiveNtNtCs33DBFqlTBxO_11arrow_array5types24TimestampMicrosecondTypeNtNtCsRVR2D38dYC_8foldhash4fast10FixedStateECsg1h6yN5JQqd_7hashasm
	jmp	.LBB75_721
.LBB75_665:
	mov	rdi, r13
	call	_RINvNtCs33DBFqlTBxO_11arrow_array4cast18as_primitive_arrayNtNtB4_5types23TimestampNanosecondTypeECsg1h6yN5JQqd_7hashasm
	movzx	r8d, r12b
	mov	rdi, rax
	mov	rsi, qword ptr [rsp + 16]
	mov	rdx, qword ptr [rsp + 8]
	mov	rcx, qword ptr [rsp + 24]
	call	_RINvNtCspC5jgAXZxx_17datafusion_common10hash_utils20hash_array_primitiveNtNtCs33DBFqlTBxO_11arrow_array5types23TimestampNanosecondTypeNtNtCsRVR2D38dYC_8foldhash4fast10FixedStateECsg1h6yN5JQqd_7hashasm
	jmp	.LBB75_721
.LBB75_663:
	mov	rdi, r13
	call	_RINvNtCs33DBFqlTBxO_11arrow_array4cast18as_primitive_arrayNtNtB4_5types24TimestampMillisecondTypeECsg1h6yN5JQqd_7hashasm
	movzx	r8d, r12b
	mov	rdi, rax
	mov	rsi, qword ptr [rsp + 16]
	mov	rdx, qword ptr [rsp + 8]
	mov	rcx, qword ptr [rsp + 24]
	call	_RINvNtCspC5jgAXZxx_17datafusion_common10hash_utils20hash_array_primitiveNtNtCs33DBFqlTBxO_11arrow_array5types24TimestampMillisecondTypeNtNtCsRVR2D38dYC_8foldhash4fast10FixedStateECsg1h6yN5JQqd_7hashasm
	jmp	.LBB75_721
.LBB75_734:
	lea	rdi, [rip + .Lanon.98d58ce4ceb595e8c03795bebe6d48b9.7]
	lea	rdx, [rip + .Lanon.98d58ce4ceb595e8c03795bebe6d48b9.8]
	mov	esi, 37
	call	qword ptr [rip + _RNvNtCscI6d9CVNmLh_4core6option13expect_failed@GOTPCREL]
.LBB75_6:
	lea	rcx, [rip + .Lanon.98d58ce4ceb595e8c03795bebe6d48b9.13]
	lea	r9, [rip + .Lanon.98d58ce4ceb595e8c03795bebe6d48b9.42]
	lea	rsi, [rsp + 784]
	lea	rdx, [rsp + 656]
	mov	r8d, 99
	xor	edi, edi
	call	qword ptr [rip + _RINvNtCscI6d9CVNmLh_4core9panicking13assert_failedjjEB4_@GOTPCREL]
.LBB75_705:
	mov	rsi, qword ptr [rbp + 32]
	mov	rdi, r13
	call	_RINvNtCs33DBFqlTBxO_11arrow_array4cast19as_dictionary_arrayNtNtB4_5types9UInt8TypeECsg1h6yN5JQqd_7hashasm
	movzx	r9d, r12b
	lea	rdi, [rsp + 144]
	mov	rsi, rax
	mov	rdx, qword ptr [rsp + 16]
	mov	rcx, qword ptr [rsp + 8]
	mov	r8, qword ptr [rsp + 24]
	call	_RINvNtCspC5jgAXZxx_17datafusion_common10hash_utils15hash_dictionaryNtNtCs33DBFqlTBxO_11arrow_array5types9UInt8TypeNtNtCsRVR2D38dYC_8foldhash4fast10FixedStateECsg1h6yN5JQqd_7hashasm
	jmp	.LBB75_682
.LBB75_703:
	mov	rsi, qword ptr [rbp + 32]
	mov	rdi, r13
	call	_RINvNtCs33DBFqlTBxO_11arrow_array4cast19as_dictionary_arrayNtNtB4_5types9Int32TypeECsg1h6yN5JQqd_7hashasm
	movzx	r9d, r12b
	lea	rdi, [rsp + 144]
	mov	rsi, rax
	mov	rdx, qword ptr [rsp + 16]
	mov	rcx, qword ptr [rsp + 8]
	mov	r8, qword ptr [rsp + 24]
	call	_RINvNtCspC5jgAXZxx_17datafusion_common10hash_utils15hash_dictionaryNtNtCs33DBFqlTBxO_11arrow_array5types9Int32TypeNtNtCsRVR2D38dYC_8foldhash4fast10FixedStateECsg1h6yN5JQqd_7hashasm
	jmp	.LBB75_682
.LBB75_704:
	mov	rsi, qword ptr [rbp + 32]
	mov	rdi, r13
	call	_RINvNtCs33DBFqlTBxO_11arrow_array4cast19as_dictionary_arrayNtNtB4_5types9Int64TypeECsg1h6yN5JQqd_7hashasm
	movzx	r9d, r12b
	lea	rdi, [rsp + 144]
	mov	rsi, rax
	mov	rdx, qword ptr [rsp + 16]
	mov	rcx, qword ptr [rsp + 8]
	mov	r8, qword ptr [rsp + 24]
	call	_RINvNtCspC5jgAXZxx_17datafusion_common10hash_utils15hash_dictionaryNtNtCs33DBFqlTBxO_11arrow_array5types9Int64TypeNtNtCsRVR2D38dYC_8foldhash4fast10FixedStateECsg1h6yN5JQqd_7hashasm
	jmp	.LBB75_682
.LBB75_708:
	mov	rsi, qword ptr [rbp + 32]
	mov	rdi, r13
	call	_RINvNtCs33DBFqlTBxO_11arrow_array4cast19as_dictionary_arrayNtNtB4_5types10UInt64TypeECsg1h6yN5JQqd_7hashasm
	movzx	r9d, r12b
	lea	rdi, [rsp + 144]
	mov	rsi, rax
	mov	rdx, qword ptr [rsp + 16]
	mov	rcx, qword ptr [rsp + 8]
	mov	r8, qword ptr [rsp + 24]
	call	_RINvNtCspC5jgAXZxx_17datafusion_common10hash_utils15hash_dictionaryNtNtCs33DBFqlTBxO_11arrow_array5types10UInt64TypeNtNtCsRVR2D38dYC_8foldhash4fast10FixedStateECsg1h6yN5JQqd_7hashasm
	jmp	.LBB75_682
.LBB75_702:
	mov	rsi, qword ptr [rbp + 32]
	mov	rdi, r13
	call	_RINvNtCs33DBFqlTBxO_11arrow_array4cast19as_dictionary_arrayNtNtB4_5types9Int16TypeECsg1h6yN5JQqd_7hashasm
	movzx	r9d, r12b
	lea	rdi, [rsp + 144]
	mov	rsi, rax
	mov	rdx, qword ptr [rsp + 16]
	mov	rcx, qword ptr [rsp + 8]
	mov	r8, qword ptr [rsp + 24]
	call	_RINvNtCspC5jgAXZxx_17datafusion_common10hash_utils15hash_dictionaryNtNtCs33DBFqlTBxO_11arrow_array5types9Int16TypeNtNtCsRVR2D38dYC_8foldhash4fast10FixedStateECsg1h6yN5JQqd_7hashasm
	jmp	.LBB75_682
.LBB75_706:
	mov	rsi, qword ptr [rbp + 32]
	mov	rdi, r13
	call	_RINvNtCs33DBFqlTBxO_11arrow_array4cast19as_dictionary_arrayNtNtB4_5types10UInt16TypeECsg1h6yN5JQqd_7hashasm
	movzx	r9d, r12b
	lea	rdi, [rsp + 144]
	mov	rsi, rax
	mov	rdx, qword ptr [rsp + 16]
	mov	rcx, qword ptr [rsp + 8]
	mov	r8, qword ptr [rsp + 24]
	call	_RINvNtCspC5jgAXZxx_17datafusion_common10hash_utils15hash_dictionaryNtNtCs33DBFqlTBxO_11arrow_array5types10UInt16TypeNtNtCsRVR2D38dYC_8foldhash4fast10FixedStateECsg1h6yN5JQqd_7hashasm
	jmp	.LBB75_682
.LBB75_707:
	mov	rsi, qword ptr [rbp + 32]
	mov	rdi, r13
	call	_RINvNtCs33DBFqlTBxO_11arrow_array4cast19as_dictionary_arrayNtNtB4_5types10UInt32TypeECsg1h6yN5JQqd_7hashasm
	movzx	r9d, r12b
	lea	rdi, [rsp + 144]
	mov	rsi, rax
	mov	rdx, qword ptr [rsp + 16]
	mov	rcx, qword ptr [rsp + 8]
	mov	r8, qword ptr [rsp + 24]
	call	_RINvNtCspC5jgAXZxx_17datafusion_common10hash_utils15hash_dictionaryNtNtCs33DBFqlTBxO_11arrow_array5types10UInt32TypeNtNtCsRVR2D38dYC_8foldhash4fast10FixedStateECsg1h6yN5JQqd_7hashasm
.LBB75_682:
	cmp	dword ptr [rsp + 144], -1
	je	.LBB75_721
	mov	rax, qword ptr [rsp + 176]
	mov	rcx, qword ptr [rsp + 32]
	mov	qword ptr [rcx + 32], rax
	movdqu	xmm0, xmmword ptr [rsp + 144]
	movups	xmm1, xmmword ptr [rsp + 160]
	movups	xmmword ptr [rcx + 16], xmm1
	movdqu	xmmword ptr [rcx], xmm0
	jmp	.LBB75_722
.LBB75_721:
	mov	rax, qword ptr [rsp + 32]
	mov	qword ptr [rax], -1
.LBB75_722:
	add	rsp, 952
	.cfi_def_cfa_offset 56
	pop	rbx
	.cfi_def_cfa_offset 48
	pop	r12
	.cfi_def_cfa_offset 40
	pop	r13
	.cfi_def_cfa_offset 32
	pop	r14
	.cfi_def_cfa_offset 24
	pop	r15
	.cfi_def_cfa_offset 16
	pop	rbp
	.cfi_def_cfa_offset 8
	ret
.LBB75_600:
	.cfi_def_cfa_offset 1008
	lea	rcx, [rip + .Lanon.98d58ce4ceb595e8c03795bebe6d48b9.13]
	lea	r9, [rip + .Lanon.98d58ce4ceb595e8c03795bebe6d48b9.42]
	lea	rsi, [rsp + 912]
	lea	rdx, [rsp + 920]
	mov	r8d, 99
	xor	edi, edi
	call	qword ptr [rip + _RINvNtCscI6d9CVNmLh_4core9panicking13assert_failedjjEB4_@GOTPCREL]
.LBB75_699:
	lea	rdi, [rip + .Lanon.98d58ce4ceb595e8c03795bebe6d48b9.28]
	lea	rdx, [rip + .Lanon.98d58ce4ceb595e8c03795bebe6d48b9.33]
	mov	esi, 40
	call	qword ptr [rip + _RNvNtCscI6d9CVNmLh_4core9panicking5panic@GOTPCREL]
.LBB75_716:
	lea	rdi, [rip + .Lanon.98d58ce4ceb595e8c03795bebe6d48b9.28]
	lea	rdx, [rip + .Lanon.98d58ce4ceb595e8c03795bebe6d48b9.36]
	mov	esi, 40
	call	qword ptr [rip + _RNvNtCscI6d9CVNmLh_4core9panicking5panic@GOTPCREL]
.LBB75_735:
	lea	rdi, [rip + .Lanon.98d58ce4ceb595e8c03795bebe6d48b9.30]
	call	qword ptr [rip + _RNvNtCscI6d9CVNmLh_4core6option13unwrap_failed@GOTPCREL]
.LBB75_714:
	add	rax, 40
	mov	qword ptr [rsp + 656], rax
	lea	rax, [rsp + 656]
	mov	qword ptr [rsp + 144], rax
	lea	rax, [rip + _RNvXs1i_NtCscI6d9CVNmLh_4core3fmtRNtNtCsk9P1WA8sPmy_12arrow_schema8datatype8DataTypeNtB6_7Display3fmtCsg1h6yN5JQqd_7hashasm]
	mov	qword ptr [rsp + 152], rax
	lea	rdi, [rip + .Lanon.98d58ce4ceb595e8c03795bebe6d48b9.34]
	lea	rdx, [rip + .Lanon.98d58ce4ceb595e8c03795bebe6d48b9.35]
	lea	rsi, [rsp + 144]
	call	qword ptr [rip + _RNvNtCscI6d9CVNmLh_4core9panicking9panic_fmt@GOTPCREL]
.LBB75_700:
	mov	qword ptr [rsp + 656], rcx
	lea	rax, [rsp + 656]
	mov	qword ptr [rsp + 144], rax
	lea	rax, [rip + _RNvXs1i_NtCscI6d9CVNmLh_4core3fmtRNtNtCsk9P1WA8sPmy_12arrow_schema8datatype8DataTypeNtB6_7Display3fmtCsg1h6yN5JQqd_7hashasm]
	mov	qword ptr [rsp + 152], rax
	lea	rdi, [rip + .Lanon.98d58ce4ceb595e8c03795bebe6d48b9.31]
	lea	rdx, [rip + .Lanon.98d58ce4ceb595e8c03795bebe6d48b9.32]
	lea	rsi, [rsp + 144]
	call	qword ptr [rip + _RNvNtCscI6d9CVNmLh_4core9panicking9panic_fmt@GOTPCREL]
.LBB75_726:
.Ltmp475:
	mov	rbx, rax
	mov	rsi, qword ptr [rsp + 656]
	test	rsi, rsi
	je	.LBB75_723
	mov	rdi, qword ptr [rsp + 664]
	mov	edx, 1
	call	qword ptr [rip + _RNvCs9hJ03s5DiqP_7___rustc14___rust_dealloc@GOTPCREL]
.LBB75_723:
	mov	rsi, qword ptr [rsp + 784]
	test	rsi, rsi
	je	.LBB75_725
	mov	rdi, qword ptr [rsp + 792]
	mov	edx, 1
	call	qword ptr [rip + _RNvCs9hJ03s5DiqP_7___rustc14___rust_dealloc@GOTPCREL]
.LBB75_725:
	mov	rdi, rbx
	call	_Unwind_Resume@PLT
.Lfunc_end75:
	.size	_RINvNtCspC5jgAXZxx_17datafusion_common10hash_utils17hash_single_arrayNtNtCsRVR2D38dYC_8foldhash4fast10FixedStateECsg1h6yN5JQqd_7hashasm, .Lfunc_end75-_RINvNtCspC5jgAXZxx_17datafusion_common10hash_utils17hash_single_arrayNtNtCsRVR2D38dYC_8foldhash4fast10FixedStateECsg1h6yN5JQqd_7hashasm
	.cfi_endproc
