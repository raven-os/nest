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
            SubCommand::with_name("group")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .about("Operate on groups")
                .subcommand(
                    SubCommand::with_name("add")
                        .about("Create new groups")
                        .arg(
                            Arg::with_name("GROUP")
                                .help("Groups to create")
                                .multiple(true)
                                .required(true)
                        )
                        .arg(
                            Arg::with_name("PARENT")
                                .long("parent")
                                .help("Parent group of the groups to create")
                                .takes_value(true)
                                .default_value("@root")
                        )
                )
                .subcommand(
                    SubCommand::with_name("remove")
                        .about("Remove existing groups")
                        .arg(
                            Arg::with_name("GROUP")
                                .help("Groups to remove")
                                .multiple(true)
                                .required(true),
                        )
                )
                .subcommand(
                    SubCommand::with_name("list")
                        .about("List existing groups")
                )
        )
        .subcommand(
            SubCommand::with_name("requirement")
                .setting(AppSettings::SubcommandRequiredElseHelp)
                .about("Operate on requirements")
                .subcommand(
                    SubCommand::with_name("add")
                        .about("Add new requirements")
                        .arg(
                            Arg::with_name("PACKAGE")
                                .help("Requirements to add")
                                .multiple(true)
                                .required(true),
                        )
                        .arg(
                            Arg::with_name("PARENT")
                                .long("parent")
                                .help("Parent group of the requirements to add")
                                .takes_value(true)
                                .default_value("@root")
                        )
                )
                .subcommand(
                    SubCommand::with_name("remove")
                        .about("Remove existing requirements")
                        .arg(
                            Arg::with_name("PACKAGE")
                                .help("Requirements to remove")
                                .multiple(true)
                                .required(true),
                        )
                        .arg(
                            Arg::with_name("PARENT")
                                .long("parent")
                                .help("Parent group of the requirements to add")
                                .takes_value(true)
                                .default_value("@root")
                        )
                )
        )
        .subcommand(
            SubCommand::with_name("merge")
                .about("Merge the scratch dependency graph with the regular dependency graph")
        )
        .get_matches();

    let result: Result<(), failure::Error> = try {
        let mut config = config::Config::load()?;

        if let Some(chroot_path) = matches.value_of("chroot") {
            *config.paths_mut() = config.paths().chroot(chroot_path);
        }

        match matches.subcommand() {
            ("pull", _) => commands::pull(&config),
            ("install", Some(matches)) => commands::install(&config, &matches),
            ("upgrade", _) => commands::upgrade(&config),
            ("uninstall", Some(matches)) => commands::uninstall(&config, &matches),
            ("group", Some(sub_matches)) => match sub_matches.subcommand() {
                ("add", Some(cmd_matches)) => commands::group_add(
                    &config,
                    cmd_matches.value_of("PARENT").unwrap(),
                    &cmd_matches,
                ),
                ("remove", Some(cmd_matches)) => commands::group_remove(&config, &cmd_matches),
                ("list", _) => commands::group_list(&config),
                _ => unimplemented!(),
            },
            ("requirement", Some(sub_matches)) => match sub_matches.subcommand() {
                ("add", Some(cmd_matches)) => commands::requirement_add(
                    &config,
                    cmd_matches.value_of("PARENT").unwrap(),
                    &cmd_matches,
                ),
                ("remove", Some(cmd_matches)) => commands::requirement_remove(
                    &config,
                    cmd_matches.value_of("PARENT").unwrap(),
                    &cmd_matches,
                ),
                _ => unimplemented!(),
            },
            ("merge", _) => commands::merge(&config),
            _ => unimplemented!(),
        }?;
    };

    if let Err(e) = result {
        use std::process::exit;

        let fail: &failure::Fail = e.as_fail();
        eprint!("error: {}", fail);
        for cause in fail.iter_causes() {
            eprint!(": {}", cause);
        }
        eprintln!();

        exit(1);
    }
}
