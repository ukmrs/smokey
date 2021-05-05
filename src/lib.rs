pub mod application;
pub mod colorscheme;
mod handlers;
mod langs;
mod painters;
pub mod utils;

#[macro_use]
extern crate log;
use simplelog::*;

use application::{App, Screen};
use crossterm::event::{poll, read, Event as CEvent};
use handlers::key_handle;
use std::fs::File;
use std::io::Stdout;
use std::time::Duration;
use utils::termprep;

use tui::{backend::CrosstermBackend, Terminal};

pub type Backend = CrosstermBackend<Stdout>;
pub type Term = Terminal<Backend>;

pub fn run(mut app: App, terminal: Term) -> crossterm::Result<()> {
    init_logger();

    app.reset_test();
    app.screen = Screen::Test;

    termprep::init();
    main_loop(app, terminal)?;
    termprep::shutdown();
    Ok(())
}

fn main_loop<'a>(mut app: App<'a>, mut terminal: Term) -> crossterm::Result<()> {
    while app.is_alive {
        // drawing to the screen
        app.paint(&mut terminal);

        // handling events
        if poll(Duration::from_millis(250))? {
            let read = read()?;
            if let CEvent::Key(event) = read {
                key_handle(event, &mut app);
            }
        }
    }
    Ok(())
}

fn init_logger() {
    WriteLogger::init(
        LevelFilter::Debug,
        Config::default(),
        File::create("smokey.log").unwrap(),
    )
    .expect("logger init went oof");
}
