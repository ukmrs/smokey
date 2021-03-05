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
    pub should_quit: bool,

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
            should_quit: false,
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

#[allow(dead_code)]
pub struct TestState<'a> {
    // letter inputs 
    pub done: usize,
    // blanks are unfortuante consequence of appending mistakes
    // at the end of the word
    pub blanks: usize,
    // corrects are 99% not needed
    pub correct: u32,
    pub mistakes: u32,
    pub extras: u32,

    pub current_char: char,
    pub word_amount: u32,

    // TODO time of the first input
    pub begining: Instant,
    // source for generating test
    pub source: String,

    pub text: Vec<Span<'a>>,
    pub test_length: usize,
}

impl<'a> Default for TestState<'a> {
    fn default() -> Self {
        TestState {
            text: vec![],
            begining: Instant::now(),
            done: 0,
            blanks: 0,
            correct: 0,
            extras: 0,
            mistakes: 0,
            source: "./languages/english".to_string(),
            test_length: 0,
            current_char: ' ',
            word_amount: 5,
        }
    }

}
impl<'a> TestState<'a> {
    pub fn calculate_wpm(&self) -> f64 {
        let numerator: f64 = 12. * (self.done - self.blanks) as f64;
        let elapsed = Instant::now().duration_since(self.begining).as_secs() as f64;
        numerator / elapsed
    }

    pub fn restart_test(&mut self, app: &mut App, th: &'a Theme) {
        app.cursor_x = 1;
        self.blanks = 0;
        self.done = 0;
        self.text = prepare_test(&self.source, self.word_amount, th);
        // self.test_text = langs::mock(th);
        self.begining = Instant::now();
        self.mistakes = 0;
        self.current_char = self.text[self.done].content.chars().next().unwrap();
        self.test_length = self.text.len();
    }

    pub fn set_next_char(&mut self){ 
        self.current_char = self.text[self.done].content.chars().next().expect("oof");
    }

    pub fn get_next_char(&mut self) -> Option<char> { 
        self.text[self.done].content.chars().next()
    }

    pub fn fetch(&self, index: usize) -> String {
        self.text[index].content.to_string()
    }

    pub fn change(&mut self, index: usize, item: String) {
        self.text[index].content = Cow::from(item);
    }
}
