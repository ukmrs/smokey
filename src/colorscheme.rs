//! obviously underdeveloped theming department
//! main struct: Theme
//! for now there is nothing there
//! but maybe in the future there will be some sort of colorschemes

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
