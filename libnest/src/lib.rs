//! Nest's backend library
//!
//! This crate performs backend operations for Raven's Package Manager, like installation, removal of a package.

#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![feature(try_blocks)]
#![feature(result_map_or_else)]

#[macro_use]
mod error;

pub mod cache;
pub mod chroot;
pub mod config;
pub mod lock_file;
pub mod package;
pub mod repository;
pub mod transaction;
