//! Functions to execute the `install` operation.

use std::fs;
use std::process;

use clap::ArgMatches;
use libnest::config::Config;
use libnest::system::System;
use progressbar::ProgressBar;

use query;

/// Installs all the given packages
///
/// This will go through all targets, checks that the package exists, resolves the dependency graph,
/// ensures the installation will not break anything or delete any file and installs the package.
pub fn install(config: &Config, matches: &ArgMatches) {
    let sys = System::current();
    let mut targets = Vec::new();

    // Search in the cache for manifests, ensuring the target exists.
    for target in matches.values_of("PACKAGE").unwrap() {
        match query::cache(config, target) {
            Some(query) => {
                match query.perform() {
                    Ok(mut manifests) => {
                        match manifests.len() {
                            0 => {
                                eprintln!("{}: couldn't find a package with name \"{}\". Try \"{}\" to look for an existing package.",
                                    red!("error"),
                                    purple!(target),
                                    purple!("nest search")
                                );
                                process::exit(1);
                            }
                            1 => targets.append(&mut manifests),
                            len => {
                                eprintln!("{}: found {} packages with name \"{}\". Please be more explicit.", red!("error"), len, purple!(target));
                                for (i, package) in manifests.iter().enumerate() {
                                    eprintln!("\t- {}", package);
                                    if i == 9 && len > 10 {
                                        eprintln!("and {} more...", len - 10);
                                        break;
                                    }
                                }
                                process::exit(1);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("{}: couldn't go through cache: {}.", red!("error"), e);
                        process::exit(1);
                    }
                }
            }
            None => {
                eprintln!(
                    "{}: target \"{}\" isn't a valid target name.",
                    red!("error"),
                    purple!(target),
                );
                process::exit(1);
            }
        }
    }

    // Download and install packages
    let len = targets.len();
    for (i, target) in targets.iter().enumerate() {
        let repo = target.repository();
        for mirror in repo.mirrors() {
            let target_path = target.data_path(config);

            if let Some(_) = target_path.parent().map(|x| fs::create_dir_all(x).ok()) {
                let mut pb = ProgressBar::new(String::from("download"));
                pb.set_target(format!(
                    "({}/{}) {}",
                    i + 1,
                    len,
                    target.manifest().metadatas().name()
                ));

                let r = repo.download(
                    config,
                    mirror,
                    target.manifest(),
                    &target_path,
                    |cur: f64, max: f64| {
                        pb.set_max(max as usize);
                        pb.update(cur as usize);
                        true
                    },
                );

                pb.finish(&r);

                match r {
                    Ok(_) => {
                        sys.install(&target_path);
                        break;
                    }
                    Err(e) => eprintln!(
                        "{}: failed to download \"{}\": {}.",
                        red!("error"),
                        purple!(target),
                        e
                    ),
                }
            } else {
                eprintln!(
                    "{}: couldn't create \"{}\".",
                    red!("error"),
                    purple!(target_path.display()),
                );
                process::exit(1);
            }
        }
    }
}
