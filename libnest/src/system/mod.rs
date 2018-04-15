//! The system, that is the targeted machine.
//!
//! This module provides interfaces to talk to the targed system: the one we want to operate
//! on. Usually, the operations fall in one of the three below:
//!  * Installing/Upgrading softwares
//!  * Removing softwares
//!  * Looking for informations, like the installed softwares or the architecture.
//!
//! Usually, the targeted system is the one running nest, but in some special cases (like installing
//! packages on a remote installation), it might not be the case.

pub mod arch;

use std::path::Path;

use self::arch::Arch;

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
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct System;

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
        System {}
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

    /// Installs the package located at the given path.
    pub fn install(&self, path: &Path) {
        println!("Installing package located at {}", path.display());
    }
}
