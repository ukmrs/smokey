pub mod post;
pub mod settings;
pub mod typer;

use crate::application::App;
use crossterm::event::KeyEvent;

pub type KeyHandler = fn(KeyEvent, &mut App);
