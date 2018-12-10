use clap::{crate_authors, crate_name, crate_version, App, AppSettings, Arg, SubCommand};

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
        .get_matches();

    let result = match matches.subcommand() {
        ("pull", _) => commands::pull(),
        ("install", _) => commands::install(),
        ("upgrade", _) => commands::upgrade(),
        ("uninstall", _) => commands::uninstall(),
        _ => unimplemented!(),
    };

    if let Err(_) = result {
        use std::process::exit;

        eprintln!("Whoopsie");
        exit(1);
    }
}
