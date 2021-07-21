use lazy_static::lazy_static;
use tui::style::{Color, Style};

lazy_static! {
    pub static ref THEME: Theme = Theme::default();
}

pub trait ToForeground {
    fn fg(self) -> Style;
}

impl ToForeground for Color {
    fn fg(self) -> Style {
        Style::default().fg(self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Theme {
    pub done: Color,
    pub wrong: Color,
    pub todo: Color,
    pub hover: Color,
    pub active: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            done: Color::White,
            wrong: Color::Red,
            todo: Color::Gray,
            hover: Color::Magenta,
            active: Color::Green,
        }
    }
}
