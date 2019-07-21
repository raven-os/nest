use std::fs::{self, File};
use std::io::{Seek, Write};

use failure::{Error, ResultExt};

use crate::config::Config;
use crate::package::PackageID;

/// Structure representing a package download
#[derive(Clone, Hash, Debug)]
pub struct PackageDownload(PackageID);

impl PackageDownload {
    /// Create a download from a [`PackageID`]
    pub fn from(target: PackageID) -> Self {
        Self(target)
    }

    /// Retrieves the target package for this download
    pub fn target(&self) -> &PackageID {
        &self.0
    }

    /// Creates the download file and returns a handle to it
    pub fn create_download_file(&self, config: &Config) -> Result<(impl Write + Seek), Error> {
        // Create target folder and destination file
        let npf_path = config
            .paths()
            .downloaded()
            .join(self.target().repository().as_str())
            .join(self.target().category().as_str())
            .join(self.target().name().as_str());
        fs::create_dir_all(&npf_path).with_context(|_| npf_path.display().to_string())?;
        let tarball_path = npf_path.join(format!(
            "{}-{}.nest",
            self.target().name(),
            self.target().version()
        ));

        // Open the destination file and return it as the writer handle
        let tarball_file =
            File::create(&tarball_path).with_context(|_| tarball_path.display().to_string())?;
        Ok(tarball_file)
    }
}
