use lazy_static::lazy_static;
use serde_derive::{Deserialize, Serialize};

use std::path::{Path, PathBuf};

lazy_static! {
    static ref NEST_PATH_ROOT: &'static Path = Path::new("/");
    static ref NEST_PATH_CACHE: &'static Path = Path::new("/var/nest/available/");
    static ref NEST_PATH_DOWNLOADED: &'static Path = Path::new("/var/nest/downloaded/");
    static ref NEST_PATH_INSTALLED: &'static Path = Path::new("/var/nest/installed/");
    static ref NEST_PATH_DEPGRAPH: &'static Path = Path::new("/var/nest/depgraph");
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
        }
    }

    /// Changes all config paths to make them relative to the given root path.
    #[inline]
    pub fn chroot<P: AsRef<Path>>(&mut self, root: P) {
        use crate::chroot::Chroot;

        self.root = self.root.with_root(root.as_ref());
        self.available = self.available.with_root(root.as_ref());
        self.downloaded = self.downloaded.with_root(root.as_ref());
        self.installed = self.installed.with_root(root.as_ref());
        self.depgraph = self.depgraph.with_root(root.as_ref());
    }

    /// Returns a reference to the root path where packages should be installed.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # extern crate libnest;
    /// # extern crate failure;
    /// # fn main() -> Result<(), failure::Error> {
    /// use std::path::{Path, PathBuf};
    /// use libnest::config::Config;
    ///
    /// let mut config = Config::load()?;
    /// assert_eq!(config.paths().root(), Path::new("/"));
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
    /// use libnest::config::Config;
    ///
    /// let mut config = Config::load()?;
    /// *config.paths_mut().root_mut() = PathBuf::from("/mnt");
    /// assert_eq!(config.paths().root(), Path::new("/mnt"));
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
    /// use std::path::{Path, PathBuf};
    /// use libnest::config::Config;
    ///
    /// let mut config = Config::load()?;
    /// assert_eq!(config.paths().available(), Path::new("/var/nest/available"));
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
    /// use libnest::config::Config;
    ///
    /// let mut config = Config::load()?;
    /// *config.paths_mut().available_mut() = PathBuf::from("/tmp/available");
    /// assert_eq!(config.paths().available(), Path::new("/tmp/available"));
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
    /// use std::path::{Path, PathBuf};
    /// use libnest::config::Config;
    ///
    /// let mut config = Config::load()?;
    /// assert_eq!(config.paths().downloaded(), Path::new("/var/nest/downloaded"));
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
    /// use libnest::config::Config;
    ///
    /// let mut config = Config::load()?;
    /// *config.paths_mut().downloaded_mut() = PathBuf::from("/tmp/downloaded");
    /// assert_eq!(config.paths().downloaded(), Path::new("/tmp/downloaded"));
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
    /// use std::path::{Path, PathBuf};
    /// use libnest::config::Config;
    ///
    /// let mut config = Config::load()?;
    /// assert_eq!(config.paths().installed(), Path::new("/var/nest/installed"));
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
    /// use libnest::config::Config;
    ///
    /// let mut config = Config::load()?;
    /// *config.paths_mut().installed_mut() = PathBuf::from("/tmp/installed");
    /// assert_eq!(config.paths().installed(), Path::new("/tmp/installed"));
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
    /// use std::path::{Path, PathBuf};
    /// use libnest::config::Config;
    ///
    /// let mut config = Config::load()?;
    /// assert_eq!(config.paths().depgraph(), Path::new("/var/nest/depgraph"));
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
    /// use libnest::config::Config;
    ///
    /// let mut config = Config::load()?;
    /// *config.paths_mut().depgraph_mut() = PathBuf::from("/tmp/depgraph");
    /// assert_eq!(config.paths().depgraph(), Path::new("/tmp/depgraph"));
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn depgraph_mut(&mut self) -> &mut PathBuf {
        &mut self.depgraph
    }
}

impl Default for ConfigPaths {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
