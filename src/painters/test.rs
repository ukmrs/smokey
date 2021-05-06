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
                .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
                .vertical_margin(app.margin)
                .horizontal_margin(app.margin)
                .split(frame.size());

            frame.set_cursor(app.cursor_x + app.margin, chunks[0].height + 1 + app.margin);

            let wpm: String = test.calculate_wpm().round().to_string();

            #[allow(unused_mut)]
            let mut dbg_info = String::new();

            // ---***---
            // dbg_info += &format!("mistakes: {}/ ", test.mistakes);
            // dbg_info += &format!("done: {}/ ", test.done);
            // dbg_info += &format!("fetch: {}/ ", test.fetch(test.done));
            // dbg_info += &format!("cchar: {}/ ", test.current_char);
            // // ---***---

            let up_txt = vec![Spans::from(wpm), Spans::from(dbg_info)];

            let block =
                Paragraph::new(up_txt).block(Block::default().title("WPM").borders(Borders::ALL));

            frame.render_widget(block, chunks[0]);

            let txt = vec![Spans::from(app.test.text.clone())];

            let paragraph = Paragraph::new(txt)
                .block(Block::default().title("Text box").borders(Borders::ALL))
                .style(Style::default().fg(Color::White))
                .wrap(Wrap { trim: false });

            frame.render_widget(paragraph, chunks[1]);
        })
        .expect("drawing test went fine");
}
