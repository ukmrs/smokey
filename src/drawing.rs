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
            let block = Paragraph::new(Span::from(wpm))
                .block(Block::default().title("WPM").borders(Borders::ALL));

            frame.render_widget(block, chunks[0]);

            let txt = vec![Spans::from(test.text.clone())];

            let paragraph = Paragraph::new(txt)
                .block(Block::default().title("Text box").borders(Borders::ALL))
                .style(Style::default().fg(Color::White).bg(Color::Black));

            frame.render_widget(paragraph, chunks[1]);
        }).unwrap();
}
