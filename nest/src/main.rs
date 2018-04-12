//! Nest is Raven's package manager.
//!
//! This implementation is the CLI (command-line interface) version of Nest. A GUI version may be
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
#![warn(trivial_casts)]
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
#![cfg_attr(feature = "cargo-clippy", warn(stutter))]
#![cfg_attr(feature = "cargo-clippy", warn(use_debug))]
#![cfg_attr(feature = "cargo-clippy", warn(used_underscore_binding))]
#![cfg_attr(feature = "cargo-clippy", warn(wrong_pub_self_convention))]

extern crate ansi_term;
extern crate clap;
#[macro_use]
extern crate lazy_static;
extern crate libc;
extern crate libnest;
extern crate regex;

#[macro_use]
pub mod tty;
pub mod command;
pub mod progressbar;
pub mod query;

use std::path::Path;

use clap::{App, AppSettings, Arg, SubCommand};
use libnest::config::Config;

fn main() {
    let mut config = Config::new();
    config.load(Path::new("Config.toml"));

    let matches = App::new("nest")
        .template("{usage}\n{about}\n\nFLAGS\n{flags}\n\nOPERATIONS\n{subcommands}")
        .usage("nest [FLAGS] OPERATION")
        .about("Raven's package manager")
        .version(format!(
            "{}.{}.{}",
            env!("CARGO_PKG_VERSION_MAJOR"),
            env!("CARGO_PKG_VERSION_MINOR"),
            env!("CARGO_PKG_VERSION_PATCH"),
        ).as_ref())
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .subcommand(
            SubCommand::with_name("pull").about("Pulls repositories and updates local cache"),
        )
        .subcommand(
            SubCommand::with_name("install")
                .about("Installs the given packages")
                .arg(
                    Arg::with_name("PACKAGE")
                        .help("Packages to install")
                        .multiple(true)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("uninstall")
                .visible_alias("remove")
                .about("Uninstalls the given packages")
                .arg(
                    Arg::with_name("PACKAGE")
                        .help("Packages to uninstall")
                        .multiple(true)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("search")
                .about("Searches the local database for packages")
                .arg(
                    Arg::with_name("KEYWORD")
                        .help("A keyword that a package name or description must contain")
                        .multiple(true)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("upgrade")
                .about("Upgrades installed packages")
                .arg(
                    Arg::with_name("PACKAGE")
                        .help("Packages to upgrade. If no packages are given, upgrades all installed packages")
                        .multiple(true)
                ),
        )
        .subcommand(
            SubCommand::with_name("list")
                .about("Lists informations about installed packages")
        )
        .get_matches();

    if matches.subcommand_matches("pull").is_some() {
        command::pull::pull(&config);
    } else if let Some(matches) = matches.subcommand_matches("install") {
        command::install::install(&config, matches);
    } else if let Some(matches) = matches.subcommand_matches("uninstall") {
        command::uninstall::uninstall(matches);
    } else if let Some(matches) = matches.subcommand_matches("search") {
        command::search::search(matches);
    } else if let Some(matches) = matches.subcommand_matches("search") {
        command::search::search(matches);
    } else if let Some(matches) = matches.subcommand_matches("upgrade") {
        command::upgrade::upgrade(matches);
    } else if let Some(matches) = matches.subcommand_matches("list") {
        command::list::list(matches);
    }
}
