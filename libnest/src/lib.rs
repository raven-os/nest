//! Nest backend library
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
// Clippy
#![warn(fallible_impl_from)]
#![warn(float_cmp_const)]
#![warn(int_plus_one)]
#![warn(mem_forget)]
#![warn(mut_mut)]
#![warn(mutex_integer)]
#![warn(nonminimal_bool)]
#![warn(pub_enum_variant_names)]
#![warn(range_plus_one)]
#![warn(stutter)]

pub mod config;
pub mod repository;
