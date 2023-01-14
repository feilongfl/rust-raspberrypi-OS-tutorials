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
	csrr t0, mhartid
	beqz t0, L_boot_first_core
	// If execution reaches here, it is the boot core.
	j	.L_parking_loop

L_boot_first_core:
	// Initialize DRAM.
	la	t0, __bss_start
	la	t1, __bss_end_exclusive

.L_bss_init_loop:
	bgeu	t0, t1, .L_prepare_rust
	sw	zero, 0(t0)
	j	.L_bss_init_loop

	// Prepare the jump to Rust code.
.L_prepare_rust:
	// Set the stack pointer.
	la	sp, __boot_core_stack_end_exclusive

	// set return address to _start_rust
	la	ra, .L_parking_loop

	// Jump to Rust code.
	j	_start_rust

	// Infinitely wait for events (aka "park the core").
.L_parking_loop:
	wfi
	j	.L_parking_loop

.size	_start, . - _start
.type	_start, function
.global	_start
