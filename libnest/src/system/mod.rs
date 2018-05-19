//! The system, that is the targeted machine.
//!
//! This module provides interfaces to talk to the targeted system: the one we want to operate
//! on. Usually, the operations fall in one of the three below:
//!  * Installing/Upgrading softwares.
//!  * Removing softwares.
//!  * Looking for informations, like the installed softwares or the architecture.
//!
//! Usually, the targeted system is the one running nest, but in some special cases (like installing
//! packages on a remote installation), it might not be the case.

pub mod arch;
pub mod installer;

use std::path::{Path, PathBuf};

use config::Config;
use package::Package;

use self::arch::Arch;
use self::installer::Installer;

/// The targeted system.
///
/// It represents the whole targeted system, and let interact with it, like installing a new package or
/// looking for the ones that are already installed.
///
/// # Examples
///
/// ```
/// # extern crate libnest;
/// use libnest::system::System;
///
/// let system = System::current();
/// println!("Running on {}", system.arch());
/// ```
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct System {
    install_path: PathBuf,
}

impl System {
    /// Returns an instance of the current system.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate libnest;
    /// use libnest::system::System;
    ///
    /// let system = System::current();
    /// ```
    #[inline]
    pub fn current() -> System {
        System {
            install_path: PathBuf::from("/"),
        }
    }

    /// Returns the architecture of the targeted system.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate libnest;
    /// use libnest::system::System;
    /// use libnest::system::arch::Arch;
    ///
    /// let system = System::current();
    /// assert_eq!(system.arch(), Arch::x86_64);
    /// ```
    #[inline]
    pub fn arch(&self) -> Arch {
        if cfg!(target_arch = "x86_64") {
            Arch::x86_64
        } else if cfg!(target_arch = "mips") {
            Arch::Mips
        } else if cfg!(target_arch = "powerpc") {
            Arch::PowerPc
        } else if cfg!(target_arch = "powerpc64") {
            Arch::PowerPc64
        } else if cfg!(target_arch = "arm") {
            Arch::Arm
        } else if cfg!(target_arch = "aarch64") {
            Arch::AArch64
        } else {
            panic!("Unknown target architecture")
        }
    }

    /// Returns a reference to the installation path for the targeted system, usually the root folder.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate libnest;
    /// use std::path::Path;
    /// use libnest::system::System;
    ///
    /// let system = System::current();
    /// assert_eq!(system.install_path(), Path::new("/"));
    /// ```
    pub fn install_path(&self) -> &Path {
        &self.install_path
    }

    /// Returns a mutable reference to the installation path for the targeted system, usually the root folder.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate libnest;
    /// use std::path::{PathBuf, Path};
    /// use libnest::system::System;
    ///
    /// let mut system = System::current();
    /// *system.install_path_mut() = PathBuf::from("/mnt/");
    /// assert_eq!(system.install_path(), Path::new("/mnt/"));
    /// ```
    pub fn install_path_mut(&mut self) -> &mut PathBuf {
        &mut self.install_path
    }

    /// Installs the package located at the given path, following the given manifest.
    pub fn installer<'a, 'b, 'c, 'd>(
        &'a self,
        config: &'b Config,
        path: &'c Path,
        package: &'d Package<'d>,
    ) -> Installer<'a, 'b, 'c, 'd> {
        Installer::from(self, config, path, package)
    }
}
