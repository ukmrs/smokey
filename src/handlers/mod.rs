mod post;
mod settings;
mod test;

use crate::application::{App, Screen};
use crate::colorscheme::Theme;
use crossterm::event::KeyEvent;

pub fn key_handle<'a>(key: KeyEvent, app: &mut App<'a>, theme: &'a Theme) {
    match app.screen {
        Screen::Test => test::handle(key, app, theme),
        Screen::Post => post::handle(key, app, theme),
        Screen::Settings => settings::handle(key, app, theme),
    }
}

pub trait Handler {
    fn handle<'a>(key: KeyEvent, app: &mut App<'a>, theme: &'a Theme);
}
