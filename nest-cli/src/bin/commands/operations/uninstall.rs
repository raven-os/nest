use failure::Error;
use indicatif::{ProgressBar, ProgressStyle};
use libnest::config::Config;
use libnest::lock_file::LockFileOwnership;
use libnest::transaction::RemoveTransaction;

pub fn uninstall_package(
    config: &Config,
    trans: &RemoveTransaction,
    ownernship: &LockFileOwnership,
) -> Result<(), Error> {
    let progress_bar = ProgressBar::new(80);
    progress_bar.set_style(ProgressStyle::default_bar().template("[{pos:>3}/{len:3}] {bar:80}"));

    // Remove the package
    progress_bar.println(format!("Removing {}...", trans.target()));
    trans.perform(config, ownernship)?;

    progress_bar.finish_and_clear();
    println!("Successfully uninstalled {}", trans.target());
    Ok(())
}
