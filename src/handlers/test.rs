use crate::application::{App, TestState};
use crate::colorscheme::{Theme, ToForeground};
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

fn set_next_char_or_end(app: &mut App, _theme: &Theme) {
    if app.test.done < app.test.test_length {
        set_next_char_beware_blanks(&mut app.test)
    } else {
        debug!("{}", app.test.calculate_wpm());
        // test.reset(app, theme);
        app.end_test();
    }
}

/// handles keys during test
pub fn handle<'a>(
    key: KeyEvent,
    app: &mut App<'a>,
    theme: Theme,
) {
    let test = &mut app.test;
    // well doing this in terminal was a bad idea XD
    // Ctrl + Backspace registers as weird thing in terminals
    // I got ctrl(h) and ctrl(7) among others
    // but the ctrl is always there
    // so I just detect ctrl
    // its iffy but works
    // renders ctrl useless during test for other uses though
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
                set_next_char_or_end(app, &theme);

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
                    set_next_char_or_end(app, &theme);
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

        KeyCode::Esc => app.switch_to_settings(),

        _ => (),
    }
}
