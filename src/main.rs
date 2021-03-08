mod application;
mod colorscheme;
mod drawing;
mod langs;
mod terminal_prep;
mod testkeys;

use std::time::{Duration, Instant};
use std::{borrow::Cow, error::Error, fs::File, io::stdout, sync::mpsc, thread};

use application::{App, Screen, TestState};
use colorscheme::Theme;
use drawing::draw;
use terminal_prep::{cleanup_terminal, init_terminal};
use testkeys::test_key_handle;

#[allow(unused_imports)]
#[macro_use]
extern crate log;

use simplelog::*;

use crossterm::event::{poll, read, Event as CEvent, KeyCode, KeyEvent, KeyModifiers};

use tui::{backend::CrosstermBackend, Terminal};

fn main() -> crossterm::Result<()> {
    WriteLogger::init(
        LevelFilter::Debug,
        Config::default(),
        File::create("smokey.log").unwrap(),
    )
    .unwrap();

    init_terminal();

    #[allow(unused_mut)]
    let mut sout = stdout();

    let backend = CrosstermBackend::new(sout);
    let mut terminal = Terminal::new(backend)?;

    let mut test = TestState::default();
    let mut app = App::create_app();
    let theme = Theme::initial();

    test.restart_test(&mut app, &theme);

    loop {
        if app.should_quit {
            break;
        }

        match app.screen {
            Screen::Test => draw(&mut terminal, &mut app, &mut test),
            _ => todo!(),
        }

        if poll(Duration::from_millis(250))? {
            let read = read()?;
            if let CEvent::Key(event) = read {
                test_key_handle(event.code, &mut app, &mut test, &theme);
            }
        } else {
            // TODO a tick event?
            // sneak an afk?
            // Timeout expired and no `Event` is available
        }
    }

    cleanup_terminal();
    Ok(())
}
