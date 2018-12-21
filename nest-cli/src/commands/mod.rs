use failure::Error;
use libnest::config::Config;

pub fn pull(_config: &Config) -> Result<(), Error> {
    Ok(())
}

pub fn install(_config: &Config) -> Result<(), Error> {
    Ok(())
}

pub fn uninstall(_config: &Config) -> Result<(), Error> {
    Ok(())
}

pub fn upgrade(_config: &Config) -> Result<(), Error> {
    Ok(())
}
