use crate::langs;
use crate::colorscheme;

use colorscheme::Theme;
use langs::prepare_test;

use std::time::{Duration, Instant};
use tui::{
    style::{Color, Modifier, Style},
    text::Span,
};

use std::borrow::Cow;

pub struct App<'a> {
    pub begining: Instant,

    pub done: usize,
    pub source: String,
    pub test_text: Vec<Span<'a>>,
    pub test_length: u32,

}

impl<'a> App<'a> {
    pub fn create_app() -> Self {
        App {
            test_text: vec![],
            begining: Instant::now(),
            done: 1,
            source: "./languages/english".to_string(),
            test_length: 15,
        }
    }

    pub fn start_timer(&mut self) {
        self.begining = Instant::now();
    }

    pub fn restart_test(&mut self, th: &'a Theme) {
        self.done = 1;
        self.test_text = prepare_test(&self.source, self.test_length, th);
        self.begining = Instant::now();
    }


    pub fn calculate_wpm(&self) -> f64 {
        let numerator: f64 = 12.0 * ((self.done - 1) as f64 / 2.0);
        let elapsed = Instant::now().duration_since(self.begining).as_secs() as f64;
        numerator / elapsed
    }

    pub fn fetch_content(&self, index: usize) -> String {
        self.test_text[index].content.to_string()
    }

    pub fn change_content(&mut self, index: usize, item: String) {
        self.test_text[index].content = Cow::from(item);
    }
}
