#![feature(try_blocks)]

use clap::{crate_authors, crate_name, crate_version, App, AppSettings, Arg, SubCommand};
use libnest::config;

pub mod commands;

fn main() {
    let matches = App::new(crate_name!())
        .template("{usage}\n\n{about}\n\nOPTIONS\n{flags}\n\nSUBCOMMANDS\n{subcommands}")
        .usage("nest [OPTION]... SUBCOMMAND [SUBCOMMAND OPTIONS]...")
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
                .help("Set the level of verbosity"),
        )
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .help("Use the given path as a configuration file")
                .takes_value(true)
                .default_value("/etc/nest/config.toml")
        )
        .arg(
            Arg::with_name("chroot")
                .long("chroot")
                .help("Use the current configuration but operate on the given folder, as if it was the root folder")
                .takes_value(true)
        )
        .subcommand(
            SubCommand::with_name("pull").about("Pull repositories and update the local cache"),
        )
        .subcommand(
            SubCommand::with_name("install")
                .alias("add")
                .about("Download and install the given packages [alias: add]")
                .arg(
                    Arg::with_name("PACKAGE")
                        .help("Packages to install")
                        .multiple(true)
                        .required(true),
                )
        )
        .subcommand(
            SubCommand::with_name("upgrade")
                .alias("update")
                .about("Upgrade all installed packages [alias: update]")
        )
        .subcommand(
            SubCommand::with_name("uninstall")
                .alias("remove")
                .about("Uninstall the given packages [alias: remove]")
                .arg(
                    Arg::with_name("PACKAGE")
                        .help("Packages to uninstall")
                        .multiple(true)
                        .required(true),
                )
        )
        .subcommand(
            SubCommand::with_name("reinstall")
                .about("Reinstall an already-installed package")
                .arg(
                    Arg::with_name("PACKAGE")
                        .help("Packages to reinstall")
                        .multiple(true)
                        .required(true),
                )
        )
        .subcommand(
            SubCommand::with_name("list")
                .about("List installed packages")
                .arg(
                    Arg::with_name("with-deps")
                        .long("with-deps")
                        .help("Include the dependencies of installed packages")
                )
        )
        .get_matches();

    let result: Result<(), failure::Error> = try {
        let mut config = config::Config::load_from(matches.value_of("config").unwrap())?;

        if let Some(chroot_path) = matches.value_of("chroot") {
            *config.paths_mut() = config.paths().chroot(chroot_path);
        }

        match matches.subcommand() {
            ("pull", _) => commands::pull(&config),
            ("install", Some(matches)) => commands::install(&config, &matches),
            ("upgrade", Some(matches)) => commands::upgrade(&config, &matches),
            ("uninstall", Some(matches)) => commands::uninstall(&config, &matches),
            ("reinstall", Some(matches)) => commands::reinstall(&config, &matches),
            ("list", Some(matches)) => commands::list(&config, &matches),
            _ => unimplemented!(),
        }?;
    };

    if let Err(e) = result {
        use std::process::exit;

        let fail = e.as_fail();
        eprint!("error: {}", fail);
        for cause in fail.iter_causes() {
            eprint!(": {}", cause);
        }
        eprintln!();

        exit(1);
    }
}
