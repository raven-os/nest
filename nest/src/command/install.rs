//! Functions to execute the `install` operation.

use std::fs;
use std::path::PathBuf;

use clap::ArgMatches;
use failure::{Error, ResultExt};
use libnest::config::Config;
use libnest::package::Package;
use libnest::system::installer::InstallState;
use libnest::system::System;

use command::download::download_package;
use progress::Progress;
use progressbar::ProgressBar;
use query;

/// Installs the given package.
///
/// This function will draw a progress bar on the user's output.
/// It will return `Ok` if the install succeeds, or `Err` otherwise.
pub fn install_package(
    config: &Config,
    sys: &System,
    progress: &Progress,
    target: &Package,
) -> Result<(), Error> {
    let mut old_state = InstallState::Waiting;
    let mut pb = ProgressBar::new(old_state.to_string());
    pb.set_target(format!(
        "({}) {}",
        progress,
        target.manifest().metadata().name()
    ));

    let res = sys.installer(config, &target.data_path(config), &target)
        .perform(|state, progression| {
            // Update the action only when it's significant
            if old_state != state {
                pb.set_action(state.to_string());
                old_state = state;
            }
            if let Some((cur, max)) = progression {
                pb.set_max(max);
                pb.update(cur);
            } else {
                pb.set_max(0);
                pb.update(0);
            }
        });
    pb.finish(&res);
    res
}

/// Installs all the given packages.
///
/// This will go through all targets, check that they exist, resolve the dependency graph,
/// download the packages, ensure the installation will not break anything nor delete any file
/// and then install the package.
pub fn install(config: &Config, matches: &ArgMatches) -> Result<(), Error> {
    // Retrieve targets
    let targets = query::packages(config, &matches.values_of_lossy("PACKAGE").unwrap())?;
    let mut progress = Progress::new(targets.len());

    // targeted system
    let mut sys = System::current();
    if let Some(path) = matches.value_of("install-dir") {
        *sys.install_path_mut() = PathBuf::from(path);
    }

    // Iterate through all targets
    for target in &targets {
        // Create destination folder
        if let Some(path) = target.data_path(config).parent() {
            fs::create_dir_all(path).context(path.display().to_string())?;
        }

        // Download and install the package
        download_package(config, &progress, target)?;
        install_package(config, &sys, &progress, target).context(format!(
            "the installation of \"{}\" failed",
            purple!(target)
        ))?;
        progress.next();
    }
    Ok(())
}
