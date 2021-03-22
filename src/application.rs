//! This mod keeps tabs that on the state of the app
//! as well as current typing test
//! main structs App and TestState

use crate::colorscheme;
use crate::langs;

use colorscheme::Theme;
use langs::prepare_test;

use std::time::Instant;
use tui::text::Span;

use std::borrow::Cow;

pub enum Screen {
    Test,
    Post,
}

pub struct App {
    pub screen: Screen,
    pub should_quit: bool,
    pub cursor_x: u16,
    pub margin: u16,
}

impl App {
    pub fn new() -> Self {
        App {
            screen: Screen::Test,
            should_quit: false,
            cursor_x: 1,
            margin: 2,
        }
    }
    pub fn end_test(&mut self) {
        self.screen = Screen::Post;
    }
}

/// keeps track of wpms roughly every second
/// absolute precisiion is not important here
#[derive(Debug)]
pub struct WpmHoarder {
    // If decide against it there is spin-sleep,
    // crossterm events can be wrapped in a channel
    // would be interesting for sure
    pub wpms: Vec<f64>,
    pub capacity: usize,
    pub seconds: u64,
}

impl WpmHoarder {
    fn new(capacity: usize) -> Self {
        WpmHoarder {
            capacity,
            wpms: Vec::with_capacity(capacity),
            seconds: 1,
        }
    }

    fn reset(&mut self) {
        self.wpms.clear();
        self.seconds = 1;
    }

    fn is_due(&mut self, begining: Instant) -> bool {
        let elapsed = begining.elapsed().as_secs();
        let due_time = self.seconds * (self.wpms.len() as u64 + 1);
        elapsed >= due_time
    }

    fn push(&mut self, wpm: f64) {
        self.wpms.push(wpm);
        if self.wpms.len() == self.capacity {
            let new_len: usize = self.wpms.len() / 2;
            for i in 0..(self.wpms.len() / 2) {
                self.wpms[i] = (self.wpms[i + 1] + self.wpms[i]) / 2.;
            }
            self.wpms.resize(new_len, 0.);
            self.seconds *= 2;
        }
    }

    pub fn get_min_and_max(&self) -> (f64, f64) {
        let mut min: f64 = self.wpms[0];
        let mut max: f64 = min;
        for wpm in &self.wpms[1..] {
            if *wpm < min {
                min = *wpm
            } else if *wpm > max {
                max = *wpm
            }
        }
        (min, max)
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
    pub hoarder: WpmHoarder,
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
            word_amount: 15,
            hoarder: WpmHoarder::new(32),
        }
    }
}

impl<'a> TestState<'a> {
    pub fn calculate_wpm(&self) -> f64 {
        let numerator: f64 = 12. * (self.done - self.blanks) as f64;
        let elapsed = Instant::now().duration_since(self.begining).as_secs() as f64;
        numerator / elapsed
    }

    pub fn reset(&mut self, app: &mut App, th: &'a Theme) {
        app.cursor_x = 1;
        self.blanks = 0;
        self.done = 0;
        self.text = prepare_test(&self.source, self.word_amount, th);
        // self.text = langs::mock(th);
        self.begining = Instant::now();
        self.mistakes = 0;
        self.current_char = self.text[self.done].content.chars().next().unwrap();
        self.test_length = self.text.len();
        debug!("{:?}", self.hoarder);
        self.hoarder.reset();
    }

    pub fn update_wpm_history(&mut self) {
        if self.hoarder.is_due(self.begining) {
            self.hoarder.push(self.calculate_wpm());
        }
    }

    // this section feels awful
    // aaaaaah
    pub fn set_next_char(&mut self) {
        self.current_char = self.text[self.done].content.chars().next().expect("oof");
    }

    pub fn get_next_char(&mut self) -> Option<char> {
        self.text[self.done].content.chars().next()
    }

    pub fn fetch(&self, index: usize) -> &str {
        self.text[index].content.as_ref()
    }

    pub fn change(&mut self, index: usize, item: String) {
        self.text[index].content = Cow::from(item);
    }
}
