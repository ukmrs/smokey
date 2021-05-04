//! ```text
//!   _._ _  _ |  _    
//!  _>| | |(_)|<(/_\/
//!                 /  
//! ```
//! by ukmrs https://github.com/ukmrs/smokey
//! A simple typing test terminal UI app

mod application;
mod colorscheme;
mod handlers;
mod langs;
mod painters;
mod utils;
use smokey;

use std::time::Duration;
use std::{fs::File, io::stdout};

use application::{App, Screen};
use colorscheme::Theme;
use handlers::key_handle;
use utils::termprep;

#[macro_use]
extern crate log;

use simplelog::*;

use crossterm::event::{poll, read, Event as CEvent};

use tui::{backend::CrosstermBackend, Terminal};

fn main() -> crossterm::Result<()> {
    WriteLogger::init(
        LevelFilter::Debug,
        Config::default(),
        File::create("smokey.log").unwrap(),
    )
    .expect("logger init went fine");

    termprep::init();

    #[allow(unused_mut)]
    let mut sout = stdout();

    let backend = CrosstermBackend::new(sout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    // TODO Theme is all over the place
    // best to integrate into the app struct
    let theme = Theme::new();

    app.reset_test(&theme);
    app.screen = Screen::Test;

    while !app.should_quit {
        // draw to terminal using current painter
        (app.painter)(&mut terminal, &mut app);

        // handle events
        // if there are not after tick rate's been execeed
        // update the screen
        if poll(Duration::from_millis(250))? {
            let read = read()?;
            if let CEvent::Key(event) = read {
                key_handle(event, &mut app, &theme);
            }
        }
    }

    termprep::shutdown();
    Ok(())
}
