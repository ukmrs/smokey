pub mod application;
pub mod colorscheme;
mod handlers;
mod langs;
mod painters;
pub mod typer;
pub mod utils;

#[allow(unused_imports)]
#[macro_use]
extern crate log;
use simplelog::*;

use application::App;
use crossterm::event::{poll, read, Event as CEvent};
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

    termprep::init();
    main_loop(app, terminal)?;
    termprep::shutdown();
    Ok(())
}

fn main_loop(mut app: App, mut terminal: Term) -> crossterm::Result<()> {
    while app.is_alive {
        // drawing to the screen
        app.paint(&mut terminal);

        // handling events
        if poll(Duration::from_millis(250))? {
            let read = read()?;
            if let CEvent::Key(event) = read {
                app.handle_key_event(event)
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
