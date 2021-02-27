use tui::{
    style::{Color, Modifier, Style},
    text::Span,
};
use std::time::{Instant, Duration};
use std::borrow::Cow;

pub struct App<'a> {
    pub wrongcolor: Style,
    pub todocolor: Style,
    pub donecolor: Style,
    pub test_text: Vec<Span<'a>>,
    // time keeping
    pub begining: Instant,
    pub done: usize,
}

impl<'a> App<'a> {
    pub fn default() -> Self {
        App {
            donecolor: Style::default().fg(Color::LightCyan),
            wrongcolor: Style::default().fg(Color::LightRed),
            todocolor: Style::default().fg(Color::DarkGray),
            test_text: vec![],
            begining: Instant::now(),
            done: 1,
        }
    }

    pub fn start(&mut self) {
        self.begining = Instant::now();
    }

    pub fn calculate_wpm(&self) -> f64 {
        let numerator: f64 = 60.0 * ((self.done as f64 - 1.0) / 2.0) / 5.0;
        let elapsed = Instant::now().duration_since(self.begining).as_secs() as f64;
        numerator / elapsed
}
pub fn fetch_content(&self, index: usize) -> String {
    self.test_text[index].content.to_string()
}

pub fn change_content(&mut self, index: usize, item: String){
    self.test_text[index].content = Cow::from(item);
}
}
