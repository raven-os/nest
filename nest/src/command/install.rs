use std::process;

use clap::ArgMatches;
use libnest::config::Config;
use progressbar::ProgressBar;

use query;

/// Installs all the given packages
///
/// This will go through all targets, checks that the package exists, resolves the dependency graph,
/// ensures the installation will not break anything or delete any file and installs the package.
pub fn install(config: &Config, matches: &ArgMatches) {
    let mut targets = Vec::new();

    // Search cache for existing packages, ensuring the target exists.
    for target in matches.values_of("PACKAGE").unwrap() {
        match query::cache(config, target) {
            Some(query) => {
                match query.perform() {
                    Ok(mut packages) => {
                        match packages.len() {
                            0 => {
                                eprintln!("{} couldn't find a package with name \"{}\". Try \"nest search\" to look for an existing package.",
                                red!("error:"),
                                purple!(target),
                            );
                                process::exit(1);
                            }
                            1 => targets.append(&mut packages),
                            len => {
                                eprintln!("{} found {} packages with name \"{}\". Please be more explicit.", red!("error:"), len, purple!(target));
                                for (i, package) in packages.iter().enumerate() {
                                    if i == 10 && len > 10 {
                                        println!("and {} more...", len - 10);
                                        break;
                                    }
                                    println!("\t- {}", package);
                                }
                                process::exit(1);
                            }
                        }
                        for package in packages {
                            println!(
                                "\t{}::{}/{}",
                                package.repository().name(),
                                package.content().category(),
                                package.content().name()
                            );
                        }
                    }
                    Err(e) => {
                        eprintln!("{} couldn't go through cache: {}.", red!("error:"), e);
                        process::exit(1);
                    }
                }
            }
            None => {
                eprintln!(
                    "{} target \"{}\" isn't a valid target name.",
                    red!("error:"),
                    purple!(target)
                );
            }
        }
    }

    // Download packages
    let len = targets.len();
    for (i, target) in targets.iter().enumerate() {
        let repo = target.repository();
        for mirror in repo.mirrors() {
            let target_name = format!("({}/{}) {}", i + 1, len, target.content().name());
            let mut pb = ProgressBar::new(String::from("download"));
            pb.set_target(&target_name);

            let r = repo.download(config, mirror, target.content(), |cur: f64, max: f64| {
                pb.set_max(max as usize);
                pb.update(cur as usize);
                true
            });

            pb.finish(&r);

            match r {
                Ok(_) => break,
                Err(e) => eprintln!(
                    "{}: failed to download \"{}\": {}.",
                    red!("error:"),
                    target,
                    e
                ),
            }
        }
    }

    // Install packages

    // TODO
}
