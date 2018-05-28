//! Chroot-like path.
//!
//! This module isn't provided for any kind of sandboxing or any security-like functionnalities, but as
//! a simple way to prevent going before a folder doing `../` and as a path-beautifier.

use std::path::{Component, Path, PathBuf};

/// The Chroot trait provides two functions to interact with [`Path`][1]-like structs:
/// the first one if the current path is the root and the other the content, like in `/this/is/root + /this/is/content = /this/is/root/this/is/content`
/// or the other one, when the current path is the content and the other one is the root.
///
/// The intention behind this trait is to bypass a restriction of [`PathBuf.push`][2] that won't work as we need
/// when joining an absoluthe path with an existing path.
///
/// [1]: https://doc.rust-lang.org/std/path/struct.Path.html
/// [2]: https://doc.rust-lang.org/std/path/struct.PathBuf.html#method.push
pub trait Chroot {
    /// Returns a PathBuf using the current path as the root-base and the given path as the content.
    fn with_content<P: AsRef<Path>>(&self, p: P) -> PathBuf;
    /// Returns a PathBuf using the given path as the root-base and the current path as the content.
    fn with_root<P: AsRef<Path>>(&self, p: P) -> PathBuf;
}

impl Chroot for PathBuf {
    fn with_content<P: AsRef<Path>>(&self, p: P) -> PathBuf {
        self.as_path().with_content(p)
    }

    fn with_root<P: AsRef<Path>>(&self, p: P) -> PathBuf {
        self.as_path().with_root(p)
    }
}

impl Chroot for Path {
    fn with_content<P: AsRef<Path>>(&self, p: P) -> PathBuf {
        let mut out = PathBuf::new();
        for part in p.as_ref().components() {
            match part {
                Component::Prefix(..) | Component::RootDir | Component::CurDir => continue,
                Component::ParentDir => {
                    out.pop();
                }
                Component::Normal(part) => out.push(part),
            }
        }
        assert!(!out.has_root());
        self.join(out)
    }

    fn with_root<P: AsRef<Path>>(&self, p: P) -> PathBuf {
        let mut out = PathBuf::new();
        for part in self.components() {
            match part {
                Component::Prefix(..) | Component::RootDir | Component::CurDir => continue,
                Component::ParentDir => {
                    out.pop();
                }
                Component::Normal(part) => out.push(part),
            }
        }
        assert!(!out.has_root());
        p.as_ref().join(out)
    }
}
