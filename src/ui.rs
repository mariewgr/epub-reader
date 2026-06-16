use crate::app::App;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    text::Text,
    widgets::{Block, Borders, Paragraph},
};

pub fn draw(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(frame.area());

    let visible_lines: Vec<&str> = app
        .lines
        .iter()
        .skip(app.scroll)
        .take(chunks[0].height.saturating_sub(2) as usize)
        .map(|l| l.as_str())
        .collect();

    let text = Text::from(visible_lines.join("\n"));
    let paragraph = Paragraph::new(text).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" epub-reader "),
    );
    frame.render_widget(paragraph, chunks[0]);

    let status = format!(
        " Line {}/{} — j/k or ↑/↓ to scroll, q to quit",
        app.scroll + 1,
        app.lines.len()
    );
    let status_bar = Paragraph::new(status);
    frame.render_widget(status_bar, chunks[1]);
}
