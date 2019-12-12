//! Errors that can be returned by the config module

use failure::{Context, Fail};

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

    /// The data in the configuration file is invalid
    #[fail(display = "invalid configuration file")]
    InvalidConfigFile,
}

use_as_error!(ConfigError, ConfigErrorKind);
