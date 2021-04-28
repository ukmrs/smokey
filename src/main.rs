//! ```text
//!   _._ _  _ |  _    
//!  _>| | |(_)|<(/_\/
//!                 /  
//! ```
//! by ukmrs https://github.com/ukmrs/smokey
//! A simple typing test terminal UI app

mod application;
mod colorscheme;
mod painters;
mod langs;
mod utils;
mod handlers;

use std::panic;
use std::time::Duration;
use std::{fs::File, io::stdout};

use application::{App, Screen};
use colorscheme::Theme;
use crossterm::{execute, style::Print};
use utils::terminal_prep;
use handlers::key_handle;

#[macro_use]
extern crate log;

use simplelog::*;

use crossterm::event::{poll, read, Event as CEvent};

use tui::{backend::CrosstermBackend, Terminal};

/// In case of panic restores terminal before program terminates
fn panic_hook(panic_info: &panic::PanicInfo) {
    terminal_prep::cleanup_terminal();
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

    terminal_prep::cleanup_terminal();
    Ok(())
}
