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
    execute!(sout, EnterAlternateScreen).expect("enter alt screen");
    execute!(sout, cursor::MoveTo(0, 0)).expect("write to alt screen failed");
    execute!(sout, Clear(ClearType::All)).expect("Unable to clear screen.");
    enable_raw_mode().expect("Unable to enter raw mode.");
}

pub fn cleanup_terminal() {
    let mut sout = stdout();
    execute!(sout, Clear(ClearType::All)).expect("Unable to clear screen.");
    execute!(sout, LeaveAlternateScreen).expect("Unable to leave alternate screen.");
    disable_raw_mode().expect("Unable to disable raw mode");
}
