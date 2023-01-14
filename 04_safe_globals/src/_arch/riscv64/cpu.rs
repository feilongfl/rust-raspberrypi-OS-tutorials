// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2022-2023 YuLong Yao <feilongphone@gmail.com>

use riscv::asm;

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------

/// Pause execution on the core.
// #[inline(always)]
pub fn wait_forever() -> ! {
    loop {
        unsafe { asm::wfi() }
    }
}
