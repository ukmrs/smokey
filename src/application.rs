//! This mod keeps tabs that on the state of the app
//! as well as current typing test
//! main structs App and TestState

use crate::config;
use crossterm::event::KeyEvent;

use crate::handlers::{self, KeyHandler};
use crate::painters::*;
use crate::settings::Settings;
use crate::typer::TestState;
use crate::Term;

pub const APPLOGO: &str = " _._ _  _ |  _    
_>| | |(_)|<(/_\\/ 
               /  ";

pub struct App<'t> {
    pub settings: Settings,
    pub test: TestState<'t>,
    pub margin: u16,
    pub paragraph: u16,
    pub key_handler: KeyHandler,
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
        (self.key_handler)(key_event, self)
    }

    pub fn change_to_post(&mut self) {
        self.painter = draw_post;
        self.key_handler = handlers::post::handle;
    }

    pub fn change_to_settings(&mut self) {
        self.painter = draw_settings;
        self.key_handler = handlers::settings::handle;
    }

    pub fn change_to_test(&mut self) {
        self.painter = draw_test_and_update;
        self.key_handler = handlers::typer::handle;
    }

    pub fn stop(&mut self) {
        self.is_alive = false;
    }

    pub fn reset_test(&mut self) {
        self.test.cursor_x = 1;
        self.test.reset(&self.settings.test_cfg);
    }

    pub fn from_config() -> Self {
        let final_config = config::get_final_config();
        let test = TestState::with_colors(final_config.theme.to_test_colors());
        let settings = Settings::with_config(
            final_config.theme.to_settings_colors(),
            final_config.typing_test_config,
        );

        Self {
            settings,
            test,
            ..Self::default()
        }
    }
}

impl<'t> Default for App<'t> {
    /// Creates App instance
    /// the test isnt initialized though
    fn default() -> Self {
        Self {
            test: TestState::default(),
            is_alive: true,
            margin: 2,
            paragraph: 62,
            settings: Settings::default(),
            /// unwrap wont painc because the Squad Default always returns Some
            painter: draw_test_and_update,
            key_handler: handlers::typer::handle,
        }
    }
}
