use crate::application::App;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle(key: KeyEvent, app: &mut App) {
    match key.code {
        KeyCode::Esc => app.stop(),

        KeyCode::Tab => {
            app.switch_to_test();
            app.reset_test();
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
}
