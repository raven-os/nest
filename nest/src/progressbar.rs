//! Types and functions to add a progress bar while a background action takes place.

use std::io::{self, Write};
use std::time::{Duration, Instant};

use libnest::transaction::{TransactionKind, TransactionStep};

use failure::Error;
use lazy_static::lazy_static;

use crate::tty;

lazy_static! {
    static ref REFRESH_RATE: Duration = Duration::new(0, NANOS_PER_SEC / 10);
}
static NANOS_PER_SEC: u32 = 1_000_000_000;
static BYTES_UNITS: [&'static str; 9] =
    ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB", "YiB"];
static TIME_UNITS: [&'static str; 3] = ["s", "m", "h"];

/// Current state of a progress bar.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ProgressState {
    Running,
    Ok,
    Err,
}

/// A progres bar and all it's internal data.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct ProgressBar {
    transaction: TransactionKind,
    target: String,
    retry: bool,
    status: ProgressState,
    step: TransactionStep,
    current: usize,
    max: usize,
    start_time: Instant,
    next_time: Instant,
}

impl ProgressBar {
    /// Creates a new `ProgressBar` with default values.
    ///
    /// The given `action` parameter is the name of the executed action. It will be printed with
    /// colors, and cannot go over 8 chars.
    /// Maximum value is 100.
    pub fn new(transaction: TransactionKind, target: String) -> ProgressBar {
        let now = Instant::now();
        ProgressBar {
            transaction,
            target,
            retry: false,
            status: ProgressState::Running,
            step: TransactionStep::Waiting,
            current: 0,
            max: 0,
            start_time: now,
            next_time: now, //XXX: Make sure the progress bar will be drawed on first update
        }
    }

    /// Updates the current value and maximum value with the ones given.
    ///
    /// If the current value is over the maximum value, it will be set as equal to the maximum value.
    pub fn update(&mut self, mut val: usize, max: usize) {
        if val > max {
            val = max;
        }
        self.max = max;
        self.current = val;
        self.render();
    }

    /// Returns the speed at which the progress bar is going.
    // XXX: Should return a more accurate speed instead of average speed
    pub fn speed(&self, time_elapsed: &Duration) -> f64 {
        let ftime_elapsed = time_elapsed.as_secs() as f64
            + f64::from(time_elapsed.subsec_nanos()) / f64::from(NANOS_PER_SEC);
        self.current as f64 / ftime_elapsed
    }

    /// Returns how much time is left before the action ends.
    pub fn time_left(&self, speed: f64) -> f64 {
        if speed > 0.0 {
            1.0 / speed * (self.max - self.current) as f64
        } else {
            0.0
        }
    }

    /// Returns a ratio of the current value to the max value.
    pub fn ratio(&self) -> f64 {
        if self.max > 0 {
            self.current as f64 / self.max as f64
        } else {
            0.0
        }
    }

    /// Renders the progress bar on screen.
    pub fn render(&mut self) {
        let now = Instant::now();

        // Refresh rate
        if now >= self.next_time {
            self.draw();
            self.next_time = now + *REFRESH_RATE;
        }
    }

    /// Draws the progress bar.
    fn draw(&self) {
        let time_elapsed = Instant::now().duration_since(self.start_time);

        // Speed calculation
        let speed = self.speed(&time_elapsed);
        let time_left = self.time_left(speed);
        let ratio = self.ratio();

        // Width calculation
        let tty_width = tty::width();

        let mut bar_width = tty_width as f64 - 78.0; // Everything but the progress bar
        let content_length = tty_width - 41;

        if bar_width < 0. {
            bar_width = 0.;
        }
        let current_width = (ratio * bar_width).round();
        let remaining_width = bar_width - current_width;

        /*
        0         1         2         3         4         5         6         7
        01234567890123456789012345678901234567890123456789012345678901234567890123456789
         <action> <target             > <state >
         <action> <target             > <step  > <cur  >/<max  > <speed>/s <T> [PB.]<%%>

         Examples:
          install sys-lib/gcc           download 1000MiB/1999MiB 1999MiB/s 33s [=>-] 99%
          install sys-lib/gcc           Finished
        */

        /// XXX: Me having fun with nested scopes and block values
        print!(
            "\r {action} {target:<21.21} {state} {progress:<content_length$.content_length$}",
            action = match self.transaction {
                TransactionKind::Pull => cyan!("{:>8.8}", "pull"),
                TransactionKind::Install => green!("{:>8.8}", "install"),
                TransactionKind::Remove => red!("{:>8.8}", "remove"),
                TransactionKind::Upgrade => yellow!("{:>8.8}", "upgrade"),
                TransactionKind::Downgrade => yellow!("{:>8.8}", "downgrade"),
                TransactionKind::Reinstall => yellow!("{:>8.8}", "reinstall"),
            },
            target = self.target,
            state = match self.status {
                ProgressState::Ok => green!("{:<8.8}", "Finished"),
                ProgressState::Err => red!("{:<8.8}", "Failed"),
                ProgressState::Running => cyan!("{:<8.8}", self.step.to_string()),
            },
            progress = {
                // Draw progress bar only after 0.25s and max > 0
                if self.status == ProgressState::Running
                    && (time_elapsed.as_secs() > 0
                        || time_elapsed.subsec_nanos() > NANOS_PER_SEC / 4)
                    && self.max > 0
                {
                    format!(
                        "{current:>7.7}/{max:<7.7} {speed:>7.7}/s {time_left:>3.3} [{phantom:=<current_width$.width$}>{phantom:-<remaining_width$.width$}]{percent:>3.3}%",
                        current = humanize_bytes(self.current as f64),
                        max = humanize_bytes(self.max as f64),
                        speed = humanize_bytes(speed),
                        time_left = humanize_time(time_left),
                        phantom = "",
                        width = bar_width as usize,
                        current_width = current_width as usize,
                        remaining_width = remaining_width as usize,
                        percent = (ratio * 100.0).round() as u32,
                    )
                } else {
                    String::new()
                }
            },
            content_length = content_length,
        );
        io::stdout().flush().expect("Couldn't flush stdout");
    }

    /// Resets the current step, marking this attemps as a retry
    pub fn retry(&mut self) {
        self.retry = true;
        self.status = ProgressState::Running;
        self.max = 0;
        self.current = 0;
        self.next_time = Instant::now();
        self.start_time = Instant::now();
        self.render();
    }

    pub fn next_step(&mut self, step: TransactionStep) {
        self.retry = false;
        self.status = ProgressState::Running;
        self.max = 0;
        self.current = 0;
        self.step = step;
        self.next_time = Instant::now();
        self.start_time = Instant::now();
        self.render();
    }

    /// Redraws the `ProgressBar` with the given step status.
    pub fn finish<T>(&mut self, status: &Result<T, Error>) {
        if self.status == ProgressState::Running {
            match status {
                Ok(_) => self.status = ProgressState::Ok,
                Err(_) => self.status = ProgressState::Err,
            }
            self.draw();
        }
        println!();
    }
}

/// Returns a human-readable string for a given value in bytes.
fn humanize_bytes(mut bytes: f64) -> String {
    for unit in &BYTES_UNITS {
        if bytes <= 2048.0 {
            return format!("{}{}", bytes.round() as usize, unit);
        }
        bytes /= 1024.0;
    }
    String::from("???")
}

/// Returns a human-readable string for a given value in seconds.
fn humanize_time(mut time: f64) -> String {
    for unit in &TIME_UNITS {
        if time <= 60.0 {
            return format!("{}{}", time.round() as usize, unit);
        }
        time /= 60.0;
    }
    String::from("???")
}
