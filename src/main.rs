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
        Commands::Read { file, chapter } => {
            let mut doc = EpubDoc::new(&file).expect("Failed to open epub file");

            let content = match chapter {
                Some(n) => {
                    doc.set_current_chapter(n - 1);
                    doc.get_current_str()
                        .map(|(html, _)| strip_html(&html))
                        .unwrap_or_default()
                }
                None => {
                    let mut all = String::new();
                    for i in 0..doc.get_num_chapters() {
                        doc.set_current_chapter(i);
                        if let Some((html, _)) = doc.get_current_str() {
                            all.push_str(&format!("=== Chapter {} ===\n", i + 1));
                            all.push_str(&strip_html(&html));
                            all.push_str("\n\n");
                        }
                    }
                    all
                }
            };

            let (width, _) = crossterm::terminal::size().unwrap_or((80, 24));
            let mut app = app::App::new(content, (width as usize).saturating_sub(2));
            let mut terminal = tui::enter().expect("Failed to start TUI");

            while !app.should_quit {
                terminal.draw(|frame| ui::draw(frame, &app)).unwrap();
                event::handle_events(&mut app).unwrap();
            }

            tui::exit().expect("Failed to restore terminal");
        }
    }
}
