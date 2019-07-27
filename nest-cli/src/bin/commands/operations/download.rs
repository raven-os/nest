use std::io::{Seek, SeekFrom, Write};
use std::iter::Iterator;
use std::sync::mpsc::channel;

use curl::easy::Easy;
use failure::{format_err, Error, ResultExt};
use libnest::config::{Config, MirrorUrl};
use libnest::transaction::PackageDownload;
use threadpool::ThreadPool;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Download<'a> {
    target_route: &'a str,
}

impl<'a> Download<'a> {
    /// Creates a download from a given route
    pub fn from(target_route: &'a str) -> Self {
        Download { target_route }
    }

    /// Performs the download, using any of the specified mirrors
    pub fn perform_with_mirrors<W>(
        &self,
        writer: &mut W,
        mirrors: &[MirrorUrl],
    ) -> Result<(), Error>
    where
        W: Write + Seek,
    {
        let mut curl = Easy::new();
        curl.follow_location(true)?;
        curl.fail_on_error(true)?;
        curl.progress(true)?;

        let succeeded = mirrors.iter().any(|mirror| {
            let res: Result<_, Error> = try {
                // Overwrite any data from a previous failed attempt
                writer.seek(SeekFrom::Start(0))?;

                let url = mirror.join(self.target_route)?;
                curl.url(url.as_str())?;

                let mut transfer = curl.transfer();
                transfer.write_function(|data| Ok(writer.write(data).unwrap_or(0)))?;
                transfer.perform()?;
            };
            res.is_ok()
        });

        if !succeeded {
            Err(format_err!("no working mirror found"))
        } else {
            Ok(())
        }
    }
}

pub fn download_package(config: &Config, package_download: &PackageDownload) -> Result<(), Error> {
    // Find the repository hosting the package
    let repo = config
        .repositories()
        .into_iter()
        .find(|repository| repository.name() == **package_download.target().repository())
        .ok_or_else(|| {
            format_err!(
                "unable to find repository '{}'",
                package_download.target().repository()
            )
        })?;

    // Build the target route
    let target_url = format!(
        "api/p/{}/{}/{}/download",
        package_download.target().category(),
        package_download.target().name(),
        package_download.target().version(),
    );

    // Download the package archive
    let download = Download::from(&target_url);
    download
        .perform_with_mirrors(
            &mut package_download.create_download_file(config)?,
            &repo.config().mirrors(),
        )
        .context(format_err!(
            "unable to download package from repository '{}'",
            repo.name()
        ))?;

    Ok(())
}

pub fn download_packages(
    config: &Config,
    downloads: impl Iterator<Item = PackageDownload>,
) -> Result<(), Error> {
    let pool = ThreadPool::new(num_cpus::get());
    let (sender, receiver) = channel();
    let mut n = 0;

    for download in downloads {
        let sender = sender.clone();
        let config = config.clone();
        pool.execute(move || {
            let result = download_package(&config, &download);
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
