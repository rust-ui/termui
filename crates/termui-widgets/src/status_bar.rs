use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

/// A one-line status display with left and right sections.
#[must_use]
pub struct StatusBar<'a> {
    left: &'a str,
    right: &'a str,
}

impl<'a> StatusBar<'a> {
    pub fn new(left: &'a str) -> Self {
        Self { left, right: "" }
    }

    pub fn right(mut self, right: &'a str) -> Self {
        self.right = right;
        self
    }

    pub fn line(&self) -> Line<'a> {
        Line::from(vec![
            Span::styled(self.left, Style::default().fg(Color::Cyan)),
            Span::styled("  │  ", Style::default().fg(Color::DarkGray)),
            Span::raw(self.right),
        ])
    }
}
