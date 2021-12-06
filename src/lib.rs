#![cfg_attr(not(test), no_std)]

pub mod consts;
pub mod interpret;
pub mod types;

#[cfg(target_arch = "riscv64")]
pub mod jit;
