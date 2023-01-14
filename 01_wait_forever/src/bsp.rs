// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2018-2022 Andre Richter <andre.o.richter@gmail.com>
// Copyright (c) 2022-2023 YuLong Yao <feilongphone@gmail.com>

//! Conditional reexporting of Board Support Packages.

#[cfg(any(feature = "bsp_rpi3", feature = "bsp_rpi4"))]
mod raspberrypi;

#[cfg(any(feature = "bsp_rpi3", feature = "bsp_rpi4"))]
pub use raspberrypi::*;

#[cfg(any(feature = "mangopi_mq_pro"))]
mod mangopi_mq_pro;

#[cfg(any(feature = "mangopi_mq_pro"))]
pub use mangopi_mq_pro::*;
