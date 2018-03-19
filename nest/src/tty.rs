use libc::{ioctl, STDOUT_FILENO, TIOCGWINSZ};
use std::os::raw::*;

macro_rules! color {
    ($color:expr, $( $arg:expr ), *) => {
        {
            use libc::isatty;
            use ansi_term::Colour::*;

            if unsafe { isatty(2) } == 1 {
                format!("{}", ($color.bold().paint(format!($( $arg ), * ))))
            } else {
                format!($( $arg ), * )
            }
        }
    };
}

macro_rules! green {
    ( $var:expr ) => (color!(Green, "{}", $var));
    ( $( $arg:expr ), * ) => (color!(Green, $( $arg ),* ));
}

macro_rules! red {
    ( $var:expr ) => (color!(Red, "{}", $var));
    ( $( $arg:expr ), * ) => (color!(Red, $( $arg ),* ));
}

#[repr(C)]
struct WinSize {
    row: c_ushort,
    col: c_ushort,
    xpixel: c_ushort,
    ypixel: c_ushort,
}

/// Returns the width of the tty, or 80 if it's not available.
pub fn width() -> usize {
    let mut winsize = WinSize {
        row: 0,
        col: 0,
        xpixel: 0,
        ypixel: 0,
    };
    if unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut winsize) } != -1 {
        if winsize.col > 0 {
            winsize.col as usize
        } else {
            80
        }
    } else {
        80
    }
}
