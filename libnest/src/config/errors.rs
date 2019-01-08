//! Errors that can be returned by the config module

use failure::{Context, Fail};
use std::fmt::Display;

/// Error type for configuration errors
#[derive(Debug)]
pub struct ConfigError {
    inner: Context<ConfigErrorKind>,
}

/// Error kind describing a kind of configuration error
#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail)]
pub enum ConfigErrorKind {
    /// The data in the configuration file could not be loaded
    #[fail(display = "unable to load the configuration file")]
    ConfigLoadError,

    /// The data in the configuration file could not be parsed
    #[fail(display = "unable to parse the configuration file")]
    ConfigParseError,
}

macro_rules! use_as_error {
    ($Error:ident, $ErrorKind:ident) => {
        /// Forward the fail implementation to the inner context
        impl Fail for $Error {
            fn cause(&self) -> Option<&Fail> {
                self.inner.cause()
            }

            fn backtrace(&self) -> Option<&failure::Backtrace> {
                self.inner.backtrace()
            }
        }

        /// Forward the display to the inner context
        impl Display for $Error {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                Display::fmt(&self.inner, f)
            }
        }

        impl $Error {
            /// Extract the error kind from the error
            pub fn kind(&self) -> $ErrorKind {
                *self.inner.get_context()
            }
        }

        /// Allow converting from an ErrorKind to an Error
        impl From<$ErrorKind> for $Error {
            fn from(kind: $ErrorKind) -> Self {
                $Error {
                    inner: Context::new(kind),
                }
            }
        }

        /// Allow converting from a Context<ErrorKind> to an Error
        impl From<Context<$ErrorKind>> for $Error {
            fn from(inner: Context<$ErrorKind>) -> Self {
                $Error { inner }
            }
        }
    };
}

use_as_error!(ConfigError, ConfigErrorKind);
