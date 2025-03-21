// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2021-2022 Andre Richter <andre.o.richter@gmail.com>
// Copyright (c) 2022-2023 YuLong Yao <feilongphone@gmail.com>

//! Boot code.

#[cfg(target_arch = "aarch64")]
#[path = "../_arch/aarch64/cpu/boot.rs"]
mod arch_boot;

#[cfg(target_arch = "riscv64")]
#[path = "../_arch/riscv64/cpu/boot.rs"]
mod arch_boot;
