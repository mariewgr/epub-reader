use crate::app::{App, Focus};
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::time::Duration;

pub fn handle_events(app: &mut App, reader_width: usize) -> std::io::Result<()> {
    if event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => app.should_quit = true,
                    KeyCode::Tab => app.toggle_focus(),
                    KeyCode::Enter => {
                        if app.focus == Focus::Chapters {
                            app.load_chapter(reader_width);
                            app.toggle_focus();
                        }
                    }
                    KeyCode::Down | KeyCode::Char('j') => match app.focus {
                        Focus::Reader => app.scroll_down(),
                        Focus::Chapters => app.next_chapter(),
                    },
                    KeyCode::Up | KeyCode::Char('k') => match app.focus {
                        Focus::Reader => app.scroll_up(),
                        Focus::Chapters => app.prev_chapter(),
                    },
                    _ => {}
                }
            }
        }
    }
    Ok(())
}
