use crate::application::{App, Screen, TestState};
use crate::colorscheme::Theme;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle<'a>(key: KeyEvent, app: &mut App, test: &mut TestState<'a>, theme: &'a Theme) {
    match key.code {
        KeyCode::Esc => app.should_quit = true,

        KeyCode::Tab => {
            app.screen = Screen::Test;
            test.reset(app, theme);
        }

        KeyCode::Char(c) => {
            if let KeyModifiers::CONTROL = key.modifiers {
                if c == 'c' {
                    app.should_quit = true;
                }
            }
        }
        _ => (),
    }
}
