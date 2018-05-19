//! A structure representing the progress of the operation executed.

use std::fmt::{self, Display, Formatter};

/// Represents the number of steps done out of the total number of steps.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub struct Progress {
    current: usize,
    max: usize,
}

impl Progress {
    pub fn new(max: usize) -> Progress {
        Progress { current: 1, max }
    }

    pub fn current(&self) -> usize {
        self.current
    }

    pub fn max(&self) -> usize {
        self.max
    }

    pub fn next(&mut self) {
        self.current += 1
    }
}

impl Display for Progress {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}/{}", self.current, self.max)
    }
}
