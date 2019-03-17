use lazy_static::lazy_static;
use serde_derive::{Deserialize, Serialize};

use std::path::{Path, PathBuf};

lazy_static! {
    static ref NEST_PATH_ROOT: &'static Path = Path::new("/");
    static ref NEST_PATH_CACHE: &'static Path = Path::new("/var/nest/available/");
    static ref NEST_PATH_DOWNLOADED: &'static Path = Path::new("/var/nest/downloaded/");
    static ref NEST_PATH_INSTALLED: &'static Path = Path::new("/var/nest/installed/");
    static ref NEST_PATH_DEPGRAPH: &'static Path = Path::new("/var/nest/depgraph");
    static ref NEST_PATH_SCRATCH_DEPGRAPH: &'static Path = Path::new("/var/nest/scratch_depgraph");
    static ref NEST_PATH_LOCKFILE: &'static Path = Path::new("/var/lock/nest.lock");
}

/// A structure holding all important paths for libnest. It's a sub member of [`Config`][1].
///
/// [1]: struct.Config.html
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug)]
#[serde(default)]
pub struct ConfigPaths {
    root: PathBuf,
    available: PathBuf,
    downloaded: PathBuf,
    installed: PathBuf,
    depgraph: PathBuf,
    scratch_depgraph: PathBuf,
    lockfile_path: PathBuf,
}

impl ConfigPaths {
    #[inline]
    pub(crate) fn new() -> ConfigPaths {
        ConfigPaths {
            root: PathBuf::from(*NEST_PATH_ROOT),
            available: PathBuf::from(*NEST_PATH_CACHE),
            downloaded: PathBuf::from(*NEST_PATH_DOWNLOADED),
            installed: PathBuf::from(*NEST_PATH_INSTALLED),
            depgraph: PathBuf::from(*NEST_PATH_DEPGRAPH),
            scratch_depgraph: PathBuf::from(*NEST_PATH_SCRATCH_DEPGRAPH),
            lockfile_path: PathBuf::from(*NEST_PATH_LOCKFILE),
        }
    }

    /// Changes all config paths to make them relative to the given root path.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate libnest;
    /// fn main() -> Result<(), failure::Error> {
    /// use libnest::config::ConfigPaths;
    /// use std::path::Path;
    ///
    /// let chroot_path = "/chroot/";
    /// let default_paths = ConfigPaths::default();
    /// let paths = default_paths.chroot(chroot_path);
    /// assert_eq!(paths.root(), Path::new("/chroot/"));
    /// assert_eq!(paths.available(), Path::new("/chroot/var/nest/available"));
    /// assert_eq!(paths.downloaded(), Path::new("/chroot/var/nest/downloaded"));
    /// assert_eq!(paths.installed(), Path::new("/chroot/var/nest/installed"));
    /// assert_eq!(paths.depgraph(), Path::new("/chroot/var/nest/depgraph"));
    /// assert_eq!(paths.lock_file(), Path::new("/chroot/var/lock/nest.lock"));
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn chroot<P: AsRef<Path>>(&self, root: P) -> ConfigPaths {
        use crate::chroot::Chroot;

        ConfigPaths {
            root: self.root.with_root(root.as_ref()),
            available: self.available.with_root(root.as_ref()),
            downloaded: self.downloaded.with_root(root.as_ref()),
            installed: self.installed.with_root(root.as_ref()),
            depgraph: self.depgraph.with_root(root.as_ref()),
            scratch_depgraph: self.scratch_depgraph.with_root(root.as_ref()),
            lockfile_path: self.lockfile_path.with_root(root.as_ref()),
        }
    }

    /// Returns a reference to the root path where packages should be installed.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # extern crate libnest;
    /// # extern crate failure;
    /// # fn main() -> Result<(), failure::Error> {
    /// use std::path::Path;
    /// use libnest::config::ConfigPaths;
    ///
    /// let paths = ConfigPaths::default();
    /// assert_eq!(paths.root(), Path::new("/"));
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Returns a mutable reference to the root path where packages should be installed.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # extern crate libnest;
    /// # extern crate failure;
    /// # fn main() -> Result<(), failure::Error> {
    /// use std::path::{Path, PathBuf};
    /// use libnest::config::ConfigPaths;
    ///
    /// let mut paths = ConfigPaths::default();
    /// *paths.root_mut() = PathBuf::from("/mnt");
    /// assert_eq!(paths.root(), Path::new("/mnt"));
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn root_mut(&mut self) -> &mut PathBuf {
        &mut self.root
    }

    /// Returns a reference to the path where available packages are cached.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # extern crate libnest;
    /// # extern crate failure;
    /// # fn main() -> Result<(), failure::Error> {
    /// use std::path::Path;
    /// use libnest::config::ConfigPaths;
    ///
    /// let paths = ConfigPaths::default();
    /// assert_eq!(paths.available(), Path::new("/var/nest/available"));
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn available(&self) -> &Path {
        &self.available
    }

    /// Returns a mutable reference to the path where available packages are cached.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # extern crate libnest;
    /// # extern crate failure;
    /// # fn main() -> Result<(), failure::Error> {
    /// use std::path::{Path, PathBuf};
    /// use libnest::config::ConfigPaths;
    ///
    /// let mut paths = ConfigPaths::default();
    /// *paths.available_mut() = PathBuf::from("/tmp/available");
    /// assert_eq!(paths.available(), Path::new("/tmp/available"));
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn available_mut(&mut self) -> &mut PathBuf {
        &mut self.available
    }

    /// Returns a reference to the path where downloaded packages are stored, before installation.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # extern crate libnest;
    /// # extern crate failure;
    /// # fn main() -> Result<(), failure::Error> {
    /// use std::path::Path;
    /// use libnest::config::ConfigPaths;
    ///
    /// let paths = ConfigPaths::default();
    /// assert_eq!(paths.downloaded(), Path::new("/var/nest/downloaded"));
    /// # Ok(())
    /// # }
    /// ```
    pub fn downloaded(&self) -> &Path {
        &self.downloaded
    }

    /// Returns a mutable reference to the path where downloaded packages are stored, before installation.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # extern crate libnest;
    /// # extern crate failure;
    /// # fn main() -> Result<(), failure::Error> {
    /// use std::path::{Path, PathBuf};
    /// use libnest::config::ConfigPaths;
    ///
    /// let mut paths = ConfigPaths::default();
    /// *paths.downloaded_mut() = PathBuf::from("/tmp/downloaded");
    /// assert_eq!(paths.downloaded(), Path::new("/tmp/downloaded"));
    /// # Ok(())
    /// # }
    /// ```
    pub fn downloaded_mut(&mut self) -> &mut PathBuf {
        &mut self.downloaded
    }

    /// Returns a reference to the path where installed packaged are logged.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # extern crate libnest;
    /// # extern crate failure;
    /// # fn main() -> Result<(), failure::Error> {
    /// use std::path::Path;
    /// use libnest::config::ConfigPaths;
    ///
    /// let paths = ConfigPaths::default();
    /// assert_eq!(paths.installed(), Path::new("/var/nest/installed"));
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn installed(&self) -> &Path {
        &self.installed
    }

    /// Returns a mutable reference to the path where packages' metadata are cached.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # extern crate libnest;
    /// # extern crate failure;
    /// # fn main() -> Result<(), failure::Error> {
    /// use std::path::{Path, PathBuf};
    /// use libnest::config::ConfigPaths;
    ///
    /// let mut paths = ConfigPaths::default();
    /// *paths.installed_mut() = PathBuf::from("/tmp/installed");
    /// assert_eq!(paths.installed(), Path::new("/tmp/installed"));
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn installed_mut(&mut self) -> &mut PathBuf {
        &mut self.installed
    }

    /// Returns a reference to the file's path where the dependency graph is stored
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # extern crate libnest;
    /// # extern crate failure;
    /// # fn main() -> Result<(), failure::Error> {
    /// use std::path::Path;
    /// use libnest::config::ConfigPaths;
    ///
    /// let paths = ConfigPaths::default();
    /// assert_eq!(paths.depgraph(), Path::new("/var/nest/depgraph"));
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn depgraph(&self) -> &Path {
        &self.depgraph
    }

    /// Returns a mutable reference to the file's path where the dependency graph is stored
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # extern crate libnest;
    /// # extern crate failure;
    /// # fn main() -> Result<(), failure::Error> {
    /// use std::path::{Path, PathBuf};
    /// use libnest::config::ConfigPaths;
    ///
    /// let mut paths = ConfigPaths::default();
    /// *paths.depgraph_mut() = PathBuf::from("/tmp/depgraph");
    /// assert_eq!(paths.depgraph(), Path::new("/tmp/depgraph"));
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn depgraph_mut(&mut self) -> &mut PathBuf {
        &mut self.depgraph
    }

    /// Returns a reference to the file's path where the scratch dependency graph is stored
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # extern crate libnest;
    /// # extern crate failure;
    /// # fn main() -> Result<(), failure::Error> {
    /// use std::path::Path;
    /// use libnest::config::ConfigPaths;
    ///
    /// let paths = ConfigPaths::default();
    /// assert_eq!(paths.scratch_depgraph(), Path::new("/var/nest/scratch_depgraph"));
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn scratch_depgraph(&self) -> &Path {
        &self.scratch_depgraph
    }

    /// Returns a mutable reference to the file's path where the scratch dependency graph is stored
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # extern crate libnest;
    /// # extern crate failure;
    /// # fn main() -> Result<(), failure::Error> {
    /// use std::path::{Path, PathBuf};
    /// use libnest::config::ConfigPaths;
    ///
    /// let mut paths = ConfigPaths::default();
    /// *paths.scratch_depgraph_mut() = PathBuf::from("/tmp/scratch_depgraph");
    /// assert_eq!(paths.scratch_depgraph(), Path::new("/tmp/scratch_depgraph"));
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn scratch_depgraph_mut(&mut self) -> &mut PathBuf {
        &mut self.scratch_depgraph
    }

    /// Returns a reference to the file's path where the lock file is stored
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # extern crate libnest;
    /// # extern crate failure;
    /// # fn main() -> Result<(), failure::Error> {
    /// use std::path::Path;
    /// use libnest::config::ConfigPaths;
    ///
    /// let paths = ConfigPaths::default();
    /// assert_eq!(paths.lock_file(), Path::new("/var/lock/nest.lock"));
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn lock_file(&self) -> &Path {
        &self.lockfile_path
    }

    /// Returns a mutable reference to the file's path where the lock file is stored
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # extern crate libnest;
    /// # extern crate failure;
    /// # fn main() -> Result<(), failure::Error> {
    /// use std::path::{Path, PathBuf};
    /// use libnest::config::ConfigPaths;
    ///
    /// let mut paths = ConfigPaths::default();
    /// *paths.lock_file_mut() = PathBuf::from("/tmp/nest.lock");
    /// assert_eq!(paths.lock_file(), Path::new("/tmp/nest.lock"));
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn lock_file_mut(&mut self) -> &mut PathBuf {
        &mut self.lockfile_path
    }
}

impl Default for ConfigPaths {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
