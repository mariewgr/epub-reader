pub struct App {
    pub lines: Vec<String>,
    pub scroll: usize,
    pub should_quit: bool,
}

impl App {
    pub fn new(content: String) -> Self {
        let lines = content.lines().map(|l| l.to_string()).collect();
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
}
