//! Functions to execute the `download` operation

use std::fs;

use clap::ArgMatches;
use failure::{Error, Fail, ResultExt};
use libnest::config::Config;
use libnest::package::Package;

use error::{RepositoryError, RepositoryErrorKind};
use progress::Progress;
use progressbar::ProgressBar;
use query;

/// Downoads the given package
///
/// This function will draw a progress bar on the user's output *and* print download errors (if any).
/// It will return `Ok` if the download succeeded with any of the mirror, or `Err` otherwise.
pub fn download_package(
    config: &Config,
    progress: &Progress,
    target: &Package,
) -> Result<(), Error> {
    let target_path = target.data_path(config);
    let repo = target.repository();

    let any = repo.mirrors().iter().any(|mirror| {
        let mut pb = ProgressBar::new(String::from("download"));
        pb.set_target(format!(
            "({}) {}",
            progress,
            target.manifest().metadata().name()
        ));

        let res = repo.download(
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

        pb.finish(&res);
        res.is_ok()
    });
    // Throw error if all mirrors are down
    if !any {
        Err(
            Into::<RepositoryError>::into(RepositoryErrorKind::AllMirrorDown)
                .context(purple!(repo.name())),
        )?;
    }
    Ok(())
}

/// Downloads all the given packages
///
/// This will go through all targets, check that they exist, and download them.
pub fn download(config: &Config, matches: &ArgMatches) -> Result<(), Error> {
    let mut config = (*config).clone(); // Clone config for internal modifications

    if let Some(path) = matches.value_of("download-dir") {
        use std::path::PathBuf;
        *config.download_path_mut() = PathBuf::from(path);
    }

    // Retrieve targets
    let targets = query::packages(&config, &matches.values_of_lossy("PACKAGE").unwrap())?;
    let mut progress = Progress::new(targets.len());

    // Iterate through all targets
    for target in &targets {
        // Create destination folder
        if let Some(path) = target.data_path(&config).parent() {
            fs::create_dir_all(path).context(path.display().to_string())?;
        }

        // Download the package
        download_package(&config, &progress, target)?;

        // Go to the next one
        progress.next();
    }
    Ok(())
}
