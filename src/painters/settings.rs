use crate::application::{App, APPLOGO};

use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame, Terminal,
};

pub fn draw_settings<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) {
    terminal
        .draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(

                    [
                        Constraint::Percentage(20),
                        Constraint::Percentage(40),
                        Constraint::Percentage(40),
                    ]
                    .as_ref(),

                )
                .vertical_margin(app.margin)
                .horizontal_margin(app.margin)
                .split(f.size());

            draw_title(f, chunks[0]);
            draw_row_with_freq_and_len(f, app, chunks[1]);
            draw_row_with_words_and_mods(f, app, chunks[2]);
        })
        .expect("drawing settings");
}

pub fn draw_title<B: Backend>(f: &mut Frame<B>, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(area);

    let block = Paragraph::new(APPLOGO).block(Block::default().borders(Borders::NONE));
    f.render_widget(block, chunks[0]);

    let block = Paragraph::new(APPLOGO).block(Block::default().borders(Borders::NONE));
    f.render_widget(block, chunks[1]);

}

pub fn draw_row_with_freq_and_len<B: Backend>(f: &mut Frame<B>, app: &mut App, rect: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(rect);

    render_stateful_list(
        f,
        &app.settings.length_list.items,
        &mut app.settings.length_list.state,
        "test length",
        chunks[0],
    );

    render_stateful_list(
        f,
        &app.settings.frequency_list.items,
        &mut app.settings.frequency_list.state,
        "word amount",
        chunks[1],
    );
}

pub fn draw_row_with_words_and_mods<B: Backend>(f: &mut Frame<B>, app: &mut App, rect: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(rect);

    render_stateful_list(
        f,
        &app.settings.words_list.items,
        &mut app.settings.words_list.state,
        "language",
        chunks[0],
    );

    render_stateful_list(
        f,
        &app.settings.mods_list.items,
        &mut app.settings.mods_list.state,
        "mods",
        chunks[1],
    );
}

pub fn render_stateful_list<B: Backend>(
    f: &mut Frame<B>,
    sl: &[String],
    ls: &mut ListState,
    title: &str,
    area: Rect,
) {
    let items = create_item_list(sl, title);
    f.render_stateful_widget(items, area, ls)
}

pub fn create_item_list<'a>(sl: &[String], title: &'a str) -> List<'a> {
    let items: Vec<ListItem> = sl
        .iter()
        .map(|i| ListItem::new(Span::from(i.clone())).style(Style::default().fg(Color::Gray)))
        .collect();

    List::new(items)
        .block(Block::default().borders(Borders::ALL).title(title))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ")
}
