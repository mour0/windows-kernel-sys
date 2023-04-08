#![no_std]

pub mod base;

pub use cty::*;

#[cfg(feature = "ntoskrnl")]
pub mod ntoskrnl;
