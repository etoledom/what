#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "macos")]
mod macos;

mod lsof_utils;
mod common;

pub use common::*;
