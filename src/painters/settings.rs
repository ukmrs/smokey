use crate::application::{App, APPLOGO};
use crate::settings::SetList;
use std::collections::HashMap;

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

            let color_code = app
                .settings
                .color_hover_or_active(app.theme.hover, app.theme.active);

            draw_row_with_freq_and_len(f, app, chunks[1], &color_code);
            draw_row_with_words_and_mods(f, app, chunks[2], &color_code);
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

    let block =
        Paragraph::new("english: 10000 words").block(Block::default().borders(Borders::NONE));
    f.render_widget(block, chunks[1]);
}

pub fn draw_row_with_freq_and_len<B: Backend>(
    f: &mut Frame<B>,
    app: &mut App,
    rect: Rect,
    clrcode: &HashMap<SetList, Option<Color>>,
) {
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
        clrcode[&SetList::Length],
    );

    render_stateful_list(
        f,
        &app.settings.frequency_list.items,
        &mut app.settings.frequency_list.state,
        "word amount",
        chunks[1],
        clrcode[&SetList::Frequency],
    );
}

pub fn draw_row_with_words_and_mods<B: Backend>(
    f: &mut Frame<B>,
    app: &mut App,
    rect: Rect,
    clrcode: &HashMap<SetList, Option<Color>>,
) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(rect);

    render_stateful_list(
        f,
        &app.settings.tests_list.items,
        &mut app.settings.tests_list.state,
        "test",
        chunks[0],
        clrcode[&SetList::Test],
    );

    render_stateful_list(
        f,
        &app.settings.mods_list.items,
        &mut app.settings.mods_list.state,
        "mods",
        chunks[1],
        clrcode[&SetList::Mods],
    );
}

pub fn render_stateful_list<B: Backend>(
    f: &mut Frame<B>,
    sl: &[String],
    ls: &mut ListState,
    title: &str,
    area: Rect,
    clr: Option<Color>,
) {
    let border_style: Style = match clr {
        Some(c) => Style::default().fg(c).add_modifier(Modifier::BOLD),
        None => Style::default().fg(Color::Gray),
    };

    let items = create_item_list(sl, title, border_style);
    f.render_stateful_widget(items, area, ls)
}

pub fn create_item_list<'a>(sl: &[String], title: &'a str, border_style: Style) -> List<'a> {
    let items: Vec<ListItem> = sl
        .iter()
        .map(|i| ListItem::new(Span::from(i.clone())).style(Style::default().fg(Color::Gray)))
        .collect();

    List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(border_style)
                .title(title),
        )
        .highlight_style(
            Style::default()
                .fg(border_style.fg.unwrap())
                .add_modifier(Modifier::BOLD),
        )
    // .highlight_symbol("> ")
}
