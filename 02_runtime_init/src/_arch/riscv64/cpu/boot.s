// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2022-2023 YuLong Yao <feilongphone@gmail.com>

//--------------------------------------------------------------------------------------------------
// Definitions
//--------------------------------------------------------------------------------------------------

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------
.section .text._start

//------------------------------------------------------------------------------
// fn _start()
//------------------------------------------------------------------------------
_start:
	// Only proceed on the boot core. Park it otherwise.
	// OpenSBI will choose one core to boot first,
	// and the other cores will be parked until `ipi` interrupt is arrive.
	lla	t0, {OPENSBI_HART_LOTTERY}
	li	t1, 1
	amoadd.w t0, t1, (t0)
	// If the return value is 0, then this is the boot core.
	// Otherwise, loop infinitely.
	bnez	t0, .L_parking_loop

L_boot_first_core:
	// Initialize DRAM.
	la	a0, __bss_start
	la	a1, __bss_end_exclusive

.L_bss_init_loop:
	bgeu	a0, a1, .L_prepare_rust
	sw	zero, 0(a0)
	j	.L_bss_init_loop

	// Prepare the jump to Rust code.
.L_prepare_rust:
	// Set the stack pointer.
	la a0, __boot_core_stack_end_exclusive
	mv	sp, a0

	// Jump to Rust code.
	j	_start_rust

	// Infinitely wait for events (aka "park the core").
.L_parking_loop:
	wfi
	j	.L_parking_loop

.size	_start, . - _start
.type	_start, function
.global	_start
