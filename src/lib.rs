//! 

#![no_std]
#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
extern crate riscv;
#[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
pub use riscv::*;
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
extern crate x86;
#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
extern crate arm;
#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
pub use arm::*;