use crate::application::App;
use crate::colorscheme::Theme;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle<'a>(
    key: KeyEvent,
    app: &mut App,
    _theme: Theme,
) {
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
                    return;
                }
            }

            match c {
                'h' => {}
                'j' => {}
                'k' => {}
                'l' => {}
                'q' => app.stop(),
                _ => {}
            }
        }

        KeyCode::Left => {}
        KeyCode::Down => {}
        KeyCode::Up => {}
        KeyCode::Right => {}

        _ => (),
    }
}
