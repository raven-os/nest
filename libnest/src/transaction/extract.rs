use std::fs;
use std::io::{Seek, SeekFrom};
use std::path::Path;

use flate2::read::GzDecoder;
use tar::Archive;

use crate::cache::installed::log::{FileLogEntry, Log};
use crate::chroot::Chroot;
use crate::config::Config;
use crate::lock_file::LockFileOwnership;
use crate::package::{Kind, NPFExplorer, PackageID};

use super::{InstallError, InstallErrorKind::*};

/// Extract the package from a given [`NPFExplorer`] as a given [`PackageID`]
pub(crate) fn extract_package(
    config: &Config,
    lock_ownership: &LockFileOwnership,
    npf_explorer: NPFExplorer,
    target_id: &PackageID,
) -> Result<(), InstallError> {
    let instructions_handle = npf_explorer
        .load_instructions()
        .map_err(|_| InvalidPackageFile)?;

    if let Some(executor) = &instructions_handle {
        executor
            .execute_before_install(config.paths().root())
            .map_err(PreInstallInstructionsFailure)?;
    }

    if npf_explorer.manifest().kind() == Kind::Effective {
        let tarball_handle = npf_explorer
            .open_data()
            .map_err(|_| InvalidPackageFile)?
            .unwrap();

        let mut tarball = tarball_handle.file();
        let mut archive = Archive::new(GzDecoder::new(tarball));
        let mut files = Vec::new();

        // List all the files in the archive and check whether they already exist
        for entry in archive.entries().map_err(|_| InvalidPackageData)? {
            let entry = entry.map_err(|_| InvalidPackageData)?;
            let entry_path = entry.path().map_err(|_| InvalidPackageData)?;
            let entry_type = entry.header().entry_type();

            let abs_path = Path::new("/").with_content(&entry_path);
            let rel_path = config.paths().root().with_content(&entry_path);

            // Check whether the target file exists and retrieve its metadata (without following any symlink)
            if let Ok(metadata) = fs::symlink_metadata(&rel_path) {
                match (entry_type.is_dir(), metadata.file_type().is_dir()) {
                    // Both files are directories, there is no conflict
                    (true, true) => (),

                    // The file to extract is a directory, the existing file is a symlink, check if it resolves to a directory
                    (true, false) if metadata.file_type().is_symlink() => {
                        if let Ok(metadata) = fs::metadata(&rel_path) {
                            if !metadata.is_dir() {
                                return Err(FileAlreadyExists(abs_path).into());
                            }
                        }
                    }

                    // Otherwise, there are conflicting files, and an error is returned
                    _ => return Err(FileAlreadyExists(abs_path).into()),
                }
            }
            files.push(FileLogEntry::new(abs_path.to_path_buf(), entry_type.into()));
        }

        // Log each file to install to the log file
        config
            .installed_packages_cache(lock_ownership)
            .save_package_log(target_id, &Log::new(files))
            .map_err(LogCreationError)?;

        // Extract the tarball in the root folder
        let res: Result<_, std::io::Error> = try {
            tarball.seek(SeekFrom::Start(0))?;
            let mut archive = Archive::new(GzDecoder::new(tarball));
            for entry in archive.entries()? {
                entry?.unpack_in(config.paths().root())?;
            }
        };
        res.map_err(ExtractError)?;
    }

    if let Some(executor) = &instructions_handle {
        executor
            .execute_after_install(config.paths().root())
            .map_err(PostInstallInstructionsFailure)?;
    }

    Ok(())
}
