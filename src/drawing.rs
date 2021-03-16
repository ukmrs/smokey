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

pub fn draw<B: Backend>(terminal: &mut Terminal<B>, app: &mut App, test: &mut TestState) {
        terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
                .split(frame.size());

            frame.set_cursor(app.cursor_x, chunks[0].height + 1);



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

            let block = Paragraph::new(up_txt)
                .block(Block::default().title("WPM").borders(Borders::ALL));

            frame.render_widget(block, chunks[0]);

            let txt = vec![Spans::from(test.text.clone())];

            let paragraph = Paragraph::new(txt)
                .block(Block::default().title("Text box").borders(Borders::ALL))
                .style(Style::default().fg(Color::White));

            frame.render_widget(paragraph, chunks[1]);
        }).unwrap();
}
