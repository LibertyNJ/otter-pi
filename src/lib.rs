//! # Otter Pi
//!
//! A robot built on Raspberry Pi.

#[cfg(target_os = "linux")]
mod linux;
#[cfg(unix)]
pub mod unix;
