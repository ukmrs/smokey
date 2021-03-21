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

use crate::application::{App, TestState};

pub fn draw_test<B: Backend>(terminal: &mut Terminal<B>, app: &mut App, test: &mut TestState) {
    terminal
        .draw(|frame| {

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
                .vertical_margin(app.margin)
                .horizontal_margin(app.margin)
                .split(frame.size());

            frame.set_cursor(app.cursor_x + app.margin, chunks[0].height + 1 + app.margin);

            let wpm: String = test.calculate_wpm().round().to_string();

            #[allow(unused_mut)]
            let mut dbg_info = String::new();

            // ---***---
            // dbg_info += &format!("blanks: {}/ ", test.blanks);
            // dbg_info += &format!("done: {}/ ", test.done);
            // dbg_info += &format!("fetch: {}/ ", test.fetch(test.done));
            // dbg_info += &format!("cchar: {}/ ", test.current_char);
            // // ---***---

            let up_txt = vec![Spans::from(wpm), Spans::from(dbg_info)];

            let block =
                Paragraph::new(up_txt).block(Block::default().title("WPM").borders(Borders::ALL));

            frame.render_widget(block, chunks[0]);

            let txt = vec![Spans::from(test.text.clone())];

            let paragraph = Paragraph::new(txt)
                .block(Block::default().title("Text box").borders(Borders::ALL))
                .style(Style::default().fg(Color::White));

            frame.render_widget(paragraph, chunks[1]);
        })
        .expect("drawing test went fine");
}

/// draws post screen
pub fn draw_post<B: Backend>(terminal: &mut Terminal<B>, app: &mut App, test: &mut TestState) {
    terminal
        .draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
                .vertical_margin(2)
                .horizontal_margin(2)
                .split(frame.size());

            let up_txt = "some bs";

            let block =
                Paragraph::new(up_txt).block(Block::default().title("WPM").borders(Borders::ALL));

            frame.render_widget(block, chunks[0]);

            let secs: f64 = test.hoarder.seconds as f64;
            let length: f64 = test.hoarder.wpms.len() as f64;
            // maybe color depending on how well the test went
            // its kinda sad to calculate this closure all the time
            // even though its not a big deal, change that later
            let data = test
                .hoarder
                .wpms
                .iter()
                .enumerate()
                .map(|(i, val)| (((i + 1) as f64 * secs as f64), *val))
                .collect::<Vec<(f64, f64)>>();


            let wpm_datasets = vec![Dataset::default()
                .name("wpm")
                .marker(symbols::Marker::Dot)
                .style(Style::default().fg(Color::Blue))
                .data(&data)];

            let x_labels: Vec<Span> = data
                .iter()
                .map(|&(i, _)| Span::styled(format!("{}", i), Style::default().fg(Color::Blue)))
                .collect();


            debug!("{:?}", x_labels);

            let chart = Chart::new(wpm_datasets)
                .block(
                    Block::default()
                        .title(Span::styled(
                            "Chart 1",
                            Style::default()
                                .fg(Color::Cyan)
                                .add_modifier(Modifier::BOLD),
                        ))
                        .borders(Borders::ALL),
                )
                .x_axis(
                    Axis::default()
                        .title("X Axis")
                        .style(Style::default().fg(Color::Gray))
                        // TODO importance: meh
                        // this has to panic if length is zero right
                        // would happen in a bizarre scenario of 1 word test done under sec
                        // so there needs to be a test invalid or something idk
                        .bounds([secs, length * secs])
                        .labels(x_labels),
                )
                .y_axis(
                    Axis::default()
                        .title("Y Axis")
                        .style(Style::default().fg(Color::Gray))
                        .bounds([0., 100.]),
                );

            frame.render_widget(chart, chunks[1]);
        })
        .expect("drawing post went fine")
}
