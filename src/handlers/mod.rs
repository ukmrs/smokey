mod post;
mod settings;
mod test;

use crate::application::{App, Screen};
use crossterm::event::KeyEvent;

pub fn key_handle(key: KeyEvent, app: &mut App) {
    match app.screen {
        Screen::Test => test::handle(key, app),
        Screen::Post => post::handle(key, app),
        Screen::Settings => settings::handle(key, app),
    }
}

pub trait Handler {
    fn handle(key: KeyEvent, app: &mut App);
}
