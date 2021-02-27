mod terminal_prep;
mod application;
mod drawing;


use std::time::{Duration, Instant};
use std::{borrow::Cow, error::Error, fs::File, io::stdout, sync::mpsc, thread};

use terminal_prep::{cleanup_terminal, init_terminal};
use application::App;
use drawing::draw;

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

    let mut sout = stdout();
    let backend = CrosstermBackend::new(sout);
    let mut terminal = Terminal::new(backend)?;

    let rainbow = "quick brown fox jumps over the lazy dog!";

    let test_length: usize = rainbow.len() * 2;
    let mut string = rainbow.chars();
    let mut current_char = string.next().unwrap();
    let mut done: usize = 1;

    let mut app = App::default();
    app.wrongcolor = Style::default().fg(Color::Red);

    let mut test_text = Vec::with_capacity(test_length);
    for chr in rainbow.chars() {
        test_text.push(Span::styled(String::new(), app.wrongcolor));
        test_text.push(Span::styled(chr.to_string(), app.todocolor));
    }
    app.test_text = test_text.clone();


    app.start();
    loop {
        draw(&mut terminal, &mut app);

        if poll(Duration::from_millis(250))? {
            let read = read()?;
            if let CEvent::Key(event) = read {
                match event.code {
                    KeyCode::Char(c) => {
                        // user pressed the correct key
                        if c == current_char {
                            app.test_text[app.done].style = app.donecolor;
                            app.done += 2;
                            if app.done < test_length {
                                current_char = app.test_text[app.done].content.chars().next().unwrap();
                            } else {
                                // break
                            }
                        // wrong key
                        } else {
                            let mut append = app.test_text[app.done - 1].content.to_string();
                            if c == ' ' {
                                append.push('_');
                            } else {
                                append.push(c);
                            }
                            app.test_text[app.done - 1].content = Cow::from(append);
                        }
                    }

                    KeyCode::Backspace => {
                        if app.done > 1 {
                            if test_text[app.done - 1].content.is_empty() {
                                app.done -= 2;
                                current_char = app.fetch_content(app.done).chars().next().unwrap();
                                app.test_text[app.done].style = app.todocolor;
                            } else {
                                let temp = app.fetch_content(app.done  - 1);
                                app.change_content(app.done - 1, del_last_char(&temp))
                            }
                        }
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
