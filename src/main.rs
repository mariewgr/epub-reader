use clap::{Parser, Subcommand};
use epub::doc::EpubDoc;

mod app;
mod event;
mod tui;
mod ui;
#[derive(Parser)]
#[command(name = "epub-reader", about = "A terminal epub reader")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Show book metadata
    Info { file: String },
    /// Show chapter previews
    Read {
        file: String,
        #[arg(short, long)]
        chapter: Option<usize>,
    },
}

fn strip_html(html: &str) -> String {
    let mut result = String::new();
    let mut inside_tag = false;

    for ch in html.chars() {
        match ch {
            '<' => inside_tag = true,
            '>' => inside_tag = false,
            _ if !inside_tag => result.push(ch),
            _ => {}
        }
    }

    result.split_whitespace().collect::<Vec<_>>().join(" ")
}
fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Info { file } => {
            let doc = EpubDoc::new(&file).expect("Failed to open epub file");
            println!(
                "Title:  {}",
                doc.mdata("title")
                    .map(|item| item.value.clone())
                    .unwrap_or("Unknown".to_string())
            );
            println!(
                "Author: {}",
                doc.mdata("creator")
                    .map(|item| item.value.clone())
                    .unwrap_or("Unknown".to_string())
            );
            println!("Chapters: {}", doc.get_num_chapters());
        }
        Commands::Read { file, chapter: _ } => {
            let mut doc = EpubDoc::new(&file).expect("Failed to open epub file");

            let mut chapter_titles: Vec<String> = Vec::new();
            let mut chapter_contents: Vec<String> = Vec::new();

            for i in 0..doc.get_num_chapters() {
                doc.set_current_chapter(i);
                chapter_titles.push(format!("Chapter {}", i + 1));
                let content = doc
                    .get_current_str()
                    .map(|(html, _)| strip_html(&html))
                    .unwrap_or_default();
                chapter_contents.push(content);
            }

            let (width, _) = crossterm::terminal::size().unwrap_or((80, 24));
            let reader_width = (width as usize).saturating_sub(27);
            let mut app = app::App::new(chapter_contents, reader_width, chapter_titles);
            let mut terminal = tui::enter().expect("Failed to start TUI");

            while !app.should_quit {
                terminal.draw(|frame| ui::draw(frame, &app)).unwrap();
                event::handle_events(&mut app, reader_width).unwrap();
            }

            tui::exit().expect("Failed to restore terminal");
        }
    }
}
