use crate::application::App;
use crate::colorscheme::Theme;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn handle<'a>(
    key: KeyEvent,
    app: &mut App,
    theme: &'a Theme,
) {
    match key.code {
        KeyCode::Esc => app.should_quit = true,

        KeyCode::Tab => {
            app.switch_to_test();
            app.reset_test(theme);
        }

        KeyCode::Char(c) => {
            if let KeyModifiers::CONTROL = key.modifiers {
                if c == 'c' {
                    app.should_quit = true;
                    return;
                }
            }

            match c {
                'h' => {}
                'j' => {}
                'k' => {}
                'l' => {}
                'q' => app.should_quit = true,
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
