// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2021-2022 Andre Richter <andre.o.richter@gmail.com>
// Copyright (c) 2022-2023 YuLong Yao <feilongphone@gmail.com>

//! Architectural boot code.
//!
//! # Orientation
//!
//! Since arch modules are imported into generic modules using the path attribute, the path of this
//! file is:
//!
//! crate::cpu::boot::arch_boot

use core::arch::global_asm;

// Assembly counterpart to this file.
global_asm!(include_str!("boot.s"),);

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------

/// The Rust entry of the `kernel` binary.
///
/// The function is called from the assembly `_start` function.
#[no_mangle]
pub unsafe fn _start_rust() -> ! {
    crate::kernel_init()
}
