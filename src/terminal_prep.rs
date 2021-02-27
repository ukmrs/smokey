use std::io::stdout;

use crossterm::{
    cursor,
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};

pub fn init_terminal() {
    let mut sout = stdout();
    execute!(sout, EnterAlternateScreen).expect("Unable to enter alternate screen");
    // Restore cursor position and clear screen for TTYs
    execute!(sout, cursor::MoveTo(0, 0)).expect("Attempt to write to alternate screen failed.");
    // execute!(sout, cursor::Hide).expect("Unable to hide cursor");
    execute!(sout, Clear(ClearType::All)).expect("Unable to clear screen.");
    enable_raw_mode().expect("Unable to enter raw mode.");
}

pub fn cleanup_terminal() {
    let mut sout = stdout();
    // Restore cursor position and clear screen for TTYs
    // execute!(sout, cursor::MoveTo(0, 0)).expect("Attempt to write to alternate screen failed.");
    execute!(sout, Clear(ClearType::All)).expect("Unable to clear screen.");
    execute!(sout, LeaveAlternateScreen).expect("Unable to leave alternate screen.");
    // execute!(sout, cursor::Show).expect("Unable to restore cursor.");
    disable_raw_mode().expect("Unable to disable raw mode");
}
