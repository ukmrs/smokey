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

pub struct Theme {
    pub done: Color,
    pub wrong: Color,
    pub todo: Color,
}

impl Theme {
    pub fn new() -> Self {
        Theme {
            done: Color::LightCyan,
            wrong: Color::Red,
            todo: Color::DarkGray,
        }
    }
}
