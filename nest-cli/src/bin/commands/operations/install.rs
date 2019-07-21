use failure::{format_err, Error, ResultExt};
use libnest::config::Config;
use libnest::lock_file::LockFileOwnership;
use libnest::transaction::InstallTransaction;

pub fn install_package(
    config: &Config,
    trans: &InstallTransaction,
    ownership: &LockFileOwnership,
) -> Result<(), Error> {
    trans
        .extract(&config, ownership)
        .context(format_err!("unable to extract package"))?;

    println!("Successfully installed {}", trans.target());
    Ok(())
}
