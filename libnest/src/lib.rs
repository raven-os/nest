//! Nest backend library.
//!
//! This crate performs backend operations for Raven's Package Manager, like installation, removal, and search of a package.

// Rustc
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unreachable_pub)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]

#![feature(try_blocks)]
#![feature(try_from)]

pub mod cache;
pub mod chroot;
pub mod config;
pub mod error;
pub mod package;
pub mod repository;
pub mod transaction;
