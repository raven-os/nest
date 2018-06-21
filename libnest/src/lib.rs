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
#![cfg_attr(feature = "cargo-clippy", warn(fallible_impl_from))]
#![cfg_attr(feature = "cargo-clippy", warn(int_plus_one))]
#![cfg_attr(feature = "cargo-clippy", warn(mem_forget))]
#![cfg_attr(feature = "cargo-clippy", warn(mut_mut))]
#![cfg_attr(feature = "cargo-clippy", warn(mutex_integer))]
#![cfg_attr(feature = "cargo-clippy", warn(pub_enum_variant_names))]
#![cfg_attr(feature = "cargo-clippy", warn(range_plus_one))]
#![cfg_attr(feature = "cargo-clippy", warn(use_debug))]
#![cfg_attr(feature = "cargo-clippy", warn(used_underscore_binding))]
#![cfg_attr(feature = "cargo-clippy", warn(wrong_pub_self_convention))]
#![feature(try_from)]
#![feature(catch_expr)]
#![feature(iterator_flatten)]

extern crate curl;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json as json;
extern crate toml;
extern crate url_serde;
#[macro_use]
extern crate lazy_static;
extern crate failure;
extern crate flate2;
extern crate tar;
#[macro_use]
extern crate failure_derive;
extern crate regex;
extern crate semver;

pub mod cache;
pub mod chroot;
pub mod config;
pub mod error;
pub mod package;
pub mod repository;
pub mod transaction;
