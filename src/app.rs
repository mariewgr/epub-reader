pub struct App {
    pub lines: Vec<String>,
    pub scroll: usize,
    pub should_quit: bool,
}

impl App {
    pub fn new(content: String, width: usize) -> Self {
        let lines = Self::wrap_lines(&content, width);
        App {
            lines,
            scroll: 0,
            should_quit: false,
        }
    }

    pub fn scroll_down(&mut self) {
        self.scroll = self.scroll.saturating_add(1);
    }

    pub fn scroll_up(&mut self) {
        self.scroll = self.scroll.saturating_sub(1);
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
}
