//! obviously underdeveloped theming department
//! main struct: Theme
//! for now there is nothing there 
//! but maybe in the future there will be some sort of colorschemes

use tui::style::{Color, Style};

pub struct Theme {
    pub wrong_color: Color,
    pub done: Style,
    pub wrong: Style,
    pub todo: Style,
}

impl Theme {
    pub fn new() -> Self {
        let wrongc = Color::Red;
        Theme {
            wrong_color: wrongc,
            done: Style::default().fg(Color::LightCyan),
            wrong: Style::default().fg(wrongc),
            todo: Style::default().fg(Color::DarkGray),
        }
    }
}
