use crate::settings::SettingsColors;
use crate::typer::TestColors;
use tui::style::{Color, Style};

pub trait ToForeground {
    fn fg(self) -> Style;
}

impl ToForeground for Color {
    fn fg(self) -> Style {
        Style::default().fg(self)
    }
}

#[derive(Debug, Clone, Copy)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Theme {
    pub done: Color,
    pub mistake: Color,
    pub todo: Color,
    pub hover: Color,
    pub active: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            done: Color::White,
            mistake: Color::Red,
            todo: Color::Gray,
            hover: Color::Magenta,
            active: Color::Green,
        }
    }
}

impl Theme {
    pub fn to_settings_colors(self) -> SettingsColors {
        SettingsColors {
            hover: self.hover,
            active: self.active,
        }
    }
    pub fn to_test_colors(self) -> TestColors {
        TestColors {
            todo: self.todo,
            done: self.done,
            wrong: self.mistake,
        }
    }
}
