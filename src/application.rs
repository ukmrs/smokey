use crate::colorscheme;
use crate::langs;

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
    pub cursor_x: u16,
    pub mistakes: u32,
}

impl<'a> App<'a> {
    pub fn create_app() -> Self {
        App {
            test_text: vec![],
            begining: Instant::now(),
            done: 0,
            source: "./languages/english".to_string(),
            test_length: 15,
            cursor_x: 1,
            mistakes: 0,
        }
    }

    pub fn start_timer(&mut self) {
        self.begining = Instant::now();
    }

    pub fn restart_test(
        &mut self,
        th: &'a Theme,
        current_char: &mut char,
        test_length: &mut usize,
    ) {
        self.done = 0;
        self.cursor_x = 1;
        self.test_text = prepare_test(&self.source, self.test_length, th);
        // self.test_text = langs::mock(th);
        self.begining = Instant::now();
        self.mistakes = 0;
        *current_char = self.test_text[self.done].content.chars().next().unwrap();
        *test_length = self.test_text.len();
    }

    pub fn calculate_wpm(&self) -> f64 {
        let numerator: f64 = 12.0 * ((self.done) as f64 / 2.0);
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
