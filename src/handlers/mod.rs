mod post;
mod settings;
mod test;

use crate::application::App;
use crate::painters::*;
use crossterm::event::KeyEvent;

pub type KeyHandler = fn(KeyEvent, &mut App) -> Option<SquadChange>;

#[derive(Copy, Clone, Debug)]
pub enum SquadChange {
    StandardTest,
    Post,
    Settings,
}

impl SquadChange {
    pub fn to_squad(&self) -> Squad {
        match self {
            Self::StandardTest => Squad::new(test::handle, draw_test_and_update),
            Self::Post => Squad::new(post::handle, draw_post),
            Self::Settings => Squad::new(settings::handle, draw_settings),
        }
    }
}

#[derive(Copy, Clone)]
pub struct Squad {
    pub key_handler: KeyHandler,
    pub painter: Option<Painter>,
}

impl Squad {
    fn new(key_handler: KeyHandler, painter: Painter) -> Self {
        Self {
            key_handler,
            painter: Some(painter),
        }
    }
}

impl Default for Squad {
    fn default() -> Self {
        Self {
            key_handler: test::handle,
            painter: Some(draw_test_and_update),
        }
    }
}
