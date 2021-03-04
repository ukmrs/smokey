mod application;
mod colorscheme;
mod drawing;
mod langs;
mod terminal_prep;

use std::time::{Duration, Instant};
use std::{borrow::Cow, error::Error, fs::File, io::stdout, sync::mpsc, thread};

use application::App;
use colorscheme::Theme;
use drawing::draw;
use langs::prepare_test;
use terminal_prep::{cleanup_terminal, init_terminal};

#[allow(unused_imports)]
#[macro_use]
extern crate log;

use simplelog::*;

use crossterm::event::{poll, read, Event as CEvent, KeyCode, KeyEvent, KeyModifiers};

#[allow(unused_imports)]
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Span, Spans},
    widgets::canvas::{Canvas, Line, Map, MapResolution, Rectangle},
    widgets::{
        Axis, BarChart, Block, Borders, Chart, Dataset, Gauge, List, ListItem, Paragraph, Row,
        Sparkline, Table, Tabs, Wrap,
    },
    Frame, Terminal,
};

fn del_last_char(text: &str) -> String {
    let (cut, _) = text.char_indices().last().unwrap();
    String::from(&text[..cut])
}

fn main() -> crossterm::Result<()> {
    WriteLogger::init(
        LevelFilter::Debug,
        Config::default(),
        File::create("smokey.log").unwrap(),
    )
    .unwrap();

    init_terminal();

    #[allow(unused_mut)]
    let mut sout = stdout();

    let backend = CrosstermBackend::new(sout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::create_app();
    let theme = Theme::initial();

    app.test_text = prepare_test("./languages/english", 5, &theme);
    let mut current_char = app.test_text[app.done].content.chars().next().unwrap();
    let mut test_length: usize = app.test_text.len();
    app.start_timer();
    let mut correct = 0;

    loop {
        draw(&mut terminal, &mut app);

        if poll(Duration::from_millis(250))? {
            let read = read()?;
            if let CEvent::Key(event) = read {
                match event.code {
                    KeyCode::Char(c) => {
                        // user pressed the correct key
                        app.cursor_x += 1;
                        correct += 1;

                        if c == current_char {
                            app.test_text[app.done].style = theme.done;
                            app.done += 1;
                            if app.done < test_length {
                                let opt_char = app.test_text[app.done].content.chars().next();

                                if let Some(c) = opt_char {
                                    current_char = c;
                                } else {
                                    app.done += 1;
                                    current_char = app.test_text[app.done]
                                        .content
                                        .chars()
                                        .next()
                                        .expect("oof, somehow there is two times blank space");
                                }
                            } else {
                                debug!("{}", app.calculate_wpm());
                                let _hah = test_length as f64 / 2.0;
                                app.restart_test(&theme, &mut current_char, &mut test_length);
                            }

                        // wrong key
                        } else {
                            app.mistakes += 1;
                            if current_char == ' ' {
                                let mut append = app.test_text[app.done - 1].content.to_string();
                                append.push(c);
                                app.test_text[app.done - 1].content = Cow::from(append);
                            } else {
                                app.test_text[app.done].style = theme.wrong;
                                app.done += 1;

                                if app.done < test_length {
                                    let opt_char = app.test_text[app.done].content.chars().next();
                                    if let Some(c) = opt_char {
                                        current_char = c;
                                    } else {
                                        app.done += 1;
                                        current_char =
                                            app.test_text[app.done].content.chars().next().expect(
                                                "oof, somehow there is two times blank space",
                                            );
                                    }
                                } else {
                                    debug!("{}", app.calculate_wpm());
                                    app.restart_test(&theme, &mut current_char, &mut test_length);
                                }
                            }
                        }
                    }

                    KeyCode::Backspace => {
                        if app.done > 0 {
                            app.cursor_x -= 1;

                            if current_char == ' ' {
                                if app.test_text[app.done - 1].content.is_empty() {
                                    app.done -= 2;
                                    current_char =
                                        app.fetch_content(app.done).chars().next().unwrap();
                                    app.test_text[app.done].style = theme.todo;
                                } else {
                                    let temp = app.fetch_content(app.done - 1);
                                    app.change_content(app.done - 1, del_last_char(&temp))
                                }

                            } else {
                                app.done -= 1;
                                current_char = app.fetch_content(app.done).chars().next().unwrap();
                                app.test_text[app.done].style = theme.todo;
                            }
                        }
                    }

                    KeyCode::Tab => {
                        app.restart_test(&theme, &mut current_char, &mut test_length);
                    }

                    KeyCode::Esc => break,
                    _ => (),
                }
            }
        } else {
            // Timeout expired and no `Event` is available
        }
    }

    cleanup_terminal();
    Ok(())
}
