use failure::{format_err, Error, ResultExt};
use indicatif::{ProgressBar, ProgressStyle};
use libnest::config::Config;
use libnest::lock_file::LockFileOwnership;

use libnest::transaction::UpgradeTransaction;

pub fn upgrade_package(
    config: &Config,
    trans: &UpgradeTransaction,
    ownership: &LockFileOwnership,
) -> Result<(), Error> {
    let progress_bar = ProgressBar::new(80);
    progress_bar.set_style(ProgressStyle::default_bar().template("[{pos:>3}/{len:3}] {bar:80}"));

    // Upgrade the package
    progress_bar.println(format!(
        "Upgrading {} to {}...",
        trans.old_target(),
        trans.new_target()
    ));
    trans
        .perform(config, ownership)
        .with_context(|_| format_err!("unable to extract package"))?;

    progress_bar.finish_and_clear();
    println!("Successfully upgraded to {}", trans.new_target());
    Ok(())
}
