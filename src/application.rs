//! This mod keeps tabs that on the state of the app
//! as well as current typing test
//! main structs App and TestState

use crate::colorscheme;
use crate::handlers;
use crate::handlers::KeyHandler;
use crate::painters::*;
use crate::settings::Settings;
use crate::Term;
use crossterm::event::KeyEvent;
use std::path::PathBuf;

use colorscheme::Theme;
use directories_next::ProjectDirs;

use crate::typer::TestState;

pub const APPLOGO: &str = " _._ _  _ |  _    
_>| | |(_)|<(/_\\/ 
               /  ";

pub struct App<'t> {
    pub settings: Settings,
    pub test: TestState<'t>,
    pub theme: Theme,
    pub margin: u16,
    pub paragraph: u16,
    pub config: Config,

    pub klucznik: KeyHandler,
    pub painter: Painter,

    pub is_alive: bool,
}

impl<'t> App<'t> {
    /// returns App instance with initialized test
    /// basically ready to use
    /// perhaps this will become the new function
    pub fn setup() -> Self {
        let mut app = Self::default();
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
    /// let mut app = App::setup();
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
        (self.klucznik)(key_event, self)
    }

    pub fn change_to_post(&mut self) {
        self.painter = draw_post;
        self.klucznik = handlers::post::handle;
    }

    pub fn change_to_settings(&mut self) {
        self.painter = draw_settings;
        self.klucznik = handlers::settings::handle;
    }

    pub fn change_to_test(&mut self) {
        self.painter = draw_test_and_update;
        self.klucznik = handlers::typer::handle;
    }

    pub fn stop(&mut self) {
        self.is_alive = false;
    }

    pub fn reset_test(&mut self) {
        self.test.cursor_x = 1;
        self.test.reset(&self.config);
    }
}

impl<'t> Default for App<'t> {
    /// Creates App instance
    /// the test isnt initialized though
    fn default() -> Self {
        let config = Config::default();
        let settings = Settings::new(&PathBuf::from(config.source.clone()));

        Self {
            test: TestState::default(),
            theme: Theme::default(),
            is_alive: true,
            margin: 2,
            paragraph: 62,
            config,
            settings,
            /// unwrap wont painc because the Squad Default always returns Some
            painter: draw_test_and_update,
            klucznik: handlers::typer::handle,
        }
    }
}

pub struct Config {
    words: PathBuf,
    pub source: String,
    pub length: usize,
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
            length: 30,
            freq_cut_off: 10_000,
        }
    }
}

impl Config {
    pub fn get_source(&self) -> PathBuf {
        self.words.join(&self.source)
    }
}
