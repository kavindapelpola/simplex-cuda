	.file	"test_vec.f1360ebcce803c0a-cgu.0"
	.section	.text._ZN3std2rt10lang_start17h5dcecfaef7ae182dE,"ax",@progbits
	.hidden	_ZN3std2rt10lang_start17h5dcecfaef7ae182dE
	.globl	_ZN3std2rt10lang_start17h5dcecfaef7ae182dE
	.p2align	4
	.type	_ZN3std2rt10lang_start17h5dcecfaef7ae182dE,@function
_ZN3std2rt10lang_start17h5dcecfaef7ae182dE:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	mov	w4, w3
	mov	x3, x2
	mov	x2, x1
	str	x0, [sp, #8]
	adrp	x1, .Lanon.513646415e50f300ffbf9e3172ffdd58.0
	add	x1, x1, :lo12:.Lanon.513646415e50f300ffbf9e3172ffdd58.0
	add	x0, sp, #8
	bl	_ZN3std2rt19lang_start_internal17h41a352080e87c390E
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
.Lfunc_end0:
	.size	_ZN3std2rt10lang_start17h5dcecfaef7ae182dE, .Lfunc_end0-_ZN3std2rt10lang_start17h5dcecfaef7ae182dE
	.cfi_endproc

	.section	".text._ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h2ba9e6a148a01d64E","ax",@progbits
	.p2align	4
	.type	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h2ba9e6a148a01d64E,@function
_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h2ba9e6a148a01d64E:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	x0, [x0]
	bl	_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h0caf8a1257a8f246E
	mov	w0, wzr
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
.Lfunc_end1:
	.size	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h2ba9e6a148a01d64E, .Lfunc_end1-_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h2ba9e6a148a01d64E
	.cfi_endproc

	.section	.text._ZN3std3sys9backtrace28__rust_begin_short_backtrace17h0caf8a1257a8f246E,"ax",@progbits
	.p2align	4
	.type	_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h0caf8a1257a8f246E,@function
_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h0caf8a1257a8f246E:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	blr	x0
	//APP
	//NO_APP
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
.Lfunc_end2:
	.size	_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h0caf8a1257a8f246E, .Lfunc_end2-_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h0caf8a1257a8f246E
	.cfi_endproc

	.section	".text._ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h69d4a0332202c0c0E","ax",@progbits
	.p2align	4
	.type	_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h69d4a0332202c0c0E,@function
_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h69d4a0332202c0c0E:
	.cfi_startproc
	ldr	x0, [x0]
	b	_ZN4core3fmt5float50_$LT$impl$u20$core..fmt..Debug$u20$for$u20$f32$GT$3fmt17h4662ac25a8775292E
.Lfunc_end3:
	.size	_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h69d4a0332202c0c0E, .Lfunc_end3-_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h69d4a0332202c0c0E
	.cfi_endproc

	.section	".text._ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hd05cad5de31a77bfE","ax",@progbits
	.p2align	4
	.type	_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hd05cad5de31a77bfE,@function
_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hd05cad5de31a77bfE:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	x0, [x0]
	bl	_ZN3std3sys9backtrace28__rust_begin_short_backtrace17h0caf8a1257a8f246E
	mov	w0, wzr
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
.Lfunc_end4:
	.size	_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hd05cad5de31a77bfE, .Lfunc_end4-_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hd05cad5de31a77bfE
	.cfi_endproc

	.section	".text._ZN65_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hc544ac521547ba6bE","ax",@progbits
	.p2align	4
	.type	_ZN65_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hc544ac521547ba6bE,@function
_ZN65_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hc544ac521547ba6bE:
	.cfi_startproc
	sub	sp, sp, #80
	.cfi_def_cfa_offset 80
	stp	x29, x30, [sp, #32]
	stp	x22, x21, [sp, #48]
	stp	x20, x19, [sp, #64]
	add	x29, sp, #32
	.cfi_def_cfa w29, 48
	.cfi_offset w19, -8
	.cfi_offset w20, -16
	.cfi_offset w21, -24
	.cfi_offset w22, -32
	.cfi_offset w30, -40
	.cfi_offset w29, -48
	ldp	x20, x19, [x0, #8]
	add	x8, sp, #8
	mov	x0, x1
	bl	_ZN4core3fmt9Formatter10debug_list17ha5d931c22fef17d7E
	cbz	x19, .LBB5_3
	lsl	x21, x19, #2
	adrp	x19, .Lanon.513646415e50f300ffbf9e3172ffdd58.1
	add	x19, x19, :lo12:.Lanon.513646415e50f300ffbf9e3172ffdd58.1
	.p2align	5, , 16
.LBB5_2:
	add	x0, sp, #8
	sub	x1, x29, #8
	add	x22, x20, #4
	stur	x20, [x29, #-8]
	mov	x2, x19
	bl	_ZN4core3fmt8builders9DebugList5entry17h356f636ae901bd7cE
	subs	x21, x21, #4
	mov	x20, x22
	b.ne	.LBB5_2
.LBB5_3:
	add	x0, sp, #8
	bl	_ZN4core3fmt8builders9DebugList6finish17h2a8d86543a994d9aE
	.cfi_def_cfa wsp, 80
	ldp	x20, x19, [sp, #64]
	ldp	x22, x21, [sp, #48]
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #80
	.cfi_def_cfa_offset 0
	.cfi_restore w19
	.cfi_restore w20
	.cfi_restore w21
	.cfi_restore w22
	.cfi_restore w30
	.cfi_restore w29
	ret
.Lfunc_end5:
	.size	_ZN65_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hc544ac521547ba6bE, .Lfunc_end5-_ZN65_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hc544ac521547ba6bE
	.cfi_endproc

	.section	.text._ZN8test_vec12test_indexed17h10f032e23d6fa41cE,"ax",@progbits
	.p2align	4
	.type	_ZN8test_vec12test_indexed17h10f032e23d6fa41cE,@function
_ZN8test_vec12test_indexed17h10f032e23d6fa41cE:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	cbz	x1, .LBB6_8
	sub	x8, x1, #1
	cmp	x3, x8
	csel	x8, x3, x8, lo
	add	x8, x8, #1
	cmp	x8, #8
	b.hi	.LBB6_3
	mov	x8, xzr
	b	.LBB6_5
.LBB6_3:
	ands	x9, x8, #0x7
	mov	w10, #8
	fmov	v0.4s, #3.00000000
	csel	x9, x10, x9, eq
	add	x10, x0, #16
	sub	x8, x8, x9
	add	x9, x2, #16
	mov	x11, x8
	.p2align	5, , 16
.LBB6_4:
	ldp	q1, q2, [x9, #-16]
	ldp	q3, q4, [x10, #-16]
	subs	x11, x11, #8
	add	x9, x9, #32
	fmul	v2.4s, v2.4s, v0.4s
	fmul	v1.4s, v1.4s, v0.4s
	fsub	v2.4s, v4.4s, v2.4s
	fsub	v1.4s, v3.4s, v1.4s
	stp	q1, q2, [x10, #-16]
	add	x10, x10, #32
	b.ne	.LBB6_4
.LBB6_5:
	fmov	s0, #-3.00000000
	.p2align	5, , 16
.LBB6_6:
	cmp	x3, x8
	b.eq	.LBB6_9
	ldr	s2, [x2, x8, lsl #2]
	ldr	s1, [x0, x8, lsl #2]
	fmul	s2, s2, s0
	fadd	s1, s1, s2
	str	s1, [x0, x8, lsl #2]
	add	x8, x8, #1
	cmp	x1, x8
	b.ne	.LBB6_6
.LBB6_8:
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
.LBB6_9:
	.cfi_restore_state
	adrp	x2, .Lanon.513646415e50f300ffbf9e3172ffdd58.3
	add	x2, x2, :lo12:.Lanon.513646415e50f300ffbf9e3172ffdd58.3
	mov	x0, x3
	mov	x1, x3
	bl	_ZN4core9panicking18panic_bounds_check17h03a102694e080f49E
.Lfunc_end6:
	.size	_ZN8test_vec12test_indexed17h10f032e23d6fa41cE, .Lfunc_end6-_ZN8test_vec12test_indexed17h10f032e23d6fa41cE
	.cfi_endproc

	.section	.text._ZN8test_vec4main17h8480a02c81f3a94cE,"ax",@progbits
	.hidden	_ZN8test_vec4main17h8480a02c81f3a94cE
	.globl	_ZN8test_vec4main17h8480a02c81f3a94cE
	.p2align	4
	.type	_ZN8test_vec4main17h8480a02c81f3a94cE,@function
_ZN8test_vec4main17h8480a02c81f3a94cE:
.Lfunc_begin0:
	.cfi_startproc
	.cfi_personality 156, DW.ref.rust_eh_personality
	.cfi_lsda 28, .Lexception0
	sub	sp, sp, #128
	.cfi_def_cfa_offset 128
	stp	x29, x30, [sp, #96]
	stp	x20, x19, [sp, #112]
	add	x29, sp, #96
	.cfi_def_cfa w29, 32
	.cfi_offset w19, -8
	.cfi_offset w20, -16
	.cfi_offset w30, -24
	.cfi_offset w29, -32
	.cfi_remember_state
	bl	_RNvCsiGVaDesi5rv_7___rustc35___rust_no_alloc_shim_is_unstable_v2
	mov	w0, #24
	mov	w1, #4
	bl	_RNvCsiGVaDesi5rv_7___rustc12___rust_alloc
	cbz	x0, .LBB7_7
	fmov	v0.4s, #1.00000000
	mov	w8, #6
	mov	x20, x0
	stp	x8, x0, [sp, #8]
	str	x8, [sp, #24]
	str	q0, [x0]
	fmov	v0.2s, #1.00000000
	str	d0, [x0, #16]
	bl	_RNvCsiGVaDesi5rv_7___rustc35___rust_no_alloc_shim_is_unstable_v2
	mov	w0, #24
	mov	w1, #4
	bl	_RNvCsiGVaDesi5rv_7___rustc12___rust_alloc
	cbz	x0, .LBB7_8
	movi	v0.4s, #64, lsl #24
	mov	w1, #6
	mov	w3, #6
	mov	x19, x0
	mov	x2, x19
	str	q0, [x0]
	movi	v0.2s, #64, lsl #24
	str	d0, [x0, #16]
	mov	x0, x20
	bl	_ZN8test_vec8test_zip17hb77d7eebda2aba23E
.Ltmp0:
	mov	w1, #6
	mov	w3, #6
	mov	x0, x20
	mov	x2, x19
	bl	_ZN8test_vec12test_indexed17h10f032e23d6fa41cE
.Ltmp1:
	add	x8, sp, #8
	adrp	x9, _ZN65_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hc544ac521547ba6bE
	add	x9, x9, :lo12:_ZN65_$LT$alloc..vec..Vec$LT$T$C$A$GT$$u20$as$u20$core..fmt..Debug$GT$3fmt17hc544ac521547ba6bE
	adrp	x10, .Lanon.513646415e50f300ffbf9e3172ffdd58.5
	add	x10, x10, :lo12:.Lanon.513646415e50f300ffbf9e3172ffdd58.5
	stp	x8, x9, [x29, #-16]
	mov	w8, #2
	mov	w9, #1
	stp	x10, x8, [sp, #32]
	sub	x8, x29, #16
	stp	x9, xzr, [sp, #56]
	str	x8, [sp, #48]
.Ltmp2:
	add	x0, sp, #32
	bl	_ZN3std2io5stdio6_print17hdd860001209cfb53E
.Ltmp3:
	mov	w1, #24
	mov	w2, #4
	mov	x0, x19
	bl	_RNvCsiGVaDesi5rv_7___rustc14___rust_dealloc
	ldr	x8, [sp, #8]
	cbz	x8, .LBB7_6
	ldr	x0, [sp, #16]
	lsl	x1, x8, #2
	mov	w2, #4
	bl	_RNvCsiGVaDesi5rv_7___rustc14___rust_dealloc
.LBB7_6:
	.cfi_def_cfa wsp, 128
	ldp	x20, x19, [sp, #112]
	ldp	x29, x30, [sp, #96]
	add	sp, sp, #128
	.cfi_def_cfa_offset 0
	.cfi_restore w19
	.cfi_restore w20
	.cfi_restore w30
	.cfi_restore w29
	ret
.LBB7_7:
	.cfi_restore_state
	mov	w0, #4
	mov	w1, #24
	bl	_ZN5alloc7raw_vec12handle_error17hbfb8377505525b80E
.LBB7_8:
.Ltmp5:
	mov	w0, #4
	mov	w1, #24
	bl	_ZN5alloc7raw_vec12handle_error17hbfb8377505525b80E
.Ltmp6:
	brk	#0x1
.LBB7_10:
.Ltmp4:
	mov	w1, #24
	mov	w2, #4
	mov	x20, x0
	mov	x0, x19
	bl	_RNvCsiGVaDesi5rv_7___rustc14___rust_dealloc
	ldr	x8, [sp, #8]
	cbnz	x8, .LBB7_12
	b	.LBB7_13
.LBB7_11:
.Ltmp7:
	mov	x20, x0
	mov	w8, #6
.LBB7_12:
	ldr	x0, [sp, #16]
	lsl	x1, x8, #2
	mov	w2, #4
	bl	_RNvCsiGVaDesi5rv_7___rustc14___rust_dealloc
.LBB7_13:
	mov	x0, x20
	bl	_Unwind_Resume
.Lfunc_end7:
	.size	_ZN8test_vec4main17h8480a02c81f3a94cE, .Lfunc_end7-_ZN8test_vec4main17h8480a02c81f3a94cE
	.cfi_endproc
	.section	.gcc_except_table._ZN8test_vec4main17h8480a02c81f3a94cE,"a",@progbits
	.p2align	2, 0x0
GCC_except_table7:
.Lexception0:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 .Lcst_end0-.Lcst_begin0
.Lcst_begin0:
	.uleb128 .Ltmp0-.Lfunc_begin0
	.uleb128 .Ltmp3-.Ltmp0
	.uleb128 .Ltmp4-.Lfunc_begin0
	.byte	0
	.uleb128 .Ltmp3-.Lfunc_begin0
	.uleb128 .Ltmp5-.Ltmp3
	.byte	0
	.byte	0
	.uleb128 .Ltmp5-.Lfunc_begin0
	.uleb128 .Ltmp6-.Ltmp5
	.uleb128 .Ltmp7-.Lfunc_begin0
	.byte	0
	.uleb128 .Ltmp6-.Lfunc_begin0
	.uleb128 .Lfunc_end7-.Ltmp6
	.byte	0
	.byte	0
.Lcst_end0:
	.p2align	2, 0x0

	.section	.text._ZN8test_vec8test_zip17hb77d7eebda2aba23E,"ax",@progbits
	.p2align	4
	.type	_ZN8test_vec8test_zip17hb77d7eebda2aba23E,@function
_ZN8test_vec8test_zip17hb77d7eebda2aba23E:
	.cfi_startproc
	cmp	x3, x1
	csel	x8, x3, x1, lo
	cbz	x8, .LBB8_8
	cmp	x8, #8
	b.hs	.LBB8_3
	mov	x9, xzr
	b	.LBB8_6
.LBB8_3:
	fmov	v0.4s, #3.00000000
	and	x9, x8, #0xfffffffffffffff8
	add	x10, x2, #16
	add	x11, x0, #16
	mov	x12, x9
	.p2align	5, , 16
.LBB8_4:
	ldp	q1, q2, [x10, #-16]
	ldp	q3, q4, [x11, #-16]
	subs	x12, x12, #8
	add	x10, x10, #32
	fmul	v2.4s, v2.4s, v0.4s
	fmul	v1.4s, v1.4s, v0.4s
	fsub	v2.4s, v4.4s, v2.4s
	fsub	v1.4s, v3.4s, v1.4s
	stp	q1, q2, [x11, #-16]
	add	x11, x11, #32
	b.ne	.LBB8_4
	cmp	x8, x9
	b.eq	.LBB8_8
.LBB8_6:
	fmov	s0, #-3.00000000
	sub	x8, x8, x9
	add	x10, x0, x9, lsl #2
	add	x9, x2, x9, lsl #2
	.p2align	5, , 16
.LBB8_7:
	ldr	s1, [x9], #4
	ldr	s2, [x10]
	subs	x8, x8, #1
	fmul	s1, s1, s0
	fadd	s1, s2, s1
	str	s1, [x10], #4
	b.ne	.LBB8_7
.LBB8_8:
	ret
.Lfunc_end8:
	.size	_ZN8test_vec8test_zip17hb77d7eebda2aba23E, .Lfunc_end8-_ZN8test_vec8test_zip17hb77d7eebda2aba23E
	.cfi_endproc

	.section	.text.main,"ax",@progbits
	.globl	main
	.p2align	4
	.type	main,@function
main:
	.cfi_startproc
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	mov	x3, x1
	sxtw	x2, w0
	adrp	x8, _ZN8test_vec4main17h8480a02c81f3a94cE
	add	x8, x8, :lo12:_ZN8test_vec4main17h8480a02c81f3a94cE
	adrp	x1, .Lanon.513646415e50f300ffbf9e3172ffdd58.0
	add	x1, x1, :lo12:.Lanon.513646415e50f300ffbf9e3172ffdd58.0
	add	x0, sp, #8
	str	x8, [sp, #8]
	mov	w4, wzr
	bl	_ZN3std2rt19lang_start_internal17h41a352080e87c390E
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
.Lfunc_end9:
	.size	main, .Lfunc_end9-main
	.cfi_endproc

	.type	.Lanon.513646415e50f300ffbf9e3172ffdd58.0,@object
	.section	.data.rel.ro..Lanon.513646415e50f300ffbf9e3172ffdd58.0,"aw",@progbits
	.p2align	3, 0x0
.Lanon.513646415e50f300ffbf9e3172ffdd58.0:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.xword	_ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hd05cad5de31a77bfE
	.xword	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h2ba9e6a148a01d64E
	.xword	_ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h2ba9e6a148a01d64E
	.size	.Lanon.513646415e50f300ffbf9e3172ffdd58.0, 48

	.type	.Lanon.513646415e50f300ffbf9e3172ffdd58.1,@object
	.section	.data.rel.ro..Lanon.513646415e50f300ffbf9e3172ffdd58.1,"aw",@progbits
	.p2align	3, 0x0
.Lanon.513646415e50f300ffbf9e3172ffdd58.1:
	.asciz	"\000\000\000\000\000\000\000\000\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.xword	_ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h69d4a0332202c0c0E
	.size	.Lanon.513646415e50f300ffbf9e3172ffdd58.1, 32

	.type	.Lanon.513646415e50f300ffbf9e3172ffdd58.2,@object
	.section	.rodata.str1.1,"aMS",@progbits,1
.Lanon.513646415e50f300ffbf9e3172ffdd58.2:
	.asciz	"/tmp/test_vec.rs"
	.size	.Lanon.513646415e50f300ffbf9e3172ffdd58.2, 17

	.type	.Lanon.513646415e50f300ffbf9e3172ffdd58.3,@object
	.section	.data.rel.ro..Lanon.513646415e50f300ffbf9e3172ffdd58.3,"aw",@progbits
	.p2align	3, 0x0
.Lanon.513646415e50f300ffbf9e3172ffdd58.3:
	.xword	.Lanon.513646415e50f300ffbf9e3172ffdd58.2
	.asciz	"\020\000\000\000\000\000\000\000\013\000\000\000*\000\000"
	.size	.Lanon.513646415e50f300ffbf9e3172ffdd58.3, 24

	.type	.Lanon.513646415e50f300ffbf9e3172ffdd58.4,@object
	.section	.rodata..Lanon.513646415e50f300ffbf9e3172ffdd58.4,"a",@progbits
.Lanon.513646415e50f300ffbf9e3172ffdd58.4:
	.byte	10
	.size	.Lanon.513646415e50f300ffbf9e3172ffdd58.4, 1

	.type	.Lanon.513646415e50f300ffbf9e3172ffdd58.5,@object
	.section	.data.rel.ro..Lanon.513646415e50f300ffbf9e3172ffdd58.5,"aw",@progbits
	.p2align	3, 0x0
.Lanon.513646415e50f300ffbf9e3172ffdd58.5:
	.xword	1
	.zero	8
	.xword	.Lanon.513646415e50f300ffbf9e3172ffdd58.4
	.asciz	"\001\000\000\000\000\000\000"
	.size	.Lanon.513646415e50f300ffbf9e3172ffdd58.5, 32

	.hidden	DW.ref.rust_eh_personality
	.weak	DW.ref.rust_eh_personality
	.section	.data.DW.ref.rust_eh_personality,"awG",@progbits,DW.ref.rust_eh_personality,comdat
	.p2align	3, 0x0
	.type	DW.ref.rust_eh_personality,@object
	.size	DW.ref.rust_eh_personality, 8
DW.ref.rust_eh_personality:
	.xword	rust_eh_personality
	.ident	"rustc version 1.92.0 (ded5c06cf 2025-12-08)"
	.section	".note.GNU-stack","",@progbits
