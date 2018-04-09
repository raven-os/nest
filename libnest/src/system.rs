//!
//! The system, that is the targeted machine.
//!

/// The targeted system.
///
/// It represents the whole system, and let interact with it, like installing a new package or
/// looking for the ones that are already installed.
///
/// # Examples
///
/// ```
/// # extern crate libnest;
/// use libnest::system::System;
///
/// let system = System::get();
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct System {}

impl System {
    /// Returns an instance of the current system.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate libnest;
    /// use libnest::system::System;
    ///
    /// let system = System::get();
    /// ```
    pub fn get() -> System {
        System {}
    }
}
