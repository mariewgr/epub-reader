use epub::doc::EpubDoc;
use std::env;

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
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: epub-reader <file.epub>");
        std::process::exit(1);
    }

    let mut doc = EpubDoc::new(&args[1]).expect("Failed to open epub file");
    doc.mdata("title")
        .map(|item| item.value.clone())
        .unwrap_or("Unknown".to_string());

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
    println!();

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
