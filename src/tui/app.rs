use crate::agents::Role;

pub struct App {
    pub messages: Vec<(Role, String)>,
    pub input: String,
    pub input_cursor: usize,
    pub scroll_offset: u16,
    pub loading: bool,
    pub should_quit: bool,
}

impl Default for App {
    fn default() -> Self {
        Self {
            messages: Vec::new(),
            input: String::new(),
            input_cursor: 0,
            scroll_offset: 0,
            loading: false,
            should_quit: false,
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_user(&mut self, text: String) {
        self.messages.push((Role::User, text));
    }

    pub fn push_model(&mut self, text: String) {
        self.messages.push((Role::Model, text));
    }

    pub fn insert_char(&mut self, c: char) {
        self.input.insert(self.input_cursor, c);
        self.input_cursor += c.len_utf8();
    }

    pub fn delete_char(&mut self) {
        if self.input_cursor == 0 {
            return;
        }
        let prev = self.prev_char_boundary();
        self.input.drain(prev..self.input_cursor);
        self.input_cursor = prev;
    }

    pub fn move_cursor_left(&mut self) {
        if self.input_cursor == 0 {
            return;
        }
        self.input_cursor = self.prev_char_boundary();
    }

    pub fn move_cursor_right(&mut self) {
        if let Some(c) = self.input[self.input_cursor..].chars().next() {
            self.input_cursor += c.len_utf8();
        }
    }

    pub fn send_message(&mut self) -> String {
        self.input_cursor = 0;
        std::mem::take(&mut self.input)
    }

    pub fn scroll_up(&mut self, amount: u16) {
        self.scroll_offset = self.scroll_offset.saturating_sub(amount);
    }

    pub fn scroll_down(&mut self, amount: u16) {
        // saturating_add; the render loop clamps to the real maximum.
        self.scroll_offset = self.scroll_offset.saturating_add(amount);
    }

    pub fn scroll_to_bottom(&mut self) {
        // u16::MAX is a sentinel; the render loop clamps it to the real maximum.
        self.scroll_offset = u16::MAX;
    }

    fn prev_char_boundary(&self) -> usize {
        let mut prev = self.input_cursor - 1;
        while !self.input.is_char_boundary(prev) {
            prev -= 1;
        }
        prev
    }
}
