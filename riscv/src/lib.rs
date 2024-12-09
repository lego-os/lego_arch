//! 
#![no_std]
#[cfg(feature = "dbg")]
pub mod dbg;
#[cfg(feature = "hpv")]
pub mod hpv;
#[cfg(feature = "trap")]
pub mod trap;

#[cfg(feature = "mch")]
pub mod mch;
#[cfg(feature = "spv")]
pub mod spv;
#[cfg(feature = "user")]
pub mod user;
#[cfg(feature = "virt")]
pub mod virt;

#[cfg(any(feature = "spv", feature = "mch", feature = "user"))]
mod asm;

#[cfg(any(feature = "spv", feature = "mch", feature = "user"))]
pub use asm::*;
#[cfg(any(feature = "spv", feature = "mch", feature = "user"))]
mod macros;
#[cfg(feature = "mch")]
mod mmap_time;
#[cfg(feature = "mch")]
pub use mmap_time::*;

#[cfg(feature = "dbg")]
pub use dbg::*;
#[cfg(feature = "hpv")]
pub use hpv::*;
#[cfg(feature = "mch")]
pub use mch::*;
#[cfg(feature = "spv")]
pub use spv::*;
#[cfg(feature = "user")]
pub use user::*;
#[cfg(feature = "virt")]
pub use virt::*;

pub const MODE_U: u64 = 0;
pub const MODE_S: u64 = 0x01;
pub const MODE_M: u64 = 0x11;
