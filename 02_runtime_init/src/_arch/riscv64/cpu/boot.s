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
	csrr a0, mhartid
	beqz a0, L_boot_first_core
	// If execution reaches here, it is the boot core.
	j	.L_parking_loop

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
