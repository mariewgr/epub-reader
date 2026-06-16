use clap::{Parser, Subcommand};
use epub::doc::EpubDoc;

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
            println!(
                "Title:  {}",
                doc.mdata("title")
                    .map(|item| item.value.clone())
                    .unwrap_or("Unknown".to_string())
            );
            println!();

            match chapter {
                Some(n) => {
                    doc.set_current_chapter(n - 1);
                    if let Some((html, _mime)) = doc.get_current_str() {
                        let text = strip_html(&html);
                        println!("{}", text.trim());
                    }
                }
                None => {
                    for i in 0..doc.get_num_chapters() {
                        doc.set_current_chapter(i);
                        if let Some((html, _mime)) = doc.get_current_str() {
                            println!("--- Chapter {} ---", i + 1);
                            let text = strip_html(&html);
                            let trimmed = text.trim();
                            let preview: String = trimmed.chars().take(500).collect();
                            println!("{}", preview);
                            if trimmed.len() > 500 {
                                println!("...");
                            }
                            println!();
                        }
                    }
                }
            }
        }
    }
}
