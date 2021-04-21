mod post;
mod settings;
mod test;

use crate::application::{App, Screen, TestState};
use crate::colorscheme::Theme;
use crossterm::event::KeyEvent;

pub fn key_handle<'a>(key: KeyEvent, app: &mut App, test: &mut TestState<'a>, theme: &'a Theme) {
    match app.screen {
        Screen::Test => test::handle(key, app, test, theme),
        Screen::Post => post::handle(key, app, test, theme),
        Screen::Settings => settings::handle(key, app, test, theme),
    }
}
