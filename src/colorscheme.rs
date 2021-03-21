use tui::style::{Color, Style};

pub struct Theme {
    pub done: Style,
    pub wrong: Style,
    pub todo: Style,
}

impl Theme {
    pub fn new() -> Self {
        Theme {
            done: Style::default().fg(Color::LightCyan),
            wrong: Style::default().fg(Color::LightRed),
            todo: Style::default().fg(Color::DarkGray),
        }
    }
}
