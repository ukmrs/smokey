//! smokey by ukmrs https://github.com/ukmrs/smokey
//! A simple typing test terminal UI app

mod application;
mod colorscheme;
mod drawing;
mod langs;
mod testkeys;
mod util;

use std::panic;
use std::time::Duration;
use std::{fs::File, io::stdout};

use application::{App, Screen, TestState};
use colorscheme::Theme;
use crossterm::{execute, style::Print};
use drawing::*;
use testkeys::key_handle;
use util::terminal_prep;

#[macro_use]
extern crate log;

use simplelog::*;

use crossterm::event::{poll, read, Event as CEvent};

use tui::{backend::CrosstermBackend, Terminal};

/// In case of panic restores terminal before program terminates
fn panic_hook(panic_info: &panic::PanicInfo) {
    terminal_prep::cleanup_terminal();
    // from what I discovered
    // overflows expects
    let msg = match panic_info.payload().downcast_ref::<String>() {
        Some(s) => format!("p! {}", s),
        // panic! macro, unwraps
        // dunno if its consisitent, doesn't matter though
        // from docs its commonly String or &'static str
        None => match panic_info.payload().downcast_ref::<&str>() {
            Some(s) => format!("oof! {}", s),
            None => String::from("weird panic hook"),
        },
    };
    let location = panic_info.location().unwrap();
    let mut sout = stdout();
    execute!(sout, Print(format!("{}\n{}\n", msg, location))).unwrap();
}

fn main() -> crossterm::Result<()> {
    WriteLogger::init(
        LevelFilter::Debug,
        Config::default(),
        File::create("smokey.log").unwrap(),
    )
    .expect("logger init went fine");
    terminal_prep::init_terminal();
    panic::set_hook(Box::new(|info| panic_hook(info)));

    #[allow(unused_mut)]
    let mut sout = stdout();

    let backend = CrosstermBackend::new(sout);
    let mut terminal = Terminal::new(backend)?;

    let mut test = TestState::default();
    let mut app = App::new();
    let theme = Theme::new();

    test.reset(&mut app, &theme);
    app.screen = Screen::Settings;

    while !app.should_quit {
        match app.screen {
            Screen::Test => {
                draw_test(&mut terminal, &mut app, &mut test);
                test.update_wpm_history();
            }
            Screen::Post => draw_post(&mut terminal, &mut app, &mut test),
            Screen::Settings => draw_settings(&mut terminal, &mut app, &mut test),
        }

        if poll(Duration::from_millis(300))? {
            let read = read()?;
            if let CEvent::Key(event) = read {
                key_handle(event, &mut app, &mut test, &theme);
            }
        } else {
            // sneak an afk?
            // Timeout expired and no `Event` is available
        }
    }

    terminal_prep::cleanup_terminal();
    Ok(())
}
