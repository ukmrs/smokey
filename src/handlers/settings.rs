use crate::application::App;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle(key: KeyEvent, app: &mut App) {
    match key.code {
        KeyCode::Esc => {
            if app.settings.escape() {
                app.stop();
            }
        }

        KeyCode::Tab => {
            app.reset_test();
            app.change_to_test();
        }

        KeyCode::Char(c) => {
            if let KeyModifiers::CONTROL = key.modifiers {
                if c == 'c' {
                    app.stop();
                    return;
                }
            }

            match c {
                'h' => app.settings.left(),
                'j' => app.settings.down(),
                'k' => app.settings.up(),
                'l' => app.settings.right(),
                'q' => app.stop(),
                _ => {}
            }
        }

        KeyCode::Left => app.settings.left(),
        KeyCode::Down => app.settings.down(),
        KeyCode::Up => app.settings.up(),
        KeyCode::Right => app.settings.right(),

        KeyCode::Enter => app.settings.enter(),
        _ => (),
    }
}
