//! # Otter Pi
//!
//! A robot built on Raspberry Pi.

#[cfg(all(target_os = "linux", test))]
mod linux;
#[cfg(unix)]
pub mod unix;
