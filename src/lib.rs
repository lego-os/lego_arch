//!
#![no_std]
mod arm;
#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
pub mod riscv;
