//! Nest configuration parsing and handle.

extern crate toml;

use std::path::PathBuf;
use std::slice::Iter;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::Read;
use std;

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
/// extern crate libnest;
///
/// use libnest::config::Config;
///
/// let config = Config::new();
/// ```
#[derive(Debug)]
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
    /// extern crate libnest;
    ///
    /// use libnest::config::Config;
    ///
    /// let config = Config::new();
    /// ```
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
    pub fn cache(&self) -> &PathBuf {
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
    pub fn installation_dir(&self) -> &PathBuf {
        &self.installation_dir
    }

    /// Adds the given repository at the end of the list of repositories, meaning it has the lowest
    /// priority when looking for a package.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate libnest;
    ///
    /// use libnest::config::Config;
    /// use libnest::repository::Repository;
    ///
    /// let mut config = Config::new();
    /// let repo = Repository::new(&config, "local");
    ///
    /// config.add_repository(repo);
    /// ```
    pub fn add_repository(&mut self, repo: Repository) {
        self.repositories.push(repo);
    }

    /// Returns a reference on the vector containing all the mirrors.
    ///
    /// # Examples
    ///
    /// ```
    /// extern crate libnest;
    ///
    /// use libnest::config::Config;
    /// use libnest::repository::Repository;
    ///
    /// let mut config = Config::new();
    /// let repo = Repository::new(&config, "local");
    ///
    /// assert_eq!(config.repositories().len(), 0);
    /// config.add_repository(repo);
    /// assert_eq!(config.repositories().len(), 1);
    /// ```
    pub fn repositories(&self) -> Iter<Repository> {
        self.repositories.iter()
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
    fn default() -> Self {
        println!("Using default configuration");
        Config {
            cache: PathBuf::from(DEFAULT_CACHE_DIR),
            installation_dir: PathBuf::from(DEFAULT_INSTALLATION_DIR),
            repositories: Vec::new(),
        }
    }
}
