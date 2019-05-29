use std::iter::Iterator;
use std::sync::mpsc::channel;

use failure::{format_err, Error, ResultExt};
use libnest::config::Config;
use libnest::lock_file::LockFileOwnership;
use libnest::transaction::InstallTransaction;
use threadpool::ThreadPool;

use super::download::Download;

pub fn download_package(config: &Config, trans: &InstallTransaction) -> Result<(), Error> {
    // Find the repository hosting the package
    let repo = config
        .repositories()
        .into_iter()
        .find(|repository| repository.name() == **trans.target().repository())
        .ok_or_else(|| {
            format_err!(
                "unable to find repository '{}'",
                trans.target().repository()
            )
        })?;

    // Build the target route
    let target_url = format!(
        "api/p/{}/{}/{}/download",
        trans.target().category(),
        trans.target().name(),
        trans.target().version(),
    );

    // Download the package archive
    let download = Download::from(&target_url);
    download
        .perform_with_mirrors(
            &mut trans.create_download_file(config)?,
            &repo.config().mirrors(),
        )
        .context(format_err!(
            "unable to download package from repository '{}'",
            repo.name()
        ))?;

    Ok(())
}

pub fn download_packages<'a>(
    config: &Config,
    installs: impl Iterator<Item = &'a InstallTransaction>,
) -> Result<(), Error> {
    let pool = ThreadPool::new(num_cpus::get());
    let (sender, receiver) = channel();
    let mut n = 0;

    for install in installs {
        let sender = sender.clone();
        let config = config.clone();
        let install = install.clone();
        pool.execute(move || {
            let result = download_package(&config, &install);
            sender
                .send(result)
                .expect("cannot communicate with main thread");
        });
        n += 1;
    }
    receiver
        .into_iter()
        .take(n)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(())
}

pub fn install_package(
    config: &Config,
    trans: &InstallTransaction,
    ownership: &LockFileOwnership,
) -> Result<(), Error> {
    trans
        .extract(&config, ownership)
        .context(format_err!("unable to extract package"))?;

    println!("Successfully installed {}", trans.target());
    Ok(())
}
