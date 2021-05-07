//! This mod keeps tabs that on the state of the app
//! as well as current typing test
//! main structs App and TestState

use crate::colorscheme;
use crate::handlers::{KeyHandler, SquadChange};
use crate::langs;
use crate::painters::*;
use crate::vec_of_strings;
use crate::Term;
use crossterm::event::KeyEvent;
use std::path::{Path, PathBuf};

use colorscheme::Theme;
use directories_next::ProjectDirs;
use langs::{prepare_test, Punctuation};

use std::time::Instant;
use tui::text::Span;

use crate::utils::StatefulList;
use std::borrow::Cow;

pub const APPLOGO: &'static str = " _._ _  _ |  _    
_>| | |(_)|<(/_\\/ 
               /  ";

pub struct App<'t> {
    pub settings: Settings,
    pub test: TestState<'t>,
    pub theme: Theme,
    pub cursor_x: u16,
    pub margin: u16,
    pub config: Config,

    pub klucznik: KeyHandler,
    pub painter: Painter,

    pub is_alive: bool,
}

impl<'t> App<'t> {
    /// Creates App instance
    /// the test isnt initialized though
    pub fn new() -> Self {
        let config = Config::default();
        let settings = Settings::new(&PathBuf::from(config.source.clone()));

        let posse = SquadChange::StandardTest.to_squad();

        Self {
            test: TestState::default(),
            theme: Theme::new(),
            is_alive: true,
            cursor_x: 1,
            margin: 2,
            config,
            settings,
            /// unwrap wont painc because the Squad Default always returns Some
            painter: posse.painter.unwrap(),
            klucznik: posse.key_handler,
        }
    }

    /// returns App instance with initialized test
    /// basically ready to use
    /// perhaps this will become the new function
    pub fn setup() -> Self {
        let mut app = Self::new();
        app.reset_test();
        app
    }

    /// Paints to the screen using current painter
    pub fn paint(&mut self, terminal: &mut Term) {
        (self.painter)(terminal, self)
    }

    /// Performs an action based on KeyEvent
    /// Such action may call for changing keyhandler and painter
    /// which is also performed in the scope of this function
    /// ```
    /// use crossterm::event::{KeyCode, KeyEvent};
    /// use smokey::application::App;
    /// // app starts on the test Screen
    /// let mut app = App::new();
    /// app.reset_test();
    ///
    /// // q in this context just counts toward the test
    /// app.handle_key_event(KeyEvent::from(KeyCode::Char('q')));
    /// assert_eq!(app.test.done, 1);
    /// assert!(app.is_alive);
    ///
    /// // Esc should go back to the settings
    /// app.handle_key_event(KeyEvent::from(KeyCode::Esc));
    /// // now q char is handled differently -> (quit app)
    /// app.handle_key_event(KeyEvent::from(KeyCode::Char('q')));
    /// assert!(!app.is_alive);
    /// ```
    pub fn handle_key_event(&mut self, key_event: KeyEvent) {
        if let Some(kh) = (self.klucznik)(key_event, self) {
            let squad = kh.to_squad();
            self.klucznik = squad.key_handler;

            if let Some(painter) = squad.painter {
                self.painter = painter;
            }
        }
    }

    pub fn stop(&mut self) {
        self.is_alive = false;
    }

    pub fn reset_test(&mut self) {
        self.cursor_x = 1;
        self.test.reset(&self.config, &self.theme);
    }
}

pub struct Settings {
    pub length_list: StatefulList<String>,
    pub frequency_list: StatefulList<String>,
    pub words_list: StatefulList<String>,
    pub mods_list: StatefulList<String>,
}

impl Settings {
    fn new(path: &Path) -> Self {
        let length_list = StatefulList::with_items(vec_of_strings!["10", "15", "25", "50", "100"]);
        let frequency_list =
            StatefulList::with_items(vec_of_strings!["100", "1k", "5k", "10k", "max"]);
        let words_list: Vec<String> = path
            .iter()
            .map(|i| i.to_string_lossy().to_string())
            .collect();
        let mod_list = vec_of_strings!["Punctuation"];

        Self {
            length_list,
            frequency_list,
            words_list: StatefulList::with_items(words_list),
            mods_list: StatefulList::with_items(mod_list),
        }
    }
}

#[derive(Default)]
pub struct TestType {
    pub punctuation: Option<Punctuation>,
}

pub struct Config {
    words: PathBuf,
    pub source: String,
    pub length: usize,
    pub test_type: TestType,
    pub freq_cut_off: usize,
}

impl Default for Config {
    fn default() -> Self {
        let base = ProjectDirs::from("pl", "ukmrs", "smokey")
            .unwrap()
            .data_dir()
            .to_path_buf();

        Config {
            words: base.join("words"),
            source: String::from("english"),
            length: 10,
            test_type: TestType::default(),
            freq_cut_off: 10_000,
        }
    }
}

impl Config {
    pub fn get_source(&self) -> PathBuf {
        self.words.join(&self.source)
    }
}

/// keeps track of wpms roughly every second
/// absolute precisiion is not important here
#[derive(Debug)]
pub struct WpmHoarder {
    pub wpms: Vec<f64>,
    pub capacity: usize,
    pub seconds: u64,
    pub final_wpm: f64,
}

impl WpmHoarder {
    fn new(capacity: usize) -> Self {
        WpmHoarder {
            capacity,
            wpms: Vec::with_capacity(capacity),
            seconds: 1,
            final_wpm: 0.,
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

    // delet this
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
    pub mistakes: u32,

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
            mistakes: 0,
            source: "storage/words/english".to_string(),
            test_length: 0,
            current_char: ' ',
            word_amount: 15,
            hoarder: WpmHoarder::new(32),
        }
    }
}

impl<'a> TestState<'a> {
    pub fn calculate_wpm(&self) -> f64 {
        let numerator: f64 = 12. * (self.done - self.blanks - self.mistakes as usize) as f64;
        let elapsed = Instant::now().duration_since(self.begining).as_secs_f64();
        numerator / elapsed
    }

    pub fn reset(&mut self, config: &Config, th: &Theme) {
        self.blanks = 0;
        self.done = 0;
        self.text = prepare_test(config, th);
        self.begining = Instant::now();
        self.mistakes = 0;
        self.current_char = self.text[self.done].content.chars().next().unwrap();
        self.test_length = self.text.len();
        self.hoarder.reset();
    }

    pub fn end(&mut self) {
        self.hoarder.final_wpm = self.calculate_wpm();
    }

    pub fn update_wpm_history(&mut self) {
        if self.hoarder.is_due(self.begining) {
            self.hoarder.push(self.calculate_wpm());
        }
    }

    /// chekcs if char is a mistake and deducts it from
    /// the total count
    pub fn if_mistake_deduct(&mut self, index: usize, th: &Theme) {
        if th.wrong == self.text[index].style.fg.unwrap() {
            self.mistakes -= 1;
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
