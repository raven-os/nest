//! Nest's backend library
//!
//! This crate performs backend operations for Raven's Package Manager, like installation, remova of a package.

#![warn(missing_docs)]
#![warn(missing_debug_implementations)]

#[macro_use]
mod errors;

pub mod chroot;
pub mod config;
pub mod package;
