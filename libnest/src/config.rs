//! Nest configuration parsing and handle.
//!
//! There is two way to configure operations using the Nest package manager: globally
//! (configuration file), or locally (command line arguments).
//!
//! Within the `libnest`, many functions take a `&Config` as argument. The main reason is to allow local options to be used only for one operation, even in an
//! asynchronous context.
//!
//! This module provide a `Config` structure that holds all configuration options. This includes,
//! for exemple, proxy settings, cache path, mirrors etc.
//!
//! It also provides a way to load a `Config` from a TOML file.

extern crate toml;

use std::slice::Iter;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::Read;
use std;
use std::path::{Path, PathBuf};
use std::convert::TryFrom;

use curl;
use curl::easy::Easy;

use repository::Repository;

/// A handle to represent a configuration for Nest.
///
/// This handle is given as parameter to each libnest's function so they can use a custom configuration even in an asychronous context.
///
/// Configuration includes proxy settings, cache path, repositories and their mirrors etc.
///
/// # Examples
///
/// ```
/// # extern crate libnest;
/// use libnest::config::Config;
///
/// let config = Config::new();
/// ```
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Config {
    cache: PathBuf,
    installation_dir: PathBuf,
    repositories: Vec<Repository>,
}

static DEFAULT_CACHE_DIR: &'static str = "/var/lib/nest/cache/";
static DEFAULT_INSTALLATION_DIR: &'static str = "/tmp/";

impl Config {
    /// Creates a default configuration.
    ///
    /// The default configuration is:
    /// * Cache path: `/var/lib/nest/cache/`
    ///
    /// All other fields are empty.
    ///
    /// Example:
    /// ```
    /// # extern crate libnest;
    /// use libnest::config::Config;
    ///
    /// let config = Config::new();
    /// ```
    #[inline]
    pub fn new() -> Config {
        let config_file_path = "Config.toml";

        if let Ok(conf) = Config::parse_conf(config_file_path) {
            println!("Using {} as config file", config_file_path);

            if let Some(conf_map) = conf.as_table() {
                let cache_path =
                    Config::get_or_default_str(conf_map, "cache_dir", DEFAULT_CACHE_DIR);
                let install_path =
                    Config::get_or_default_str(conf_map, "install_dir", DEFAULT_INSTALLATION_DIR);
                Config {
                    cache: PathBuf::from(cache_path),
                    installation_dir: PathBuf::from(install_path),
                    repositories: Vec::new(),
                }
            } else {
                return Config::default();
            }
        } else {
            Config::default()
        }
    }

    /// Returns the path holding the cache of each repository.
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate libnest;
    ///
    /// use std::path::Path;
    /// # use libnest::config::Config;
    ///
    /// let config = Config::default();
    ///
    /// assert_eq!(config.cache(), Path::new("/var/lib/nest/cache"));
    /// ```
    #[inline]
    pub fn cache(&self) -> &Path {
        &self.cache
    }

    /// Returns the path of the installation directory.
    ///
    /// # Examples
    /// ```
    /// # extern crate libnest;
    ///
    /// use std::path::Path;
    /// # use libnest::config::Config;
    ///
    /// let config = Config::default();
    ///
    /// assert_eq!(config.installation_dir(), Path::new("/tmp"));
    /// ```
    #[inline]
    pub fn installation_dir(&self) -> &PathBuf {
        &self.installation_dir
    }

    /// Yields a reference to the underlying `Vec<Repository>`
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate libnest;
    /// use libnest::config::Config;
    /// use libnest::repository::Repository;
    ///
    /// let mut config = Config::new();
    /// let repo = Repository::new(&config, "local");
    ///
    /// assert!(config.repositories().is_empty());
    /// ```
    #[inline]
    pub fn repositories(&self) -> &Vec<Repository> {
        &self.repositories
    }

    /// Yields a mutable reference to the underlying `Vec<Repository>`
    ///
    /// # Examples
    ///
    /// ```
    /// # extern crate libnest;
    /// use libnest::config::Config;
    /// use libnest::repository::Repository;
    ///
    /// let mut config = Config::new();
    /// let repo = Repository::new(&config, "local");
    ///
    /// let repos = config.repositories_mut();
    ///
    /// assert!(repos.is_empty());
    /// repos.push(repo);
    /// assert_eq!(repos.len(), 1);
    /// ```
    #[inline]
    pub fn repositories_mut(&mut self) -> &mut Vec<Repository> {
        &mut self.repositories
    }
}

#[derive(Debug)]
enum ParseConfError {
    Io(std::io::Error),
    Deserialize(toml::de::Error),
}

impl Config {
    fn get_or_default_str<'a>(
        conf_map: &'a toml::value::Table,
        key: &'static str,
        default: &'a str,
    ) -> &'a str {
        if let Some(value) = conf_map.get(key) {
            if let Some(value_real_type) = value.as_str() {
                value_real_type
            } else {
                eprintln!(
                    "Config: wrong type for '{}', defaulting to '{}'",
                    key, default
                );
                default
            }
        } else {
            default
        }
    }

    fn get_or_default_primitive<T, U>(
        conf_map: &toml::value::Table,
        key: &str,
        default: U,
        func: T,
    ) -> U
    where
        T: Fn(&toml::value::Value) -> Option<U>,
        U: std::fmt::Display,
    {
        if let Some(value) = conf_map.get(key) {
            if let Some(value_real_type) = func(value) {
                value_real_type
            } else {
                eprintln!(
                    "Config: wrong type for '{}', defaulting to '{}'",
                    key, default
                );
                default
            }
        } else {
            default
        }
    }

    fn parse_conf(conf_path: &str) -> Result<toml::Value, ParseConfError> {
        match File::open(conf_path) {
            Ok(file) => {
                let mut file_reader = BufReader::new(file);
                let mut content = String::new();
                if let Err(e) = file_reader.read_to_string(&mut content) {
                    return Err(ParseConfError::Io(e));
                }
                match content.parse::<toml::Value>() {
                    Ok(v) => Ok(v),
                    Err(e) => Err(ParseConfError::Deserialize(e)),
                }
            }
            Err(e) => Err(ParseConfError::Io(e)),
        }
    }
}

impl Default for Config {
    #[inline]
    fn default() -> Self {
        println!("Using default configuration");
        Config {
            cache: PathBuf::from(DEFAULT_CACHE_DIR),
            installation_dir: PathBuf::from(DEFAULT_INSTALLATION_DIR),
            repositories: Vec::new(),
        }
    }
}

impl<'a> TryFrom<&'a Config> for Easy {
    type Error = curl::Error;

    /// Tries to create a curl handle with the given configuration.
    fn try_from(_: &'a Config) -> Result<Easy, curl::Error> {
        Ok(Easy::new())
    }
}
