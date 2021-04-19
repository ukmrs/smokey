use crate::application::{App, Screen, TestState};
use crate::colorscheme::Theme;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub fn key_handle<'a>(key: KeyEvent, app: &mut App, test: &mut TestState<'a>, theme: &'a Theme) {
    match app.screen {
        Screen::Test => handle_keys_test(key, app, test, theme),
        Screen::Post => handle_keys_post(key, app, test, theme),
        Screen::Settings => handle_keys_settings(key, app, test, theme),
    }
}

fn set_next_char_beware_blanks<'a>(test: &mut TestState<'a>) {
    if let Some(c) = test.get_next_char() {
        test.current_char = c;
    } else {
        test.done += 1;
        test.blanks += 1;
        test.set_next_char();
    }
}

fn set_next_char_or_end<'a>(app: &mut App, test: &mut TestState<'a>, _theme: &'a Theme) {
    if test.done < test.test_length {
        set_next_char_beware_blanks(test)
    } else {
        debug!("{}", test.calculate_wpm());
        // test.reset(app, theme);
        test.end(app);
    }
}

/// handles keys during test
fn handle_keys_test<'a>(key: KeyEvent, app: &mut App, test: &mut TestState<'a>, theme: &'a Theme) {
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
                app.should_quit = true;
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
            test.if_mistake_deduct(test.done, theme);
            test.text[test.done].style = theme.todo;
            test.blanks -= 1;
        } else if test.fetch(test.done - 1) == " " {
            test.text[test.done - 1].style = theme.todo;
            app.cursor_x -= test.fetch(test.done - 2).len() as u16 + 2;
            test.change(test.done - 2, String::new());
            test.done -= 3;
            test.if_mistake_deduct(test.done, theme);
            test.text[test.done].style = theme.todo;
            test.blanks -= 1;
        }

        // undoes the test until it deletes a word
        while test.done != 0 && test.fetch(test.done - 1) != " " {
            app.cursor_x -= 1;
            test.done -= 1;
            test.if_mistake_deduct(test.done, theme);
            test.text[test.done].style = theme.todo;
        }
        test.set_next_char();
        return;
    }

    match key.code {
        KeyCode::Char(c) => {
            app.cursor_x += 1;

            // user pressed the correct key
            if c == test.current_char {
                test.text[test.done].style = theme.done;
                test.done += 1;
                set_next_char_or_end(app, test, theme);

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
                        // well in this rare case nothing happens so it needs to reverse
                        app.cursor_x -= 1;
                    }
                // just changes to wrong and moves on
                } else {
                    test.mistakes += 1;
                    test.text[test.done].style = theme.wrong;
                    test.done += 1;
                    set_next_char_or_end(app, test, theme);
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
                        test.if_mistake_deduct(test.done - 2, theme);
                        test.done -= 2;
                        test.blanks -= 1;
                        test.set_next_char();
                        test.text[test.done].style = theme.todo;
                    } else {
                        test.text[test.done - 1]
                            .content
                            .to_mut()
                            .pop()
                            .expect("checked above");
                    }
                } else {
                    test.done -= 1;
                    test.if_mistake_deduct(test.done, theme);
                    test.set_next_char();
                    test.text[test.done].style = theme.todo;
                }
            }
        }

        KeyCode::Tab => {
            test.reset(app, theme);
        }

        KeyCode::Esc => app.should_quit = true,

        _ => (),
    }
}

fn handle_keys_post<'a>(
    key: KeyEvent,
    app: &mut App,
    test: &mut TestState<'a>,
    theme: &'a Theme,
) {
    match key.code {
        KeyCode::Esc => app.should_quit = true,

        KeyCode::Tab => {
            app.screen = Screen::Test;
            test.reset(app, theme);
        }

        KeyCode::Char(c) => {
            if let KeyModifiers::CONTROL = key.modifiers {
                if c == 'c' {
                    app.should_quit = true;
                }
            }
        }
        _ => (),
    }
}

fn handle_keys_settings<'a>(
    key: KeyEvent,
    app: &mut App,
    test: &mut TestState<'a>,
    theme: &'a Theme,
) {
    match key.code {
        KeyCode::Esc => app.should_quit = true,

        KeyCode::Tab => {
            app.screen = Screen::Test;
            test.reset(app, theme);
        }

        KeyCode::Char(c) => {
            if let KeyModifiers::CONTROL = key.modifiers {
                if c == 'c' {
                    app.should_quit = true;
                }
            }
        }
        _ => (),
    }
}
