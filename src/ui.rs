use crate::app::App;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    text::Text,
    widgets::{Block, Borders, Paragraph},
};

pub fn draw(frame: &mut Frame, app: &App) {
    let area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0)])
        .split(frame.area())[0];

    let visible_lines: Vec<&str> = app
        .lines
        .iter()
        .skip(app.scroll)
        .take(area.height as usize)
        .map(|l| l.as_str())
        .collect();

    let text = Text::from(visible_lines.join("\n"));

    let paragraph = Paragraph::new(text).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" epub-reader "),
    );

    frame.render_widget(paragraph, area);
}
