use crate::app::{App, InputMode};
use crate::utils::date;
use ratatui::style::palette::tailwind::{
    AMBER, BLUE, CYAN, GREEN, INDIGO, PINK, PURPLE, RED, SLATE, TEAL, YELLOW, ZINC,
};
use ratatui::style::Stylize;
use ratatui::text::Line;
use ratatui::widgets::{Borders, Padding, Widget};
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Gauge, Paragraph, Wrap},
    Frame,
};
use std::hash::{DefaultHasher, Hash, Hasher};

pub fn draw(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::vertical([
        Constraint::Length(1),
        Constraint::Length(3),
        Constraint::Min(1),
    ])
    .spacing(1)
    .margin(1)
    .split(frame.area());

    match app.input_mode {
        InputMode::Normal => {
            if app.commits.is_none() {
                let loading = Paragraph::new("loading repository data...").style(Style::default());
                frame.render_widget(loading, chunks[1]);
            }
            if app.current_week.is_some() {
                let current_week = app.current_week.unwrap();
                let current_week = Paragraph::new(
                    date::convert_timestamp_to_month_and_year(&current_week).to_string(),
                )
                .style(Style::default().fg(Color::Blue))
                .block(Block::default().borders(Borders::BOTTOM));
                frame.render_widget(current_week, chunks[1]);
            }
        }
        InputMode::Editing => {
            frame.set_cursor_position((
                chunks[1].x + app.input.visual_cursor() as u16 + 1,
                chunks[1].y + 1,
            ));
            let input = Paragraph::new(app.input.value())
                .style(Style::default().fg(Color::Yellow))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Github repository, ie: 'rust-lang/rust'"),
                )
                .slow_blink();
            frame.render_widget(input, chunks[1]);
        }
    }
    let greeting = Paragraph::new("")
        .style(Style::default().fg(Color::Yellow))
        .block(
            Block::default()
                .borders(Borders::BOTTOM)
                .title("Github commit race, press 'q' to quit "),
        );
    frame.render_widget(greeting, chunks[0]);
    if app.error.is_some() {
        let error = Paragraph::new(app.error.as_ref().unwrap().as_str())
            .style(Style::default().fg(Color::Red))
            .wrap(Wrap { trim: true })
            .block(Block::default().borders(Borders::ALL).title("Error"));
        frame.render_widget(error, chunks[2]);
    }
    if let Some(authors) = app.current_tick_authors.as_ref() {
        let layout = Layout::vertical([
            Constraint::Length(6),
            Constraint::Length(6),
            Constraint::Length(6),
            Constraint::Length(6),
            Constraint::Length(6),
        ]);
        let areas: [Rect; 5] = layout.areas(chunks[2]);

        let only_first_five = authors[0..5].to_vec();
        for (i, author_with_commit) in only_first_five.iter().enumerate() {
            if i == 0 {
                render_gauge(author_with_commit, 1.0, areas[i], frame);
            } else {
                let ratio: f64 = author_with_commit.1 as f64 / only_first_five[0].1 as f64;
                render_gauge(author_with_commit, ratio, areas[i], frame);
            }
        }
    }

    fn render_gauge(author: &(String, u32), ratio: f64, area: Rect, frame: &mut Frame) {
        let title = title_block(author.0.as_str());
        Gauge::default()
            .block(title)
            .gauge_style(assign_color(author.0.as_str()))
            .ratio(ratio)
            .label(author.1.to_string())
            .render(area, frame.buffer_mut());
    }

    fn title_block(username: &str) -> Block {
        let title = Line::from(username).centered();
        Block::new()
            .borders(Borders::NONE)
            .padding(Padding::vertical(1))
            .title(title)
            .fg(assign_color(username))
    }
}

fn hash_username(username: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    username.hash(&mut hasher);
    hasher.finish()
}

fn assign_color(username: &str) -> Color {
    let palettes = [
        &RED, &SLATE, &ZINC, &AMBER, &CYAN, &BLUE, &GREEN, &INDIGO, &PINK, &PURPLE, &TEAL, &YELLOW,
    ];
    let color_keys = [
        "c300", "c400", "c500", "c600", "c700", "c800", "c900", "c950",
    ];

    let hash = hash_username(username);
    let palette_index = (hash as usize) % palettes.len(); // Pick a palette
    let color_index = (hash as usize) % color_keys.len(); // Pick a color

    let palette = palettes[palette_index];
    match color_index {
        0 => palette.c50,
        1 => palette.c100,
        2 => palette.c200,
        3 => palette.c300,
        4 => palette.c400,
        5 => palette.c500,
        6 => palette.c600,
        7 => palette.c700,
        8 => palette.c800,
        9 => palette.c900,
        10 => palette.c950,
        _ => palette.c500, // Default fallback
    }
}
