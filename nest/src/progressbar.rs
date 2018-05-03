//! Types and functions to add a progress bar while a background action takes place.

use std::io::{self, Write};
use std::time::{Duration, Instant};

use failure::Error;
use tty;

lazy_static! {
    static ref REFRESH_RATE: Duration = Duration::new(0, NANOS_PER_SEC / 10);
}
static NANOS_PER_SEC: u32 = 1_000_000_000;
static BYTES_UNITS: [&'static str; 9] =
    ["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB", "ZiB", "YiB"];
static TIME_UNITS: [&'static str; 3] = ["s", "m", "h"];

/// Current state of a progress bar.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ProgressState {
    Running,
    Ok,
    Err,
}

/// A progres bar and all it's internal data.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct ProgressBar {
    current: usize,
    max: usize,
    action: String,
    target: String,
    start_time: Instant,
    next_time: Instant,
    status: ProgressState,
}

impl ProgressBar {
    /// Creates a new `ProgressBar` with default values.
    ///
    /// The given `action` parameter is the name of the action done. It will be printed with
    /// colors, and cannot go over 8 chars.
    /// Maximum value is 100.
    pub fn new(action: String) -> ProgressBar {
        let now = Instant::now();
        ProgressBar {
            current: 0,
            max: 100,
            action,
            target: String::new(),
            start_time: now,
            next_time: now, // Make sure the progress bar will be drawed on first update
            status: ProgressState::Running,
        }
    }

    /// Sets the action that should be color-printed at the beginning of the progress bar
    pub fn set_action(&mut self, action: String) {
        self.action = action;
    }

    /// Sets the name of the target of the current action.
    ///
    /// This will be printed after the action, in white.
    pub fn set_target(&mut self, target: String) {
        self.target = target;
    }

    /// Sets the maximum value for the progress bar.
    ///
    /// `0` is a valid value.
    pub fn set_max(&mut self, max: usize) {
        self.max = max;
    }

    /// Updates the current value with the one given.
    ///
    /// If the given value is over the maximum value, it will be troncated.
    pub fn update(&mut self, mut val: usize) {
        if val > self.max {
            val = self.max;
        }
        self.current = val;
        self.render();
    }

    // XXX: Should return a more accurate speed instead of average speed
    pub fn speed(&self, time_elapsed: &Duration) -> f64 {
        let ftime_elapsed = time_elapsed.as_secs() as f64
            + f64::from(time_elapsed.subsec_nanos()) / f64::from(NANOS_PER_SEC);
        self.current as f64 / ftime_elapsed
    }

    pub fn time_left(&self, speed: f64) -> f64 {
        if speed > 0.0 {
            1.0 / speed * (self.max - self.current) as f64
        } else {
            0.0
        }
    }

    pub fn ratio(&self) -> f64 {
        if self.max > 0 {
            self.current as f64 / self.max as f64
        } else {
            0.0
        }
    }

    pub fn render(&mut self) {
        let now = Instant::now();

        // Refresh rate
        if now > self.next_time {
            self.draw();
            self.next_time = now + *REFRESH_RATE;
        }
    }

    /// Draws the progress bar
    fn draw(&self) {
        let now = Instant::now();
        let time_elapsed = now.duration_since(self.start_time);

        // Speed calculation
        let speed = self.speed(&time_elapsed);
        let time_left = self.time_left(speed);
        let ratio = self.ratio();

        // Width calculation
        let tty_width = tty::width();
        let half_width = tty_width / 2;
        let bar_width = half_width as f64 - 22.0; // half_width - "1000MiB/s 59m [>] 100%"
        let left_width = half_width - 27; // half_width - " <action> " ... " 1000MiB/1000MiB "
        let right_width = half_width + 17; // half_width + " 1000MiB/1000MiB "
        let current_width = (ratio * bar_width).round();
        let remaining_width = bar_width - current_width;

        print!(
            "\r{}{}",
            match self.status {
                ProgressState::Running => cyan!(" {:>8.8} ", self.action),
                ProgressState::Ok => green!(" {:>8.8} ", self.action),
                ProgressState::Err => red!(" {:>8.8} ", self.action),
            },
            // Draw progress bar if the operation is running since at least 0.25s
            if self.status == ProgressState::Running
                && (time_elapsed.as_secs() > 0 || time_elapsed.subsec_nanos() > NANOS_PER_SEC / 4)
                && self.max > 0
            {
                format!(
                    "{:<left_width$.left_width$}{:<right_width$.right_width$}",
                    &self.target,
                    format!(" {current:>7.7}/{max:<7.7} {speed:>7.7}/s {time_left:>3.3} [{phantom:=<current_width$.width$}>{phantom:-<remaining_width$.width$}] {percent:>3.3}%",
                        current = humanize_bytes(self.current as f64),
                        max = humanize_bytes(self.max as f64),
                        speed = humanize_bytes(speed),
                        time_left = humanize_time(time_left),
                        phantom = "",
                        width = bar_width as usize,
                        current_width = current_width as usize,
                        remaining_width = remaining_width as usize,
                        percent = (ratio * 100.0).round() as u32,
                    ),
                    left_width = left_width,
                    right_width = right_width,
                )
            } else {
                format!(
                    "{:<width$.width$}",
                    self.target.clone(),
                    width = left_width + right_width,
                )
            }
        );
        io::stdout().flush().expect("Couldn't flush stdout");
    }

    /// Redraws the `ProgressBar` with the given status.
    pub fn finish<T>(&mut self, status: &Result<T, Error>) {
        match *status {
            Ok(_) => self.status = ProgressState::Ok,
            Err(ref e) => {
                self.target = format!("{} - {}.", self.target, format_error_causes!(e));
                self.status = ProgressState::Err;
            }
        }
        self.draw();
        println!();
    }
}

impl Default for ProgressBar {
    fn default() -> ProgressBar {
        ProgressBar::new(String::from("default"))
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
