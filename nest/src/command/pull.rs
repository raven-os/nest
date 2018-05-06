//! Functions to execute the `pull` operation.

use failure::{Error, Fail};
use libnest::config::Config;

use error::{RepositoryError, RepositoryErrorKind};
use progress::Progress;
use progressbar::ProgressBar;

/// Pulls all repositories.
///
/// This will go through all repositories, and for each one of them, go through all mirrors until the pull is
/// complete.
pub fn pull(config: &Config) -> Result<(), Error> {
    let mut progress = Progress::new(config.repositories().len());

    for repo in config.repositories().iter() {
        let any = repo.mirrors().iter().any(|mirror| {
            let mut pb = ProgressBar::new(String::from("pull"));
            pb.set_target(format!("({}) {}", progress, repo.name()));

            let res = repo.pull(config, mirror, |cur: f64, max: f64| {
                pb.set_max(max as usize);
                pb.update(cur as usize);
                true
            });

            pb.finish(&res);

            // Print error and continue
            res.is_ok()
        });

        // Throw error if all mirrors are down
        if !any {
            Err(
                Into::<RepositoryError>::into(RepositoryErrorKind::AllMirrorDown)
                    .context(purple!(repo.name())),
            )?;
        }
        progress.next();
    }
    Ok(())
}
