use std::io::{Seek, SeekFrom, Write};

use curl::easy::Easy;
use failure::{Error, ResultExt};

use crate::config::{Config, RepositoryConfig};
use crate::error::TransferError;
use crate::transaction::{Notification, Notifier, Transaction, TransactionStep};

/// Wraps in a uniform way a download from a repository (using the best mirror, and falling back
/// to the others in case of failure) while notifying the user of it's progress.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Transfer<'a, 'b> {
    config: &'a Config,
    repo_config: &'b RepositoryConfig,
    target: String,
}

impl<'a, 'b> Transfer<'a, 'b> {
    pub(crate) fn from(config: &'a Config, repo_config: &'b RepositoryConfig) -> Transfer<'a, 'b> {
        Transfer {
            config,
            repo_config,
            target: String::from("/"),
        }
    }

    /// Sets the target route for this transfer.
    ///
    /// The route is the part after following the domain-name in an URL.
    ///
    /// eg: `https://example.com/THIS/IS/THE/ROUTE`.
    ///
    /// It will be appended to the domain name of the mirror currently in use.
    pub fn target(mut self, target: String) -> Self {
        self.target = target;
        self
    }

    /// Performs the transfer, writting it's output to `writer` and notifing the user with the given `notifier`.
    ///
    /// This is a blocking call.
    pub fn perform<W>(
        &mut self,
        writer: &mut W,
        transaction: &Transaction,
        notifier: &mut Notifier,
    ) -> Result<(), Error>
    where
        W: Write + Seek,
    {
        let mut curl = Easy::new();
        let mut retry = false;

        curl.follow_location(true)?;
        curl.fail_on_error(true)?;
        curl.progress(true)?;

        let any = self.repo_config.mirrors().iter().any(|mirror| {
            let res: Result<_, Error> = try {
                // Notify parent about this new transfer attempt
                notifier.notify(
                    transaction,
                    Notification::NewStep(TransactionStep::Download, retry),
                );
                retry = true;

                let url = mirror.join(&self.target)?;
                curl.url(url.as_str())?;

                // Clear data of previous attempt
                writer.seek(SeekFrom::Start(0))?;

                {
                    let mut transfer = curl.transfer();
                    transfer.write_function(|new_data| Ok(writer.write(new_data).unwrap_or(0)))?;
                    transfer.progress_function(|a, b, _, _| {
                        notifier
                            .notify(transaction, Notification::Progress(b as usize, a as usize));
                        true
                    })?;
                    transfer.perform()?;
                }
                ()
            };
            let res: Result<_, Error> =
                res.with_context(|_| mirror.to_string()).map_err(From::from);
            match res {
                Ok(_) => true,
                Err(e) => {
                    notifier.notify(transaction, Notification::Warning(e)); // Warn parrent if mirror failed
                    false
                }
            }
        });
        if !any {
            Err(TransferError::AllMirrorsDown)?;
        }
        Ok(())
    }
}
