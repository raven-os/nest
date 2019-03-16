//! Simple file-based locking to prevent race conditions when running multiple instances of Nest

use std::fs::File;
use std::ops::Drop;
use std::path::Path;

use failure::Error;
use fs2::FileExt;

/// A handle representing ownership over Nest's lock file
#[derive(Debug)]
pub struct LockFileOwnership {
    lock_file: File,
}

impl LockFileOwnership {
    pub(crate) fn acquire(path: &Path, should_wait: bool) -> Result<Self, Error> {
        let f = File::create(path)?;

        if should_wait {
            f.lock_exclusive()?;
        } else {
            f.try_lock_exclusive()?;
        }
        Ok(LockFileOwnership { lock_file: f })
    }

    fn release(&mut self) {
        self.lock_file
            .unlock()
            .expect("unable to release the lock file");
    }
}

impl Drop for LockFileOwnership {
    fn drop(&mut self) {
        self.release()
    }
}
