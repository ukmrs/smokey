use crate::application::{App, TestState};
use crate::colorscheme::ToForeground;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

fn set_next_char_beware_blanks(test: &mut TestState) {
    if let Some(c) = test.get_next_char() {
        test.current_char = c;
    } else {
        test.done += 1;
        test.blanks += 1;
        test.set_next_char();
    }
}

fn set_next_char_or_end(app: &mut App) {
    if app.test.done < app.test.test_length {
        return set_next_char_beware_blanks(&mut app.test);
    }
    app.test.calculate_wpm();
    app.test.end();
    app.change_to_post();
}

/// handles keys during test
pub fn handle(key: KeyEvent, app: &mut App) {
    let test = &mut app.test;
    let theme = app.theme;
    // well doing this in terminal was a bad idea XD
    // Ctrl + Backspace registers as weird thing in terminals
    // I got ctrl(h) and ctrl(7) among others
    // but the ctrl is always there
    // The following code thus interprets everything with ctrl mod except ctrl+c
    // as ctrl + backspace
    // not pretty but it is what is for now
    if let KeyModifiers::CONTROL = key.modifiers {
        if let KeyCode::Char(c) = key.code {
            if c == 'c' {
                app.stop();
                return;
            }
        }

        if test.done == 0 {
            return;
        }

        if test.current_char == ' ' {
            // accounts for extra letters and deletes them
            app.cursor_x -= test.fetch(test.done - 1).len() as u16 + 1;
            test.change(test.done - 1, String::new());
            test.done -= 2;
            test.if_mistake_deduct(test.done, &theme);
            test.text[test.done].style = theme.todo.fg();
            test.blanks -= 1;
        } else if test.fetch(test.done - 1) == " " {
            test.text[test.done - 1].style = theme.todo.fg();
            app.cursor_x -= test.fetch(test.done - 2).len() as u16 + 2;
            test.change(test.done - 2, String::new());
            test.done -= 3;
            test.if_mistake_deduct(test.done, &theme);
            test.text[test.done].style = theme.todo.fg();
            test.blanks -= 1;
        }

        // undoes the test until it deletes a word
        while test.done != 0 && test.fetch(test.done - 1) != " " {
            app.cursor_x -= 1;
            test.done -= 1;
            test.if_mistake_deduct(test.done, &theme);
            test.text[test.done].style = theme.todo.fg();
        }
        test.set_next_char();
        return;
    }

    match key.code {
        KeyCode::Char(c) => {
            app.cursor_x += 1;

            // user pressed the correct key
            if c == test.current_char {
                test.text[test.done].style = theme.done.fg();
                test.done += 1;
                return set_next_char_or_end(app);

            // wrong key
            } else {
                // adds the mistake and the end of the word
                if test.current_char == ' ' {
                    // doesnt count as mistake
                    // but maybe as some sort of extra
                    if test.fetch(test.done - 1).len() < 8 {
                        test.text[test.done - 1].content.to_mut().push(c);
                    } else {
                        // cursor is pushed +1 when KeyCode::Char is matched
                        // well in this rare case nothing happens so it needs to revert
                        app.cursor_x -= 1;
                    }
                // just changes to wrong and moves on
                } else {
                    test.mistakes += 1;
                    test.text[test.done].style = theme.wrong.fg();
                    test.done += 1;
                    return set_next_char_or_end(app);
                }
            }
        }

        // TODO impl ctrl+Backspace
        // CTRL BACKSPACE registers as ctrl 7
        KeyCode::Backspace => {
            if test.done > 0 {
                app.cursor_x -= 1;

                if test.current_char == ' ' {
                    if test.text[test.done - 1].content.is_empty() {
                        test.if_mistake_deduct(test.done - 2, &theme);
                        test.done -= 2;
                        test.blanks -= 1;
                        test.set_next_char();
                        test.text[test.done].style = theme.todo.fg();
                    } else {
                        test.text[test.done - 1]
                            .content
                            .to_mut()
                            .pop()
                            .expect("checked above");
                    }
                } else {
                    test.done -= 1;
                    test.if_mistake_deduct(test.done, &theme);
                    test.set_next_char();
                    test.text[test.done].style = theme.todo.fg();
                }
            }
        }

        KeyCode::Tab => {
            app.reset_test();
        }

        KeyCode::Esc => {
            app.change_to_settings();
        }

        _ => (),
    }
}

#[cfg(test)]
mod tests {
    use crate::application::App;
    use crossterm::event::{KeyCode, KeyEvent};
    use std::thread;
    use std::time::Duration;

    fn generate_key_events_passing_standart_test(app: &App) -> Vec<KeyEvent> {
        app.test
            .text
            .iter()
            .map(|x| x.content.chars().nth(0))
            .filter(|c| c.is_some())
            .map(|c| KeyEvent::from(KeyCode::Char(c.unwrap())))
            .collect()
    }

    fn go_thorugh_test_n_times(n: usize) {
        let mut app = App::setup();
        for _ in 0..n {
            let key_events = generate_key_events_passing_standart_test(&app);

            let klucznik_ptr = app.klucznik as usize;

            for kv in key_events {
                app.handle_key_event(kv);
            }

            // key_handler should be changed by now
            // as after the completed test the app should land itself in the post screen
            assert_ne!(klucznik_ptr, app.klucznik as usize);

            app.handle_key_event(KeyEvent::from(KeyCode::Tab))
        }
    }

    #[test]
    fn go_thorugh_test_five_times() {
        go_thorugh_test_n_times(5)
    }

    #[test]
    #[ignore]
    fn go_thorugh_test_100_times() {
        go_thorugh_test_n_times(100)
    }

    // Testing results of typing test
    // TODO: Accuracy and such

    fn wpm_to_char_delay<T: Into<f64>>(wpm: T) -> Duration
    where
        f64: From<T>,
    {
        Duration::from_secs_f64(12. / f64::from(wpm))
    }

    // this tests are hardware/os dependent
    // which make them potentially bad
    fn wpm_test_setup(wpm: f64) {
        let delay = wpm_to_char_delay(wpm);

        let mut app = App::setup();
        let key_events = generate_key_events_passing_standart_test(&app);

        for kv in key_events {
            thread::sleep(delay);
            app.handle_key_event(kv);
        }

        let final_wpm = app.test.hoarder.final_wpm;
        // final_wmp can be lower by a thin margin given in the toleranca variable
        // than the requested wpm, but it can never be higher
        // as std::thread::sleep is guaranteed to sleep for at least the specified duration
        let tolerated = 0.99 * wpm;
        assert!(final_wpm < wpm);
        assert!(final_wpm > tolerated);
    }

    #[test]
    #[ignore]
    fn test_wpm_counting() {
        wpm_test_setup(60.);
        wpm_test_setup(140.);
        wpm_test_setup(220.);
    }

    // Testing Backspace

    #[test]
    fn test_backspace_at_the_test_begining() {
        let mut app = App::setup();
        for _ in 0..20 {
            app.handle_key_event(KeyEvent::from(KeyCode::Backspace))
        }
        assert_eq!(app.test.done, 0);

        app.handle_key_event(KeyEvent::from(KeyCode::Char('ź')));
        assert_eq!(app.test.done, 1);
        app.handle_key_event(KeyEvent::from(KeyCode::Backspace));
        assert_eq!(app.test.done, 0);

        for _ in 0..20 {
            app.handle_key_event(KeyEvent::from(KeyCode::Char('ź')));
            app.handle_key_event(KeyEvent::from(KeyCode::Backspace));
        }
        assert_eq!(app.test.done, 0);
    }
}
