use failure::Error;

use transaction::{Transaction, TransactionStep};

/// Notifications that transactions may use to notify the parent function.
#[derive(Debug)]
pub enum Notification<'a> {
    /// The transaction enters a new step. The boolean parameter indicates wheter or not this is a
    /// retry (true) or the first attempt (false).
    NewStep(TransactionStep, bool),
    /// Indicates the progress of the current step, giving the current amount of progress out of a total.
    Progress(usize, usize),
    /// The transaction is finished, and the result is given for notifying-purposes.
    FinishTransaction(&'a Result<(), Error>),
    /// A warning (non-fatal error) occured.
    Warning(Error),
}

/// The [`Notifier`] allows a parent process to watch what is happening inside an [`Orchestrator`] and the
/// transactions that are performed.
#[allow(missing_debug_implementations)]
pub struct Notifier<'a> {
    notify_callback: Box<FnMut(&Transaction, Notification) + 'a>,
}

impl<'a> Notifier<'a> {
    /// Creates a new notifier from it's callback.
    ///
    /// The callback is called when a transaction notifies it's parent about an event. These events
    /// exists as the [`Notification`] enum, and may contain a context with them
    /// (like the current and maximum value of the `Progress` notification, etc...)
    #[inline]
    pub fn new<F1>(notify: F1) -> Notifier<'a>
    where
        F1: FnMut(&Transaction, Notification) + 'a,
    {
        Notifier {
            notify_callback: Box::new(notify),
        }
    }

    /// Notifies the parent process of the given event.
    #[inline]
    pub fn notify(&mut self, transaction: &Transaction, notification: Notification) {
        (self.notify_callback)(transaction, notification);
    }
}
