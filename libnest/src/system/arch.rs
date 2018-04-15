//! Types for all architectures supported by `libnest`.

use std::fmt::{self, Display, Formatter};

/// An enumeration of all architectures supported by `libnest`.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
#[allow(missing_docs)]
#[allow(non_camel_case_types)]
pub enum Arch {
    x86,
    x86_64,
    Mips,
    PowerPc,
    PowerPc64,
    Arm,
    AArch64,
}

impl Display for Arch {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Arch::x86 => write!(f, "x86"),
            Arch::x86_64 => write!(f, "x86_64"),
            Arch::Mips => write!(f, "mips"),
            Arch::PowerPc => write!(f, "powerpc"),
            Arch::PowerPc64 => write!(f, "powerpc64"),
            Arch::Arm => write!(f, "arm"),
            Arch::AArch64 => write!(f, "aarch64"),
        }
    }
}
