// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2018-2022 Andre Richter <andre.o.richter@gmail.com>
// Copyright (c) 2022-2023 YuLong Yao <feilongphone@gmail.com>

//! BSP console facilities.

use crate::console;
use core::{arch::asm, fmt};

//--------------------------------------------------------------------------------------------------
// Private Definitions
//--------------------------------------------------------------------------------------------------

/// A mystical, magical device for generating QEMU output out of the void.
struct QEMUOutput;

//--------------------------------------------------------------------------------------------------
// Private Code
//--------------------------------------------------------------------------------------------------

static SBI_EXT_0_1_CONSOLE_PUTCHAR: usize = 0x1;
fn sbi_call(eid: usize, fid: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let ret;
    unsafe {
        asm!("ecall",
             inout("a0") arg0 => ret,
                in("a1") arg1,
                in("a2") arg2,
                in("a7") eid,
                in("a6") fid,
        );
    }
    ret
}
/// Implementing `core::fmt::Write` enables usage of the `format_args!` macros, which in turn are
/// used to implement the `kernel`'s `print!` and `println!` macros. By implementing `write_str()`,
/// we get `write_fmt()` automatically.
///
/// See [`src/print.rs`].
///
/// [`src/print.rs`]: ../../print/index.html
impl fmt::Write for QEMUOutput {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            sbi_call(SBI_EXT_0_1_CONSOLE_PUTCHAR, 0, c as usize, 0, 0);
        }

        Ok(())
    }
}

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------

/// Return a reference to the console.
pub fn console() -> impl console::interface::Write {
    QEMUOutput {}
}
