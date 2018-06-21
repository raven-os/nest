//! Types, enums and macros for error handling, using [`failure`][1].
//!
//! [1]: https://docs.rs/failure/0.1.1/failure/

macro_rules! format_error_causes {
    ($error:expr) => {{
        let mut s = format!("{}", $error.cause());
        for cause in $error.causes().skip(1) {
            s += &format!(": {}", cause);
        }
        s
    }};
}

macro_rules! format_error {
    ($error:expr) => {{
        format!("{}: {}.", red!("error"), format_error_causes!($error))
    }};
}

/// Errors that may occure when manipulating repositories.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail)]
pub enum RepositoryError {
    #[fail(display = "all mirrors are down")]
    AllMirrorDown,
}

/// Errors that may occure when querying manifests
// XXX: The display implementation for this enum members aren't used. Instead, QueryError implements a long, nice and complete error message.
#[derive(Clone, Eq, PartialEq, Debug, Fail)]
pub enum QueryError {
    #[fail(display = "couldn't find a package with name \"{}\"", _0)]
    NoResult(String),
    #[fail(display = "found {} packages with name \"{}\", please be more explicit", _1, _0)]
    TooManyResults(String, usize, Vec<String>),
    #[fail(display = "\"{}\" isn't a valid package name", _0)]
    InvalidPackageName(String),
}

impl QueryError {
    /// Returns an advice (for the end-user) to help him understand a failed query
    ///
    /// The returned advices may be on multiple lines, start with an uppercase
    /// letter and end with a dot followed by a '\n'.
    /// Therefore, they can't be part of an already existing sentence.
    pub fn advices(&self) -> String {
        use std::fmt::Write;

        match self {
            QueryError::NoResult(_) => format!(
                "Try \"{}\" to look for an existing package.\n",
                purple!("nest search")
            ),
            QueryError::TooManyResults(_, _, ref packages) => {
                let mut s = String::from("Packages found:\n");
                for package in packages.iter() {
                    writeln!(&mut s, "\t- {}", purple!(package)).expect("Failed to write advice");
                }
                s
            }
            QueryError::InvalidPackageName(_) => String::new(),
        }
    }
}
