//! provides init and shutdown functions allowing for entering
//! and gracefully exiting alternate screen where program can
//! happily perform its shenanigans.
//! init function sets up panic hook that will call shutdown()
//! preventing messing up the terminal if the program were to panic!
//!
//! # Usage
//! ```no_run
//! use smokey::utils::termprep;
//! termprep::init();
//! // main tui app loop
//! termprep::shutdown();
//! ```

use std::io::stdout;
use std::panic;
use std::process;

use crossterm::{
    cursor, execute,
    style::Print,
    terminal::{
        disable_raw_mode, enable_raw_mode, is_raw_mode_enabled, Clear, ClearType,
        EnterAlternateScreen, LeaveAlternateScreen,
    },
};

/// enters alt screen and sets up panic hook that prevents
/// messing up the user terminal if this program were to panic
pub fn init() {
    init_terminal();
    set_panic_hook();
}

/// performs all nesccesarry actions to leave alt screen
/// and return to terminal state before the program was run
pub fn shutdown() {
    cleanup_terminal();
}

pub fn panic_with_friendly_message(msg: &str) {
    let raw_mode_enabled = is_raw_mode_enabled()
        .expect("funny thing: I tried to prepare a nice error message but failed");

    if raw_mode_enabled {
        shutdown();
    }

    eprintln!("{}", msg);
    process::exit(1);
}

/// enters alt screen and all that good stuff
fn init_terminal() {
    let mut sout = stdout();
    execute!(sout, EnterAlternateScreen).expect("enter alt screen");
    execute!(sout, cursor::MoveTo(0, 0)).expect("write to alt screen failed");
    execute!(sout, Clear(ClearType::All)).expect("Unable to clear screen.");
    enable_raw_mode().expect("Unable to enter raw mode.");
}

/// leaves the alt screen and leaves terminal as it was before
/// launching the program
fn cleanup_terminal() {
    let mut sout = stdout();
    execute!(sout, Clear(ClearType::All)).expect("Unable to clear screen.");
    execute!(sout, LeaveAlternateScreen).expect("Unable to leave alternate screen.");
    disable_raw_mode().expect("Unable to disable raw mode");
}

// would drop work?

fn panic_hook(panic_info: &panic::PanicHookInfo) {
    cleanup_terminal();
    let msg = match panic_info.payload().downcast_ref::<String>() {
        Some(s) => format!("p! {}", s),
        None => match panic_info.payload().downcast_ref::<&str>() {
            Some(s) => format!("oof! {}", s),
            None => String::from("weird panic hook"),
        },
    };
    let location = panic_info.location().unwrap();
    let mut sout = stdout();
    execute!(sout, Print(format!("{}\n{}\n", msg, location))).unwrap();
}

/// In case of panic restores terminal before program terminates
fn set_panic_hook() {
    panic::set_hook(Box::new(panic_hook));
}
