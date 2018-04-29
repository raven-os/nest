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
extern crate regex;
extern crate url;
#[macro_use]
extern crate failure_derive;

#[macro_use]
pub mod tty;
#[macro_use]
pub mod error;
pub mod command;
pub mod progress;
pub mod progressbar;
pub mod query;

use clap::{App, AppSettings, Arg, SubCommand};
use libnest::config::Config;
use libnest::repository::{Mirror, Repository};
use url::Url;

fn main() {
    //XXX: Debug values until we have a config file
    let mut config = Config::new();
    let mut repo = Repository::new("stable");

    repo.mirrors_mut()
        .push(Mirror::new(Url::parse("http://raven-os.org").unwrap()));
    repo.mirrors_mut()
        .push(Mirror::new(Url::parse("http://localhost:8002").unwrap()));
    repo.mirrors_mut()
        .push(Mirror::new(Url::parse("http://localhost:8000").unwrap()));
    config.repositories_mut().push(repo);

    let matches = App::new(crate_name!())
        .template("{usage}\n{about}\n\nFLAGS\n{flags}\n\nOPERATIONS\n{subcommands}")
        .usage("nest [FLAGS] OPERATION")
        .about("Raven's package manager")
        .version(crate_version!())
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::ColoredHelp)
        .subcommand(
            SubCommand::with_name("pull").about("Pulls repositories and updates local cache"),
        )
        .subcommand(
            SubCommand::with_name("download")
                .about("Downloads the given packages without installing them")
                .arg(
                    Arg::with_name("PACKAGE")
                        .help("Packages to download")
                        .multiple(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("download-dir")
                        .help("Sets the output directory, overwriting the one in the configuration file")
                        .takes_value(true)
                        .long("download-dir") 
                ),
        )
        .subcommand(
            SubCommand::with_name("install")
                .about("Downloads and installs the given packages")
                .arg(
                    Arg::with_name("PACKAGE")
                        .help("Packages to install")
                        .multiple(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("install-dir")
                        .help("Sets the installation directory, the default being '/'")
                        .takes_value(true)
                        .long("install-dir")
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
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("list")
                .about("Lists installed packages")
        )
        .get_matches();

    let r = match matches.subcommand() {
        ("pull", _) => command::pull::pull(&config),
        ("download", Some(matches)) => command::download::download(&config, matches),
        ("install", Some(matches)) => command::install::install(&config, matches),
        _ => unimplemented!(),
    };

    // All errors arrive here. t's our job to print them on screen and then exit(1).
    if let Err(err) = r {
        use error::QueryErrorKind;
        use std::process::exit;

        eprintln!("{}", format_error!(err));

        // We'd like to print advices for these errors, if any is available.
        // These advices should be preceded by a blank line.

        // Try to downcast errors to query_error
        if let Ok(query_error) = err.downcast::<QueryErrorKind>() {
            eprint!("\n{}", query_error.advices());
        }
        exit(1);
    }
}
