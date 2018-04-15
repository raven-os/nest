//! Functions to execute the `pull` operation.

use libnest::config::Config;

use progressbar::ProgressBar;

/// Pulls all repository
///
/// This will go through all repositories, and for each one of them, go through all mirrors until the pull is
/// complete.
pub fn pull(config: &Config) {
    let len = config.repositories().len();
    for (i, repo) in config.repositories().iter().enumerate() {
        for mirror in repo.mirrors()
        {
            let mut pb = ProgressBar::new(String::from("pull"));
            pb.set_target(format!("({}/{}) {}", i + 1, len, repo.name()));

            let r = repo.pull(config, mirror, |cur: f64, max: f64| {
                pb.set_max(max as usize);
                pb.update(cur as usize);
                true
            });

            pb.finish(&r);

            match r {
                Ok(_) => break,
                Err(e) => eprintln!("{}: couldn't pull \"{}\": {}.", red!("error"), repo.name(), e),
            }
        }
    }
}
