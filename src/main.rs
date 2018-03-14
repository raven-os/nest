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

extern crate clap;

mod command;

use clap::{App, AppSettings, Arg, SubCommand};

fn main() {
    let matches = App::new("nest")
        .template("{usage}\n{about}\n\nFLAGS\n{flags}\n\nOPERATIONS\n{subcommands}")
        .usage("nest [FLAGS] OPERATION")
        .about("Raven's package manager")
        .version(&format!(
            "{}.{}.{}",
            env!("CARGO_PKG_VERSION_MAJOR"),
            env!("CARGO_PKG_VERSION_MINOR"),
            env!("CARGO_PKG_VERSION_PATCH"),
        ) as &str)
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
                .about("Search the local database for packages")
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
                .about("List informations about installed packages")
        )
        .get_matches();

    if matches.subcommand_matches("pull").is_some() {
        command::pull::pull();
    } else if let Some(matches) = matches.subcommand_matches("install") {
        command::install::install(matches);
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
