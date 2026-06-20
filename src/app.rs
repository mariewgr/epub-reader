pub struct App {
    pub lines: Vec<String>,
    pub scroll: usize,
    pub should_quit: bool,
    pub chapters: Vec<String>,
    pub chapter_contents: Vec<String>,
    pub selected_chapter: usize,
    pub focus: Focus,
    pub search_query: String,
    pub search_match: Option<usize>,
}

#[derive(PartialEq)]
pub enum Focus {
    Chapters,
    Reader,
    Search,
}

impl App {
    pub fn new(chapter_contents: Vec<String>, width: usize, chapters: Vec<String>) -> Self {
        let lines = Self::wrap_lines(&chapter_contents[0], width);
        App {
            lines,
            scroll: 0,
            should_quit: false,
            chapters,
            chapter_contents,
            selected_chapter: 0,
            focus: Focus::Reader,
            search_query: String::new(),
            search_match: None,
        }
    }

    pub fn load_chapter(&mut self, width: usize) {
        self.lines = Self::wrap_lines(&self.chapter_contents[self.selected_chapter], width);
        self.scroll = 0;
    }

    pub fn scroll_down(&mut self) {
        self.scroll = self.scroll.saturating_add(1);
    }

    pub fn scroll_up(&mut self) {
        self.scroll = self.scroll.saturating_sub(1);
    }

    pub fn search(&mut self) {
        if self.search_query.is_empty() {
            self.search_match = None;
            return;
        }
        let query = self.search_query.to_lowercase();
        let result = self
            .lines
            .iter()
            .enumerate()
            .skip(self.scroll + 1)
            .find(|(_, line)| line.to_lowercase().contains(&query))
            .map(|(i, _)| i);

        if let Some(line_idx) = result {
            self.scroll = line_idx;
            self.search_match = Some(line_idx);
        }
    }

    pub fn wrap_lines(content: &str, width: usize) -> Vec<String> {
        let mut result = Vec::new();

        for line in content.lines() {
            if line.is_empty() {
                result.push(String::new());
                continue;
            }

            let mut current = String::new();
            for word in line.split_whitespace() {
                if current.is_empty() {
                    current.push_str(word);
                } else if current.len() + 1 + word.len() <= width {
                    current.push(' ');
                    current.push_str(word);
                } else {
                    result.push(current.clone());
                    current = word.to_string();
                }
            }
            if !current.is_empty() {
                result.push(current);
            }
        }

        result
    }
    pub fn next_chapter(&mut self) {
        if self.selected_chapter + 1 < self.chapters.len() {
            self.selected_chapter += 1;
        }
    }

    pub fn prev_chapter(&mut self) {
        self.selected_chapter = self.selected_chapter.saturating_sub(1);
    }

    pub fn toggle_focus(&mut self) {
        self.focus = match self.focus {
            Focus::Chapters => Focus::Reader,
            Focus::Reader => Focus::Chapters,
            Focus::Search => Focus::Search,
        };
    }
}
