use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    text::Text,
};
use crate::app::{App, Focus};

pub fn draw(frame: &mut Frame, app: &App) {
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(frame.area());

    let panels = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(25), Constraint::Min(0)])
        .split(main_chunks[0]);

    // Chapter list panel
    let items: Vec<ListItem> = app.chapters
        .iter()
        .map(|c| ListItem::new(c.as_str()))
        .collect();

    let chapter_block = Block::default()
        .borders(Borders::ALL)
        .title(" Chapters ")
        .border_style(match app.focus {
            Focus::Chapters => Style::default().fg(Color::Yellow),
            Focus::Reader => Style::default(),
        });

    let chapter_list = List::new(items)
        .block(chapter_block)
        .highlight_style(Style::default().fg(Color::Black).bg(Color::Yellow));

    let mut list_state = ListState::default();
    list_state.select(Some(app.selected_chapter));

    frame.render_stateful_widget(chapter_list, panels[0], &mut list_state);

    // Reader panel
    let reader_block = Block::default()
        .borders(Borders::ALL)
        .title(" epub-reader ")
        .border_style(match app.focus {
            Focus::Reader => Style::default().fg(Color::Yellow),
            Focus::Chapters => Style::default(),
        });

    let visible_lines: Vec<&str> = app.lines
        .iter()
        .skip(app.scroll)
        .take(panels[1].height.saturating_sub(2) as usize)
        .map(|l| l.as_str())
        .collect();

    let paragraph = Paragraph::new(Text::from(visible_lines.join("\n")))
        .block(reader_block);

    frame.render_widget(paragraph, panels[1]);

    // Status bar
    let status = format!(
        " [Tab] switch panel  [j/k] scroll  [q] quit  —  Line {}/{}",
        app.scroll + 1,
        app.lines.len()
    );
    frame.render_widget(Paragraph::new(status), main_chunks[1]);
}
