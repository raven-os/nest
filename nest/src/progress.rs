//! A structure representing the progress of the operation executed.

use std::fmt::{self, Display, Formatter};

/// Represents the number of steps done out of the total number of steps.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Progress {
    current: usize,
    max: usize,
}

impl Progress {
    /// Creates a new `Progress`.
    pub fn new(max: usize) -> Progress {
        Progress { current: 1, max }
    }

    /// Returns the current number of steps done.
    pub fn current(&self) -> usize {
        self.current
    }

    /// Returns the total number of steps done.
    pub fn max(&self) -> usize {
        self.max
    }

    /// Increments by one the current number of steps done.
    pub fn next(&mut self) {
        self.current += 1
    }
}

impl Display for Progress {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}/{}", self.current, self.max)
    }
}
