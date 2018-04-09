//! Functions to execute the `pull` operation.

use libnest::config::Config;

use progressbar::ProgressBar;

/// Pulls all repository
///
/// This will go through all repositories, and for each one of them, go through all mirrors until the pull is
/// complete.
pub fn pull(config: &Config) {
    for repo in config.repositories() {
        for mirror in repo.mirrors() {
            let mut pb = ProgressBar::new(String::from("pull"));
            pb.set_target(repo.name());

            let r = repo.pull(config, mirror, |cur: f64, max: f64| {
                pb.set_max(max as usize);
                pb.update(cur as usize);
                true
            });

            pb.finish(&r);

            match r {
                Ok(_) => break,
                Err(e) => eprintln!("{}: can't pull \"{}\": {}.", red!("error:"), repo.name(), e),
            }
        }
    }
}
