use crate::application::{App, TestState};
use crate::colorscheme::Theme;
use crossterm::event::KeyCode;
use std::borrow::Cow;

fn del_last_char(text: &str) -> String {
    let (cut, _) = text.char_indices().last().unwrap();
    String::from(&text[..cut])
}

pub fn test_key_handle<'a>(key: KeyCode, app: &mut App, test: &mut TestState<'a>, theme: &'a Theme) {
    match key {
        KeyCode::Char(c) => {
            // user pressed the correct key
            app.cursor_x += 1;
            test.correct += 1;

            if c == test.current_char {
                test.text[test.done].style = theme.done;
                test.done += 1;
                if test.done < test.test_length {
                    let opt_char = test.get_next_char();

                    if let Some(c) = opt_char {
                        test.current_char = c;
                    } else {
                        test.done += 1;
                        test.blanks += 1;
                        test.set_next_char();
                    }
                } else {
                    debug!("{}", test.calculate_wpm());
                    let _hah = test.test_length as f64 / 2.0;
                    test.restart_test(app, theme);
                }

            // wrong key
            } else {
                test.mistakes += 1;
                // adds the mistake and the end of the word
                if test.current_char == ' ' {
                    let mut testend = test.text[test.done - 1].content.to_string();
                    testend.push(c);
                    test.text[test.done - 1].content = Cow::from(testend);
                // just changes to wrong and moves on
                } else {
                    test.text[test.done].style = theme.wrong;
                    test.done += 1;

                    if test.done < test.test_length {
                        let opt_char = test.get_next_char();
                        if let Some(c) = opt_char {
                            test.current_char = c;
                        } else {
                            test.done += 1;
                            test.blanks += 1;
                            test.set_next_char();
                        }
                    // the end of the test
                    // TODO calculate wpm, acc and show post screen
                    } else {
                        debug!("{}", test.calculate_wpm());
                        test.restart_test(app, theme);
                    }
                }
            }
        }

        // TODO impl ctrl+Backspace
        KeyCode::Backspace => {
            if test.done > 0 {
                app.cursor_x -= 1;

                if test.current_char == ' ' {
                    if test.text[test.done - 1].content.is_empty() {
                        test.done -= 2;
                        test.blanks -= 1;
                        test.set_next_char();
                        test.text[test.done].style = theme.todo;
                    } else {
                        let temp = test.fetch(test.done - 1);
                        test.change(test.done - 1, del_last_char(&temp));
                    }
                } else {
                    test.done -= 1;
                    test.set_next_char();
                    test.text[test.done].style = theme.todo;
                }
            }
        }

        KeyCode::Tab => {
            test.restart_test(app, theme);
        }

        KeyCode::Esc => app.should_quit = true,

        _ => (),
    }
}
