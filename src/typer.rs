use crate::application::Config;
use crate::colorscheme::Theme;
use crate::colorscheme::ToForeground;
use crate::langs::prepare_test;
use std::time::Instant;
use tui::style::Color;
use tui::text::Span;

pub struct WpmHoarder {
    pub wpms: Vec<f64>,
    pub capacity: usize,
    pub seconds: u64,
    pub final_wpm: f64,
    pub final_acc: f64,
}

impl WpmHoarder {
    fn new(capacity: usize) -> Self {
        WpmHoarder {
            capacity,
            wpms: Vec::with_capacity(capacity),
            seconds: 1,
            final_wpm: 0.,
            final_acc: 0.,
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

    pub fn get_max_wpm(&self) -> f64 {
        let mut max: f64 = self.wpms[0];

        for wpm in &self.wpms[1..] {
            if *wpm > max {
                max = *wpm
            }
        }
        max
    }
}

pub struct TestState<'a> {
    // letter inputs
    pub done: usize,

    // blanks are unfortuante consequence of appending mistakes
    // at the end of the word
    // Blanks kinda suck
    // TODO implement a better system than this
    pub blanks: usize,

    pub mistakes: usize,
    pub pmiss: usize,

    pub cursor_x: u16,
    pub current_char: char,

    // TODO time of the first input
    pub begining: Instant,
    // source for generating test
    pub source: String,

    pub text: Vec<Span<'a>>,
    pub test_length: usize,
    pub hoarder: WpmHoarder,

    cwrong: Color,
    ctodo: Color,
    cdone: Color,
}

impl<'a> Default for TestState<'a> {
    fn default() -> Self {
        let th = Theme::default();

        TestState {
            text: vec![],
            begining: Instant::now(),
            done: 0,
            blanks: 0,
            mistakes: 0,
            // persistent mistakes
            pmiss: 0,
            cursor_x: 0,

            source: "storage/words/english".to_string(),
            test_length: 0,
            current_char: ' ',
            hoarder: WpmHoarder::new(32),

            cwrong: th.wrong,
            ctodo: th.todo,
            cdone: th.done,
        }
    }
}

impl<'a> TestState<'a> {
    pub fn calculate_wpm(&self) -> f64 {
        let numerator: f64 = 12. * (self.done - self.blanks - self.mistakes as usize) as f64;
        let elapsed = Instant::now().duration_since(self.begining).as_secs_f64();
        numerator / elapsed
    }

    pub fn reset(&mut self, config: &Config) {
        self.blanks = 0;
        self.done = 0;
        self.pmiss = 0;
        self.mistakes = 0;

        self.text = prepare_test(config, self.cwrong, self.ctodo);
        self.begining = Instant::now();
        self.current_char = self.text[self.done].content.chars().next().unwrap();
        self.test_length = self.text.len();
        self.hoarder.reset();
    }

    pub fn end(&mut self) {
        self.hoarder.final_wpm = self.calculate_wpm();
        self.hoarder.final_acc = {
            let correct = (self.done - self.blanks - self.mistakes) as f64;
            let key_presses = correct + self.pmiss as f64;
            correct / key_presses * 100.
        };

        debug!("{}", self.hoarder.final_acc);
    }

    pub fn update_wpm_history(&mut self) {
        if self.hoarder.is_due(self.begining) {
            self.hoarder.push(self.calculate_wpm());
        }
    }

    /// chekcs if char is a mistake and deducts it from
    /// the total count
    pub fn if_mistake_deduct(&mut self, index: usize) {
        if self.cwrong == self.text[index].style.fg.unwrap() {
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
        *self.text[index].content.to_mut() = item;
    }

    // character response

    fn set_next_char_beware_blanks(&mut self) {
        if let Some(c) = self.get_next_char() {
            self.current_char = c;
        } else {
            self.done += 1;
            self.blanks += 1;
            self.set_next_char();
        }
    }

    fn set_next_char_or_end(&mut self) -> bool {
        if self.done < self.test_length {
            self.set_next_char_beware_blanks();
            return false;
        }
        self.calculate_wpm();
        self.end();
        true
    }

    pub fn on_char(&mut self, c: char) -> bool {
        self.cursor_x += 1;
        if c == self.current_char {
            self.text[self.done].style = self.cdone.fg();
            self.done += 1;
            return self.set_next_char_or_end();
        }

        // wrong key
        // adds the mistake and the end of the word
        if self.current_char == ' ' {
            // doesnt count as mistake
            // but maybe as some sort of extra
            self.pmiss += 1;
            if self.fetch(self.done - 1).len() < 4 {
                self.text[self.done - 1].content.to_mut().push(c);
            } else {
                // cursor is pushed +1 when KeyCode::Char is matched
                // well in this rare case nothing happens so it needs to revert
                self.cursor_x -= 1;
            }
        // just changes to wrong and moves on
        } else {
            self.mistakes += 1;
            self.pmiss += 1;
            self.text[self.done].style = self.cwrong.fg();
            self.done += 1;
            return self.set_next_char_or_end();
        }
        false
    }

    // BACKSPACE undo_char, undo_word

    // undo word

    fn undo_space_char_and_extras(&mut self) {
        self.cursor_x -= self.fetch(self.done - 1).len() as u16 + 1;
        self.change(self.done - 1, String::new());
        self.done -= 2;

        self.if_mistake_deduct(self.done);
        self.text[self.done].style = self.ctodo.fg();
        self.blanks -= 1;
    }

    pub fn undo_word(&mut self) {
        if self.current_char == ' ' {
            self.undo_space_char_and_extras();
        } else if self.fetch(self.done - 1) == " " {
            self.done -= 1;
            self.cursor_x -= 1;
            self.text[self.done].style = self.ctodo.fg();

            self.undo_space_char_and_extras();
        }

        while self.done != 0 && self.fetch(self.done - 1) != " " {
            self.cursor_x -= 1;
            self.done -= 1;
            self.if_mistake_deduct(self.done);
            self.text[self.done].style = self.ctodo.fg();
        }
    }

    // undo char
    //
    pub fn undo_char(&mut self) {
        if self.done > 0 {
            self.cursor_x -= 1;

            if self.current_char == ' ' {
                if self.text[self.done - 1].content.is_empty() {
                    self.if_mistake_deduct(self.done - 2);
                    self.done -= 2;
                    self.blanks -= 1;
                    self.set_next_char();
                    self.text[self.done].style = self.ctodo.fg();
                } else {
                    // shaves off one from extras
                    self.text[self.done - 1]
                        .content
                        .to_mut()
                        .pop()
                        .expect("checked above");
                }
            } else {
                self.done -= 1;
                self.if_mistake_deduct(self.done);
                self.set_next_char();
                self.text[self.done].style = self.ctodo.fg();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::Config;

    fn get_wrong_char(c: char) -> char {
        if c == 'ź' {
            return 'a';
        }
        'ź'
    }

    fn setup_new_test() -> TestState<'static> {
        let config = Config::default();
        let mut test = TestState::default();
        test.reset(&config);
        test
    }

    #[test]
    fn test_undo_char() {
        let mut test = setup_new_test();

        // test deletes a single mistake
        test.on_char(get_wrong_char(test.current_char));
        assert_eq!(test.done, 1);
        assert_eq!(test.mistakes, 1);
        test.undo_char();
        assert_eq!(test.done, 0);
        assert_eq!(test.mistakes, 0);

        // test doesn't do anything at the beginning
        test.undo_char();
        assert_eq!(test.done, 0);
        assert_eq!(test.mistakes, 0);

        // test deletes extras
        while test.current_char != ' ' {
            test.on_char(test.current_char);
        }
        let done = test.done;
        test.on_char(get_wrong_char(test.current_char));
        assert!(!test.fetch(done - 1).is_empty());
        test.undo_char();
        assert!(test.fetch(done - 1).is_empty());
    }
}
