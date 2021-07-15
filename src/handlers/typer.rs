use crate::application::App;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// handles keys during test
pub fn handle(key: KeyEvent, app: &mut App) {
    let test = &mut app.test;
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

        test.undo_word();
        test.set_next_char();

        return;
    }

    match key.code {
        KeyCode::Char(c) => {
            if test.on_char(c) {
                app.change_to_post();
            }
        }

        KeyCode::Backspace => test.undo_char(),
        KeyCode::Tab => app.reset_test(),
        KeyCode::Esc => app.change_to_settings(),
        _ => (),
    }
}

#[cfg(test)]
mod tests {
    use crate::application::App;
    use crossterm::event::{KeyCode, KeyEvent};
    use std::thread;
    use std::time::Duration;

    fn generate_key_events_passing_standart_test<'a>(app: &App) -> Vec<KeyEvent> {
        let mut kv = vec![];

        for a in &app.test.active {
            if let Some(c) = a.content.chars().last() {
                kv.push(KeyEvent::from(KeyCode::Char(c)))
            }
        }

        for a in &app.test.down {
            if let Some(c) = a.content.chars().last() {
                kv.push(KeyEvent::from(KeyCode::Char(c)))
            }
        }

        for d in app.test.backburner.iter().rev() {
            for a in d {
                if let Some(c) = a.content.chars().last() {
                    kv.push(KeyEvent::from(KeyCode::Char(c)))
                }
            }
        }

        for k in &kv {
            if let KeyCode::Char(c) = k.code {
                print!("{}", c);
            }
        }
        kv
    }

    fn go_thorugh_test_n_times(n: usize) {
        let mut app = App::setup();
        for _ in 0..n {
            let key_events = generate_key_events_passing_standart_test(&app);

            let klucznik_ptr = app.key_handler as usize;

            for kv in key_events {
                app.handle_key_event(kv);
            }

            // key_handler should be changed by now
            // as after the completed test the app should land itself in the post screen
            assert_ne!(klucznik_ptr, app.key_handler as usize);

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
        go_thorugh_test_n_times(1)
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

        use crate::settings::TypingTestConfig;
        let mut cfg = TypingTestConfig::default();
        cfg.length = 500;
        app.test.reset(&cfg);
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
        // wpm_test_setup(60.);
        // wpm_test_setup(140.);
        wpm_test_setup(220.);
    }

    #[test]
    fn test_accuracy() {
        let mut app = App::setup();
        let key_events = generate_key_events_passing_standart_test(&app);
        let key_events_len = key_events.len();
        let amount_of_mistakes = 10;

        for kv in &key_events[..amount_of_mistakes] {
            if let KeyCode::Char(c) = kv.code {
                match c {
                    'ź' => app.handle_key_event(KeyEvent::from(KeyCode::Char('a'))),
                    _ => app.handle_key_event(KeyEvent::from(KeyCode::Char('ź'))),
                }
            } else {
                unreachable!();
            }
            app.handle_key_event(KeyEvent::from(KeyCode::Backspace));
        }

        for kv in key_events {
            app.handle_key_event(kv);
        }

        let final_acc = app.test.hoarder.final_acc;
        let acc = {
            let denom = (key_events_len + amount_of_mistakes) as f64;
            key_events_len as f64 / denom * 100.
        };

        assert!(acc - f64::EPSILON <= final_acc);
        assert!(acc + f64::EPSILON >= final_acc);
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
