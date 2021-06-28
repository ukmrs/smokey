use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Spans,
    widgets::{Block, Borders, Paragraph, Wrap},
    Terminal,
};

use crate::application::App;
use crate::Term;

pub fn draw_test_and_update(terminal: &mut Term, app: &mut App) {
    draw_test(terminal, app);
    app.test.update_wpm_history();
}

pub fn draw_test<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) {
    terminal
        .draw(|frame| {
            let test = &mut app.test;

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(35), Constraint::Percentage(65)].as_ref())
                .split(frame.size());

            let wpm: String = test.calculate_wpm().round().to_string();

            #[allow(unused_mut)]
            let mut dbg_info = String::new();

            // ---***---
            // dbg_info += &format!("mistakes: {}/ ", test.mistakes);
            // dbg_info += &format!("done: {}/ ", test.done);
            // dbg_info += &format!("pdone: {}/ ", test.pdone);
            // dbg_info += &format!("blanks: {}/ ", test.blanks);
            // dbg_info += &format!("cchar: {}/ ", test.current_char);
            // dbg_info += &format!("cursor: {}/ ", test.cursor_x);
            // // // ---***---

            let up_txt = vec![Spans::from(wpm), Spans::from(dbg_info)];

            let block = Paragraph::new(up_txt).block(Block::default().borders(Borders::NONE));

            frame.render_widget(block, chunks[0]);

            let ghost_rect_width = (frame.size().width - app.paragraph) / 2;
            let down_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Length(ghost_rect_width), Constraint::Min(60)].as_ref())
                .split(chunks[1]);

            frame.set_cursor(
                down_chunks[0].width + test.cursor_x - 1,
                chunks[0].height + 1,
            );

            let txt = vec![
                Spans::from(app.test.up.clone()),
                Spans::from(app.test.active.clone()),
                Spans::from(app.test.down.clone()),
            ];

            let paragraph = Paragraph::new(txt)
                .block(Block::default().borders(Borders::NONE))
                .style(Style::default().fg(Color::White))
                // .alignment(Alignment::Center)
                .wrap(Wrap { trim: false });

            frame.render_widget(paragraph, down_chunks[1]);
        })
        .expect("drawing test went fine");
}
