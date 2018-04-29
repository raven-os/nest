//! Types, enums and macros for error handling, using [`failure`]
//!
//! [`failure`](https://docs.rs/failure/0.1.1/failure/)

use std::fmt::{self, Display, Formatter};

use failure::{Backtrace, Context, Fail};

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

/// The kind of a [`RepositoryError`]
///
/// [`RepositoryError`](html.struct.RepositoryError)
#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail)]
pub enum RepositoryErrorKind {
    #[fail(display = "all mirrors are down")]
    AllMirrorDown,
}

/// Errors that amy occure when manipulating repositories.
#[derive(Debug)]
pub struct RepositoryError {
    inner: Context<RepositoryErrorKind>,
}

impl RepositoryError {
    /// Returns a [`RepositoryErrorKind`] the reason why this error was thrown
    ///
    /// [`RepositoryErrorKind`](html.enum.RepositoryErrorKind)
    pub fn kind(&self) -> RepositoryErrorKind {
        *self.inner.get_context()
    }
}

impl Fail for RepositoryError {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for RepositoryError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl From<RepositoryErrorKind> for RepositoryError {
    fn from(kind: RepositoryErrorKind) -> RepositoryError {
        RepositoryError {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<RepositoryErrorKind>> for RepositoryError {
    fn from(inner: Context<RepositoryErrorKind>) -> RepositoryError {
        RepositoryError { inner }
    }
}

/// The kind of a [`QueryError`]
///
/// [`QueryError`](html.struct.QueryError)
// XXX The display implementation for this enum members aren't used. Instead, QueryError implements a long, nice and complete error message.
#[derive(Clone, Eq, PartialEq, Debug, Fail)]
pub enum QueryErrorKind {
    #[fail(display = "couldn't find a package with name \"{}\"", _0)]
    NoResult(String),
    #[fail(display = "found {} packages with name \"{}\", please be more explicit", _1, _0)]
    TooManyResults(String, usize, Vec<String>),
    #[fail(display = "\"{}\" isn't a valid package name", _0)]
    InvalidPackageName(String),
}

impl QueryErrorKind {
    /// Returns an advice (for the end-user) to help him understand a failed query
    ///
    /// The returned advices may be on multiple lines, and start with a capital
    /// letter and ends with a dot followed by a '\n'.
    /// Therefore, they can't be part of an already existing sentence.
    pub fn advices(&self) -> String {
        use std::fmt::Write;

        match self {
            QueryErrorKind::NoResult(_) => format!(
                "Try \"{}\" to look for an existing package.\n",
                purple!("nest search")
            ),
            QueryErrorKind::TooManyResults(_, _, ref packages) => {
                let mut s = String::from("Packages found:\n");
                for package in packages.iter() {
                    writeln!(&mut s, "\t- {}", purple!(package)).expect("Failed to write advice");
                }
                s
            }
            QueryErrorKind::InvalidPackageName(_) => String::new(),
        }
    }
}

/// Errors that amy occure when querying manifests
#[derive(Debug)]
pub struct QueryError {
    inner: Context<QueryErrorKind>,
}

impl QueryError {
    pub fn kind(&self) -> &QueryErrorKind {
        self.inner.get_context()
    }
}

impl Fail for QueryError {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for QueryError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl From<QueryErrorKind> for QueryError {
    fn from(kind: QueryErrorKind) -> QueryError {
        QueryError {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<QueryErrorKind>> for QueryError {
    fn from(inner: Context<QueryErrorKind>) -> QueryError {
        QueryError { inner }
    }
}
