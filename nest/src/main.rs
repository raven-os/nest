//! Nest is Raven's package manager.
//!
//! This implementation is the CLI (Command-Line Interface) version of Nest. A GUI version may be
//! added one day.
//!
//! Nest's implementation is split in two parts: `nest` (where you are), and
//! [`libnest`](../libnest/index.html).
//!
//! [`libnest`](../libnest/index.html) is a back-end library common to all front-end of Nest (CLI or GUI) that does most of the
//! stuff. It handles repositories, mirrors, etc. It downloads, installs and removes packages.
//! It's the big one.
//!
//! `nest`, in contrast, is only a front-end to [`libnest`](../libnest/index.html). It's a command-line tool to interact
//! with [`libnest`](../libnest/index.html). and maintain the system.

// Rustc
#![warn(missing_debug_implementations)]
#![warn(trivial_numeric_casts)]
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
#![feature(catch_expr)]

extern crate ansi_term;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate lazy_static;
extern crate failure;
extern crate libc;
extern crate libnest;
#[macro_use]
extern crate failure_derive;

#[macro_use]
pub mod tty;
#[macro_use]
pub mod error;
pub mod command;
pub mod progressbar;
//pub mod query;

use clap::{App, AppSettings, Arg, SubCommand};
use failure::Error;
use libnest::config::Config;

fn main() {
    let matches = App::new(crate_name!())
        .template("{usage}\n\n{about}\n\nFLAGS\n{flags}\n\nSUBCOMMANDS\n{subcommands}")
        .usage("nest [FLAGS] SUBCOMMAND [SUBCOMMANDS'S FLAGS]")
        .about("Raven-OS's package manager.")
        .version(crate_version!())
        .author(crate_authors!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::ColoredHelp)
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .arg(
            Arg::with_name("chroot")
                .long("chroot")
                .help("Use the current configuration but operate on the given folder, as if it was the root folder")
                .takes_value(true)
        )
        .subcommand(
            SubCommand::with_name("pull").about("Pull repositories and update local cache"),
        )
        .subcommand(
            SubCommand::with_name("install")
                .about("Download and install the given packages")
                .arg(
                    Arg::with_name("PACKAGE")
                        .help("Packages to install")
                        .multiple(true)
                        .required(true),
                )
        )
        .get_matches();

    let res: Result<_, Error> = do catch {
        // Load config file
        let mut config = Config::load()?;

        // Chroot (if provided)
        if let Some(path) = matches.value_of("chroot") {
            config.paths_mut().chroot(path);
        }

        match matches.subcommand() {
            ("pull", _) => command::pull::pull(&config),
            ("install", Some(matches)) => command::install::install(&config, matches),
            _ => unimplemented!(),
        }?
    };

    // All fatal errors arrive here. It's our job to print them on screen and then exit(1).
    if let Err(err) = res {
        use error::QueryError;
        use std::process::exit;

        // TODO try the backtrace! macro from failure
        eprintln!("{}", format_error!(err));

        // We'd like to print advices for these errors, if any are available.
        // These advices should be preceded by a blank line.

        // Try to downcast errors to query_error
        // XXX: Find a better way, this is hackish
        if let Ok(query_error) = err.downcast::<QueryError>() {
            eprint!("\n{}", query_error.advices());
        }
        exit(1);
    }
}
