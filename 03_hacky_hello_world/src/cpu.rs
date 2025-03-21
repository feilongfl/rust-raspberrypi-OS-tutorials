// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2020-2022 Andre Richter <andre.o.richter@gmail.com>
// Copyright (c) 2022-2023 YuLong Yao <feilongphone@gmail.com>

//! Processor code.

#[cfg(target_arch = "aarch64")]
#[path = "_arch/aarch64/cpu.rs"]
mod arch_cpu;

#[cfg(target_arch = "riscv64")]
#[path = "_arch/riscv64/cpu.rs"]
mod arch_cpu;

mod boot;

//--------------------------------------------------------------------------------------------------
// Architectural Public Reexports
//--------------------------------------------------------------------------------------------------
pub use arch_cpu::wait_forever;
