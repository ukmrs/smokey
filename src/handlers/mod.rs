mod post;
mod settings;
mod test;

use crate::application::{App, Screen};
use crate::painters::*;
use crossterm::event::KeyEvent;

pub fn key_handle(key: KeyEvent, app: &mut App) {
    match app.screen {
        Screen::Test => test::handle(key, app),
        Screen::Post => post::handle(key, app),
        Screen::Settings => settings::handle(key, app),
    };
}

pub type Respondent = fn(KeyEvent, &mut App) -> Option<KeyHandler>;

#[derive(Copy, Clone, Debug)]
pub enum KeyHandler {
    StandardTest,
    Post,
    Settings,
}

impl KeyHandler {
    pub fn to_squad(&self) -> Squad {
        match self {
            Self::StandardTest => Squad::new(test::handle, draw_test),
            Self::Post => Squad::new(post::handle, draw_post),
            Self::Settings => Squad::new(settings::handle, draw_settings),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Squad {
    pub respondent: Respondent,
    pub painter: Option<Painter>,
}

impl Squad {
    fn new(respondent: Respondent, painter: Painter) -> Self {
        Self {
            respondent,
            painter: Some(painter),
        }
    }
}

impl Default for Squad {
    fn default() -> Self {
        Self {
            respondent: test::handle,
            painter: Some(draw_test_and_update),
        }
    }
}
