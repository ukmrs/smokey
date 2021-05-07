use super::SquadChange;
use crate::application::App;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle(key: KeyEvent, app: &mut App) -> Option<SquadChange> {
    match key.code {
        KeyCode::Esc => app.stop(),

        KeyCode::Tab => {
            app.reset_test();
            return Some(SquadChange::StandardTest);
        }

        KeyCode::Char(c) => {
            if let KeyModifiers::CONTROL = key.modifiers {
                if c == 'c' {
                    app.stop();
                }
            }
        }

        _ => (),
    }
    None
}
