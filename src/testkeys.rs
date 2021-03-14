use crate::application::{App, TestState};
use crate::colorscheme::Theme;
use crossterm::event::{KeyEvent, KeyCode};
use std::borrow::Cow;

fn del_last_char(text: &str) -> String {
    let (cut, _) = text.char_indices().last().unwrap();
    String::from(&text[..cut])
}

fn set_next_char_beware_blanks<'a>(test: &mut TestState<'a>) {
    let opt_char = test.get_next_char();
    if let Some(c) = opt_char {
        test.current_char = c;
    } else {
        test.done += 1;
        test.blanks += 1;
        test.set_next_char();
    }
}

fn set_next_char_or_end<'a>(app: &mut App, test: &mut TestState<'a>, theme: &'a Theme) {
    if test.done < test.test_length {
        set_next_char_beware_blanks(test)
    } else {
        debug!("{}", test.calculate_wpm());
        test.restart_test(app, theme);
    }
}

pub fn test_key_handle<'a>(
    key: KeyEvent,
    app: &mut App,
    test: &mut TestState<'a>,
    theme: &'a Theme,
) {
    match key.code {
        KeyCode::Char(c) => {
            // debug!("{:?}, {:?}", key.code, key.modifiers);
            app.cursor_x += 1;

            // user pressed the correct key
            if c == test.current_char {
                test.correct += 1;
                test.text[test.done].style = theme.done;
                test.done += 1;
                set_next_char_or_end(app, test, theme);

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
                    set_next_char_or_end(app, test, theme);
                }
            }
        }

        // TODO impl ctrl+Backspace
        // CTRL BACKSPACE registers as ctrl 7
        KeyCode::Backspace => {
            debug!("{:?}", key.modifiers);
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
